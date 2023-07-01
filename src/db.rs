use std::fs;

use chrono::NaiveDate;
use sqlx::{
    query,
    sqlite::{SqlitePool, SqlitePoolOptions},
    Error as SqlxError 
};

#[derive(Clone)]
pub struct Database {
    conn: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, SqlxError> {
        let conn = SqlitePoolOptions::new()
            .connect("sqlite://database.db")
            .await?;

        // Path taken from https://docs.rs/sqlx/0.6.3/sqlx/sqlite/struct.SqliteConnectOptions.html
        let sql = fs::read_to_string("db.sql")?;
        query(sql.as_str()).execute(&conn).await?;
        Ok(Self { conn })
    }

    pub async fn new_challenge(&mut self, challenger: &str, challenged: &str, date: &str) -> Result<(), SqlxError> {
        match NaiveDate::parse_from_str(date, "%v %H:%M") {
            Ok(_) => {}
            Err(e) => {} // This needs to return a custom error
        };

        query("INSERT INTO History VALUES (?, ?, ?, ?, ?);")
            .bind(challenger)
            .bind(challenged)
            .bind(date)
            .bind(0)
            .bind("N/A")
            .execute(&self.conn).await?;

        println!("Added entry to challenges table.");
        Ok(())
    }
}
