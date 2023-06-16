use sqlite::{self, Connection as DbConn, State};

pub struct Connection {
    conn: DbConn,
}

impl Connection {
    pub fn new() -> Self {
        let conn = sqlite::open("challenges.db").unwrap();
        let query = "
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS Matches (
                Challenger TEXT,
                Challenged TEXT,
                Match_time TEXT,
                Winner TEXT
            );

            CREATE TABLE IF NOT EXISTS History (
                UID TEXT PRIMARY KEY,
                Name TEXT,
                Clan TEXT,
                Win INTEGER,
                Loss INTEGER,
                Disqualified INTEGER,
                Points INTEGER
            );

            CREATE TABLE IF NOT EXISTS Rank (
                UID TEXT,
                Rank TEXT,
                FOREIGN KEY (UID) REFERENCES History(UID)
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

// Implement funtions related to writing to the challenges table.
impl Connection {
    pub fn new_challenge(&mut self, challenger: &str, challenged: &str, match_time: &str) {
        let query = "INSERT INTO Matches VALUES (:challenger, :challenged, :match_time, :winner);";
        let mut stmt = self.conn.prepare(query).unwrap();

        stmt.bind((":challenger", challenger)).unwrap();
        stmt.bind((":challenged", challenged)).unwrap();
        stmt.bind((":match_time", match_time)).unwrap();
        stmt.bind((":winner", "N/A")).unwrap();

        while let Ok(State::Row) = stmt.next() {}
        println!("Added entry to challenges table.");
    }
}

// Implement funtions related to writing to the history table.
impl Connection {
    pub fn new_history(
        &mut self,
        name: &str,
        uid: &str,
        clan: &str,
        win: &str,
        loss: &str,
        disqualified: &str,
        points: &str,
    ) {
        let query =
            "INSERT INTO History VALUES (:name, :uid, :clan, :win, :loss, :disqualified, :points);";
        let mut stmt = self.conn.prepare(query).unwrap();

        stmt.bind((":name", name)).unwrap();
        stmt.bind((":uid", uid)).unwrap();
        stmt.bind((":clan", clan)).unwrap();
        stmt.bind((":win", win)).unwrap();
        stmt.bind((":loss", loss)).unwrap();
        stmt.bind((":disqualified", disqualified)).unwrap();
        stmt.bind((":points", points)).unwrap();

        while let Ok(State::Row) = stmt.next() {}
        println!("Added entry to history table.");
    }
}

// Implement funtions related to writing to the rank table.
impl Connection {
    pub fn new_rank(&mut self, uid: &str, rank: &str) {
        let query = "INSERT INTO Rank VALUES (:uid, :rank);";
        let mut stmt = self.conn.prepare(query).unwrap();
        stmt.bind((":uid", uid)).unwrap();
        stmt.bind((":rank", rank)).unwrap();

        while let Ok(State::Row) = stmt.next() {}
        println!("Added entry to rank table.");
    }
}
