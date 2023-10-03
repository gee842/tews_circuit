use std::{
    fs::{self, OpenOptions},
    io::ErrorKind,
};

use crate::{errors::Error, player::Streak};
use crate::player::Player;

use async_recursion::async_recursion;
use tracing::{info, warn};

use chrono::NaiveDateTime;
use sqlx::{
    query,
    sqlite::{SqlitePool, SqlitePoolOptions},
    Error as SqlxError, Row,
};

#[derive(Clone)]
pub struct Database {
    pub conn: SqlitePool,
}

impl Database {
    /// Attempts to connect to the database. If databse doesn't exist then it'll be
    /// created. Otherwise, connects with no issues.
    pub async fn new() -> Result<Self, SqlxError> {
        // Path taken from https://docs.rs/sqlx/0.6.3/sqlx/sqlite/struct.SqliteConnectOptions.html
        // Creates the database file if it doesn't already exist.
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("database.db")
        {
            Err(e) if e.kind() != ErrorKind::AlreadyExists => {
                return Err(SqlxError::Io(e));
            }
            _ => {}
        }

        let db_path = "sqlite://database.db";
        let conn = SqlitePoolOptions::new().connect(db_path).await?;
        let db_creation_query = fs::read_to_string("./db.sql")?;
        query(db_creation_query.as_str()).execute(&conn).await?;

        Ok(Self { conn })
    }

    pub async fn streak_info(&self, player: &Player) -> Result<Streak, SqlxError> {
        let id = player.id();

        let sql = "SELECT WinStreak, LoseStreak FROM Players WHERE UID = ?";
        let results = query(sql).bind(id).fetch_one(&self.conn).await?;

        let win_streak = results.get(0);
        let lose_streak = results.get(1);
        let amount = std::cmp::max(win_streak, lose_streak);

        if win_streak == 0 && lose_streak == 0 {
            Ok(Streak::Neither)
        } else {
            Ok(Streak::Amount(amount))
        }
}
}

// Implementations of challenge functions
impl Database {
    /// Adds a new entry to the History table.
    /// - success: A guard to prevent the function from recusing infinitely.
    #[async_recursion]
    pub async fn add_new_challenge(
        &mut self,
        challenger: &str,
        challenged: &str,
        original_date: &str,
        success: Option<bool>,
    ) -> Result<bool, SqlxError> {
        info!("Date: {:?}", original_date);
        let date = match NaiveDateTime::parse_from_str(original_date, "%e %b %Y %H:%M") {
            Ok(date) => {
                // SQLITE doesn't have a DATE type. But it does support
                // dates as TEXT in ISO 8601 format.
                info!("Date format accepted. Converting to ISO 8601");
                let formatted_date = date.format("%Y-%m-%d %H:%M").to_string();
                formatted_date
            }
            Err(e) => {
                let err_msg = e.to_string();
                warn!("{}", err_msg);
                return Err(SqlxError::Protocol(err_msg));
            }
        };

        // recusion guard
        if let Some(succeeded) = success {
            return Ok(succeeded);
        }

        if let Err(e) = query("INSERT INTO History VALUES (?, ?, ?, ?, ?);")
            .bind(challenger)
            .bind(challenged)
            .bind(date.clone())
            .bind(0)
            .bind("N/A")
            .execute(&self.conn)
            .await
        {
            match Error::kind(e) {
                Error::ForeignKeyConstraintNotMet => {
                    info!("Unregistered player(s) detected, adding them to the database.");
                    self.find_missing_player(challenger, challenged).await?;

                    info!("New player(s) registered. Re-running function.");
                    self.add_new_challenge(challenger, challenged, original_date, None)
                        .await?;
                }
                Error::Unknown(msg) => {
                    return Err(SqlxError::Protocol(msg));
                }
                Error::Locked => {
                    warn!("Database is busy with another operation, please try again.");
                }
                Error::None => {}
            }
        };

        info!("New challenge added.");
        Ok(true)
    }

    /// Checks the `Players` table to see if the `challenged` or the `challenger`
    /// already exists in the database. If either one or both do not exist
    /// they will be added to the database.
    async fn find_missing_player(
        &mut self,
        challenger: &str,
        challenged: &str,
    ) -> Result<(), SqlxError> {
        if self.add_new_player(challenger).await.is_ok() {
            info!("The challenger is missing. Added successfully.");
        }

        if self.add_new_player(challenged).await.is_ok() {
            info!("The challenged user missing. Added successfully.");
        }

        Ok(())
    }

    /// When a new player is found meaning they are not registered in the database
    /// this function will be called in order to add them.
    ///
    /// # Parameters
    /// - `user_id`: The user_id of the unregistered user.
    async fn add_new_player(&mut self, user_id: &str) -> Result<(), SqlxError> {
        let sql = query("INSERT INTO Players VALUES (?, ?, ?, ?, ?, ?, ?, ?);");
        sql.bind(user_id)
            .bind(0)
            .bind(0)
            .bind(0)
            .bind("Unrated")
            .bind(900)
            .bind(0)
            .bind(0)
            .execute(&self.conn)
            .await?;

        Ok(())
    }
}

