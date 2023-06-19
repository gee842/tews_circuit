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
        if !self.player_exists(challenger)? {
            self.add_player(challenger)?;
        }

        if !self.player_exists(challenged)? {
            self.add_player(challenged)?;
        }

        let query =
            "INSERT INTO History VALUES (:Challenger, :Challenged, :Date, :Finished, :Winner);";
        let mut stmt = self.conn.prepare(query)?;

        stmt.bind((":Challenger", challenger))?;
        stmt.bind((":Challenged", challenged))?;
        stmt.bind((":Date", date))?;
        stmt.bind((":Finished", 0))?;
        stmt.bind((":Winner", "N/A"))?;

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
        let query = "INSERT INTO Players VALUES (:user_id, :win, :loss, :disqualifications, :rank, :points)";
        let mut stmt = self.conn.prepare(query).unwrap();

        stmt.bind((":UID", user_id))?;
        stmt.bind((":Win", 0))?;
        stmt.bind((":Loss", 0))?;
        stmt.bind((":Disqualifications", 0))?;
        stmt.bind((":Rank", "Unrated"))?;
        stmt.bind((":Points", 900))?;

        Ok(())
    }

    fn player_exists(&self, user_id: &str) -> Result<bool, SqliteError> {
        let query = "SELECT * FROM Players WHERE (UID = ':UID')";
        let mut stmt = self.conn.prepare(query)?;
        stmt.bind((":UID", user_id))?;

        let mut found = false;

        println!("Columns: {}", stmt.column_count());
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
