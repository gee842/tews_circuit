use rusqlite::{Connection, OpenFlags};
use std::{
    fs,
    io::{Error as IoError, ErrorKind},
};

struct Player {
    uid: usize,
    win: u16,
    loss: u16,
    dq: u16,
    rank: String,
    points: u16,
    win_streak: u8,
    lose_streak: u8,
}

pub struct Database {
    connector: Connection,
}

impl Database {
    pub fn new() -> Result<Self, IoError> {
        let flags = OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_FULL_MUTEX;
        // Path taken from https://docs.rs/sqlx/0.6.3/sqlx/sqlite/struct.SqliteConnectOptions.html
        let connector = match Connection::open_with_flags("sqlite://database.db", flags) {
            Ok(connector) => connector,
            Err(e) => return Err(IoError::new(ErrorKind::NotFound, e.to_string())),
        };

        let sql = fs::read_to_string("db.sql")?;
        match connector.execute(&sql, ()) {
            Ok(_) => {},
            Err(_) => return Err(IoError::new(ErrorKind::InvalidInput, "There appears to be something wrong with the SQL statement in db.sql.")),
        };

        Ok(Self { connector })
    }
}