// Player related methods
impl Database {
    pub async fn update_points(&self, points: u16, user: String) -> Result<(), SqlxError> {
        let sql = "UPDATE Players SET Points = ? WHERE UID = ?";
        let _ = query(sql)
            .bind(points)
            .bind(user.clone())
            .execute(&self.conn)
            .await?;

        Ok(())
    }

    pub async fn match_finished(&self, p1: String, p2: String) -> Result<(), SqlxError> {
        let sql = "UPDATE History SET Finished = 1 WHERE Challenger = ? OR Challenged = ?";
        let _ = query(sql)
            .bind(p1.clone())
            .bind(p2.clone())
            .execute(&self.conn)
            .await?;

        Ok(())
    }

    pub async fn mark_loss(&self, player: &Player) -> Result<(), SqlxError> {
        let sql = "
            UPDATE Players 
            SET 
                Lose = Lose + 1 
                LoseStreak = LoseStreak + 1
                WinStreak = 0
            WHERE 
                UID = ?
            ";
        let _ = query(sql).bind(player.id()).execute(&self.conn).await?;

        Ok(())
    }

    pub async fn update_rank(&self, player: &Player) -> Result<(), SqlxError> {
        let sql = "UPDATE Players SET Rank = ? WHERE UID = ?";
        let _ = query(sql)
            .bind(player.rank.to_string())
            .bind(player.id())
            .execute(&self.conn)
            .await?;

        Ok(())
    }

    pub async fn mark_win(&self, player: &Player) -> Result<(), SqlxError> {
        let sql = "
            UPDATE Players 
            SET 
                Win = Win + 1 
                WinStreak = WinStreak + 1
                LoseStreak = 0
            WHERE 
                UID = ?
            ";
        let _ = query(sql).bind(player.id()).execute(&self.conn).await?;

        Ok(())
    }

    pub async fn points_data(&self, user_id: u64) -> Result<u16, SqlxError> {
        // SQL query for Players table where ID is equal to user_id
        let result = query("SELECT * FROM Players WHERE UID = ?;")
            .bind(user_id.to_string())
            .fetch_one(&self.conn)
            .await?;

        let points: u16 = result.get(5);

        Ok(points)
    }

    /// Processes all disqualifications. Is ran at the start of every function.
    pub async fn disqualify(&self) -> Result<(), SqlxError> {
        let mut sql =
            "SELECT Challenger, Challenged FROM History WHERE Date < Date('now') AND Finished = 0;";

        let unfinished_matches = query(sql).fetch_all(&self.conn).await?;

        let mut users: Vec<(String, String)> = vec![];
        users.extend(unfinished_matches.iter().map(|e| (e.get(0), e.get(1))));

        if users.is_empty() {
            info!("No matches are past due.");
            return Ok(());
        }

        info!("Processing matches that are past due.");

        for user in users {
            let challenger = user.0.clone();
            let challenged = user.1.clone();

            sql = "UPDATE Players SET Points = Points - 10 WHERE UID = ?";
            _ = query(sql)
                .bind(challenger.clone())
                .execute(&self.conn)
                .await?;

            _ = query(sql)
                .bind(challenged.clone())
                .execute(&self.conn)
                .await?;

            sql = "UPDATE History SET Finished = 1 WHERE Challenger = ? AND Challenged = ?";
            _ = query(sql)
                .bind(challenger.clone())
                .bind(challenged.clone())
                .execute(&self.conn)
                .await?;

            sql = "UPDATE Players SET Disqualifications = Disqualifications + 1 WHERE UID = ?";
            _ = query(sql)
                .bind(challenger.clone())
                .execute(&self.conn)
                .await?;

            _ = query(sql)
                .bind(challenged.clone())
                .execute(&self.conn)
                .await?;
        }

        Ok(())
    }

    pub async fn closest_matches(&self, caller_id: &str) -> Result<String, SqlxError> {
        let sql = r#"
SELECT * FROM 
History WHERE (Challenger = ? OR Challenged = ?) AND Finished = 0
ORDER BY ABS(strftime("%s", "now") - strftime("%s", "Date"))"#;

        let row = if let Ok(row) = query(sql)
            .bind(caller_id)
            .bind(caller_id)
            .fetch_one(&self.conn)
            .await
        {
            row
        } else {
            warn!("No matches found for this user.");
            return Err(SqlxError::RowNotFound);
        };

        let mut other_id: String = row.get(1);

        // Gets the id of the other user.
        if caller_id == other_id {
            other_id = row.get(0);
        }

        Ok(other_id)
    }

    /// `user` - The specified user's pending matches
    pub async fn player_matches(&self, user: &str) -> Result<Vec<(String, String)>, SqlxError> {
        let sql = "SELECT * FROM History WHERE 
            (Challenger = ? OR Challenged = ?)
            AND Finished = 0
            AND Date > Date('now')
            ";

        let rows = query(sql)
            .bind(user)
            .bind(user)
            .fetch_all(&self.conn)
            .await?;

        let mut challenges = vec![];
        let msg = format!("=========== {} ===========", user);
        info!("{}", msg);

        for row in rows {
            let challenged: String = row.get(1);
            let time: String = row.get(2);

            info!("- vs {} on {}", &challenged, &time);

            challenges.push((challenged, time));
        }

        let msg: String = "=".repeat(msg.len());
        info!("{}", msg);
        Ok(challenges)
    }
}
