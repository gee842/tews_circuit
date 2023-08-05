use sqlx::Error as SqlxError;
use tracing::warn;

pub enum Error {
    /// Signifies that no error has occured.
    None,
    /// An unknown error has occured.
    Unknown(String),
    /// Foreign key constraint not met.
    ForeignKeyConstraintNotMet,
    /// Database is tied up in some other operation.
    Locked,
}

impl Error {
    pub fn kind(error: SqlxError) -> Error {
        let error = error.as_database_error();
        if let None = error {
            return Error::None;
        }

        let error = error.unwrap();
        let code = error.code().unwrap();

        if code == "787" {
            warn!("Error::ForeignKeyConstraintNotMet");
            return Error::ForeignKeyConstraintNotMet;
        }

        if code == "5" {
            warn!("Database is tied up in some other operation.");
            return Error::Locked;
        }

        let msg = format!("Error code: '{}'\nMessage: {}", code, error.message());
        Error::Unknown(msg)
    }
}
