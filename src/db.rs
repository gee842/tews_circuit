use chrono::{NaiveDate, NaiveDateTime, Utc};
use sqlx::{
    query,
    sqlite::{SqlitePool, SqlitePoolOptions},
    Error as SqlxError, Row,
};
use std::fs;

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

    pub async fn new_challenge(
        &mut self,
        challenger: &str,
        challenged: &str,
        date: &str,
    ) -> Result<(), SqlxError> {
        match NaiveDate::parse_from_str(date, "%e %b %Y %H:%M") {
            Ok(_) => println!("Date format accepted. Added challenges to table."),
            Err(_) => {
                return Err(SqlxError::Protocol(
                    "Invalid date format. Please run the command again.".to_string(),
                ))
            }
        };

        query("INSERT INTO History VALUES (?, ?, ?, ?, ?);")
            .bind(challenger)
            .bind(challenged)
            .bind(date)
            .bind(0)
            .bind("N/A")
            .execute(&self.conn)
            .await?;

        Ok(())
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
        println!("=========== {} ===========", user);
        for row in rows {
            let challenged: String = row.get(1);
            let time: String = row.get(2);

            println!("- vs {} on {}", &challenged, &time);

            challenges.push((challenged, time));
        }

        println!("\n");
        Ok(challenges)
    }

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
}
