use std::{
    fs::{self, OpenOptions},
    io::ErrorKind,
    iter::repeat,
};

use super::errors::Error;

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
    pub async fn add_new_challenge(
        &mut self,
        challenger: &str,
        challenged: &str,
        date: &str,
        success: Option<bool>,
    ) -> Result<bool, SqlxError> {
        let date = match NaiveDateTime::parse_from_str(date, "%e %b %Y %H:%M") {
            Ok(date) => {
                // SQLITE doesn't have a DATE data type. But it does support
                // dates as TEXT in ISO 8601 format.
                info!("Date format accepted. Converting to ISO 8601");
                let formatted_date = date.format("%Y-%m-%d %H:%M").to_string();
                formatted_date
            }
            Err(_) => {
                return Err(SqlxError::Protocol(
                    "Invalid date format. Please run the command again.".to_string(),
                ))
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
                    self.add_new_challenge(challenger, challenged, &date, None)
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
        if let Ok(_) = self.add_new_player(challenger).await {
            info!("The challenger is missing. Added successfully.");
        }

        if let Ok(_) = self.add_new_player(challenged).await {
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

// Player history
impl Database {
    pub async fn closest_matches(&self, caller_id: &str) -> Result<String, SqlxError> {
        // TODO: Add a check where if the date of the challenge is past
        // current date, penalise the challenged user.

        let sql = r#"
SELECT * FROM 
History WHERE Challenger = ? OR Challenged = ? AND Finished = 0
ORDER BY ABS(strftime("%s", "now") - strftime("%s", "Date"))"#;

        let row = query(sql)
            .bind(caller_id)
            .bind(caller_id)
            .fetch_one(&self.conn)
            .await?;

        let mut other_id: String = row.get(1);

        // Gets the id of the other user.
        if caller_id == other_id {
            other_id = row.get(0);
        }

        Ok(other_id)
    }

    /// `user` - The specified user's pending matches
    pub async fn player_matches(&self, user: &str) -> Result<Vec<(String, String)>, SqlxError> {
        let rows =
            query("SELECT * FROM History WHERE Challenger = ? OR Challenged = ? AND Finished = 0")
                .bind(user)
                .bind(user)
                .fetch_all(&self.conn)
                .await?;

        let mut challenges = vec![];
        let msg = format!("=========== {} ===========", user);
        info!(msg);
        for row in rows {
            let challenged: String = row.get(1);
            let time: String = row.get(2);

            info!("- vs {} on {}", &challenged, &time);

            challenges.push((challenged, time));
        }

        let msg: String = repeat("=").take(msg.len()).collect();
        info!(msg);
        Ok(challenges)
    }
}
