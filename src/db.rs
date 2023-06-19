use sqlite::{self, Connection as DbConn, Error as SqliteError, State};

pub struct Connection {
    conn: DbConn,
}

impl Connection {
    pub fn new() -> Self {
        let conn = sqlite::open("challenges.db").unwrap();
        let query = "
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS History (
                Challenger	TEXT,
                Challenged	TEXT,
                Date	TEXT,
                Finished	INTEGER,
                Winner	TEXT,
                FOREIGN KEY(Challenger) REFERENCES Players(UID)
            );

            CREATE TABLE IF NOT EXISTS Players (
                UID	TEXT,
                Win	INTEGER,
                Loss	INTEGER,
                Disqualifications	INTEGER,
                Rank	TEXT,
                Points	INTEGER,
                PRIMARY KEY(UID)
            );

            CREATE TABLE IF NOT EXISTS Ranks (
                Name	TEXT,
                PRIMARY KEY(Name)
            );
        ";

        if let Err(_) = conn.execute(query) {
            println!("Tables already exists.");
        } else {
            println!("Tables don't exist. Creating.");
        }

        Self { conn }
    }
}

impl Connection {
    /// Creates a new entry in the History table.
    pub fn new_challenge(
        &mut self,
        challenger: &str,
        challenged: &str,
        date: &str,
    ) -> Result<(), SqliteError> {
        // Checks if both players exist in the db.
        println!("challenger: {}", challenger);
        if !self.player_exists(challenger)? {
            self.add_player(challenger)?;
        }

        println!("challenged: {}", challenged);
        if !self.player_exists(challenged)? {
            self.add_player(challenged)?;
        }

        let query =
            "INSERT INTO History VALUES (:challenger, :challenged, :date, :finished, :winner);";
        let mut stmt = self.conn.prepare(query)?;

        stmt.bind((":challenger", challenger))?;
        stmt.bind((":challenged", challenged))?;
        stmt.bind((":date", date))?;
        stmt.bind((":finished", 0))?;
        stmt.bind((":winner", "N/A"))?;

        while let Ok(State::Row) = stmt.next() {}
        println!("Added entry to challenges table.");

        Ok(())
    }

    pub fn challenge_finished(&mut self, challenger: &str, challenged: &str) {
        todo!();
    }
}

impl Connection {
    fn add_player(&mut self, user_id: &str) -> Result<(), SqliteError> {
        let query =
            "INSERT INTO Players VALUES (:uid, :win, :loss, :disqualifications, :rank, :points)";
        let mut stmt = self.conn.prepare(query).unwrap();

        // Error here too. Might be related to the one in player_exists
        stmt.bind((":uid", user_id))?;
        stmt.bind((":win", 0))?;
        stmt.bind((":loss", 0))?;
        stmt.bind((":disqualifications", 0))?;
        stmt.bind((":rank", "Unrated"))?;
        stmt.bind((":points", 900))?;

        while let Ok(State::Row) = stmt.next() {}

        Ok(())
    }

    fn player_exists(&self, user_id: &str) -> Result<bool, SqliteError> {
        let query = "SELECT * FROM Players WHERE UID = :user_id";
        let mut stmt = self.conn.prepare(query)?;
        stmt.bind((":user_id", user_id))?;

        let mut found = false;

        while let Ok(State::Row) = stmt.next() {
            let id: String = stmt.read(0)?;
            if id == user_id {
                found = true;
                break;
            }
        }

        Ok(found)
    }
}
