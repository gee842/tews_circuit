use std::{
    fs::{self, OpenOptions},
    io::ErrorKind,
    pin::Pin,
};

use async_recursion::async_recursion;
use futures::Future;
use tokio_util::time::DelayQueue;
use tracing::info;

use chrono::{NaiveDate, NaiveDateTime, Utc};
use sqlx::{
    query,
    sqlite::{SqlitePool, SqlitePoolOptions},
    Connection, Error as SqlxError, Row,
};

#[derive(Clone)]
pub struct Database {
    conn: SqlitePool,
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

        return Ok(Self { conn });
    }
}

// Implementations of challenge functions
impl Database {
    /// Adds a new entry to the History table.
    /// # Parameters
    /// - challenger: The UID of the challenger.
    /// - challenged: The UID of the challenged.
    /// - date: The date and time of the challenge.
    /// - success: A guard to prevent the function from recusing infinitely.
    #[async_recursion]
    pub async fn new_challenge(
        &mut self,
        challenger: &str,
        challenged: &str,
        date: &str,
        success: Option<bool>,
    ) -> Result<bool, SqlxError> {
        match NaiveDate::parse_from_str(date, "%e %b %Y %H:%M") {
            Ok(_) => info!("Date format accepted."),
            Err(_) => {
                return Err(SqlxError::Protocol(
                    "Invalid date format. Please run the command again.".to_string(),
                ))
            }
        };

        if let Some(succeeded) = success {
            return Ok(succeeded);
        }

        match query("INSERT INTO History VALUES (?, ?, ?, ?, ?);")
            .bind(challenger)
            .bind(challenged)
            .bind(date)
            .bind(0)
            .bind("N/A")
            .execute(&self.conn)
            .await
        {
            Err(e) => {
                // Checks for an error related to the insert query
                let error = e.as_database_error();
                if let None = error {
                    return Ok(false);
                }

                let error = error.unwrap();
                let code = error.code().unwrap();

                // 787 is the error code for foreign key constraint not met.
                // Meaning either the challenger or the challenged has not
                // been added to the Players table.
                if code != "787" {
                    let msg = format!("Error code: {}\nMessage: {}", code, error.message());
                    return Err(SqlxError::Protocol(msg));
                } else {
                    info!("Unregistered player(s) detected, adding them to the database.");
                    self.find_missing_player(challenger, challenged).await?;
                    info!("New player(s) registered. Re-running function.");
                    self.new_challenge(challenger, challenged, date, None)
                        .await?;
                }
            }
            _ => {}
        };

        info!("New challenge added to the challenges table.");
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
        let rows = query("SELECT * FROM Players WHERE UID = ? OR ?")
            .bind(challenger)
            .bind(challenged)
            .fetch_all(&self.conn)
            .await?;

        let uids: Vec<&str> = rows.iter().map(|row| row.get(1)).collect();
        let challenged_missing = !uids.contains(&challenged);
        let challenger_missing = !uids.contains(&challenger);

        if challenger_missing {
            self.add_new_player(challenger).await?;
            info!("The challenger is missing. Added successfully.");
        }

        if challenged_missing {
            self.add_new_player(challenged).await?;
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

// Data functions
impl Database {
    pub async fn time_for_match(&self) -> Vec<(f64, f64, String)> {
        let mut matches = vec![];
        // TODO: Identify what you should actually return when it is the Err match arm.
        let rows = match query("SELECT * FROM History WHERE Finished = 0")
            .fetch_all(&self.conn)
            .await
        {
            Ok(rows) => rows,
            Err(e) => return matches,
        };

        for row in rows {
            let date = match row.get(2) {
                Some(date) => date,
                None => return matches,
            };

            let match_datetime = match NaiveDateTime::parse_from_str(date, "%e %b %Y %H:%M") {
                Ok(match_datetime) => match_datetime,
                Err(_) => return matches,
            };

            // let date = DateTime::from_utc(match_datetime, 8).date_naive();
            let current_datetime = Utc::now().naive_utc();
            let remaining_time = match_datetime - current_datetime;
            let days = remaining_time.num_days();
            let hours = remaining_time.num_hours();
            let minute = remaining_time.num_minutes();

            if days <= 1 || hours <= 1 || minute <= 5 {
                let challenger: f64 = row.get(0);
                let challenged: f64 = row.get(1);
                let date = format!("{}d {}h {}min", days, hours, minute);
                matches.push((challenger, challenged, date));
            }
        }

        matches
    }

    pub fn get_player_uids<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, SqlxError>> + 'a>> {
        Box::pin(async move {
            let rows = query("SELECT UID FROM Players")
                .fetch_all(&self.conn)
                .await?;

            let uid: Vec<String> = rows.iter().map(|row| row.get(0)).collect();

            Ok(uid)
        })
    }
}

// Player history
impl Database {
    /// `user` - The specified user's pending matches
    pub async fn player_matches(&self, user: &str) -> Result<Vec<(String, String)>, SqlxError> {
        let rows =
            query("SELECT * FROM History WHERE Challenger = ? OR Challenged = ? AND Finished = 0")
                .bind(user)
                .bind(user)
                .fetch_all(&self.conn)
                .await?;

        let mut challenges = vec![];
        info!("=========== {} ===========", user);
        for row in rows {
            let challenged: String = row.get(1);
            let time: String = row.get(2);

            info!("- vs {} on {}", &challenged, &time);

            challenges.push((challenged, time));
        }

        info!("\n");
        Ok(challenges)
    }
}
