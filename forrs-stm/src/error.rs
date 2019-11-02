#[derive(Debug, Clone)]
pub enum Error {
    DbUnknown {
        description: String,
    },
    UniqueViolation,
    #[cfg(feature = "with-rocket")]
    RocketConfig {
        message: String,
    },
}

impl From<tokio_postgres::Error> for Error {
    fn from(e: tokio_postgres::Error) -> Error {
        if let Some(sql_state) = e.code() {
            use tokio_postgres::error::SqlState;
            if sql_state == &SqlState::UNIQUE_VIOLATION {
                return Error::UniqueViolation;
            }
        }
        Error::DbUnknown {
            description: e.to_string(),
        }
    }
}
