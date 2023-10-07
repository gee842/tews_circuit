use sqlx::Error as SqlxError;
use tracing::warn;

pub enum TewsError {
    /// Signifies that no error has occured.
    None,
    /// An unknown error has occured.
    Unknown(String),
    /// Foreign key constraint not met.
    ForeignKeyConstraintNotMet,
    /// Database is tied up in some other operation.
    Locked,
}

impl TewsError {
    pub fn kind(error: SqlxError) -> TewsError {
        let error = error.as_database_error();
        if error.is_none() {
            return TewsError::None;
        }

        let error = error.unwrap();
        let code = error.code().unwrap();

        if code == "787" {
            return TewsError::ForeignKeyConstraintNotMet;
        }

        if code == "5" {
            warn!("Database is tied up in some other operation.");
            return TewsError::Locked;
        }

        let msg = format!("Error code: '{}'\nMessage: {}", code, error.message());
        TewsError::Unknown(format!("UNKNOWN ERROR: {}", msg))
    }
}
