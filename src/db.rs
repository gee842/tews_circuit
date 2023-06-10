use sqlite::{self, Connection as DbConn};

pub struct Connection {
    conn: DbConn,
}

impl Connection {
    pub fn new() -> Self {
        let conn = sqlite::open("challenges.db").unwrap();
        let query = "CREATE TABLE Matches (
            Challenger TEXT,
            Challenged TEXT,
            Match_time TEXT,
            Winner TEXT
        );";

        if let Err(_) = conn.execute(query) {
            println!("Table already exists.");
        } else {
            println!("Table doesn't exist. Creating.");
        }

        Self { conn }
    }

    pub fn new_challenge(&mut self, challenger: &str, challenged: &str, match_time: &str) {
        let query = format!(
            "INSERT INTO Matches VALUES ('{}', '{}', '{}', '{}');",
            challenger, challenged, match_time, "me"
        );

        todo!("This is unsafe as the statement isn't prepared. This method works though. Waiting on https://github.com/stainless-steel/sqlite/issues/69");
        self.conn.execute(query).unwrap();
    }
}

fn main() {
    let mut conn = Connection::new();
    conn.new_challenge("me", "you", "now");
}
