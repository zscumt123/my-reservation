use sqlx::postgres::PgDatabaseError;
use thiserror::Error;

mod conflict;

pub use conflict::{ReservationConflictInfo, ReservationWindow};

#[derive(Debug, Error)]
pub enum Error {
    #[error("db error {0}")]
    DbError(sqlx::Error),
    #[error("invalid start or end time")]
    InvalidTime,
    #[error("Conflict reservation")]
    ConflictReservation(ReservationConflictInfo),
    #[error("invalid user id, {0}")]
    InvalidUserId(String),
    #[error("invalid resource id, {0}")]
    InvalidResourceId(String),
    #[error("unknown error")]
    Unknown,
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(e) => {
                let err: &PgDatabaseError = e.downcast_ref();
                match (err.code(), err.schema(), err.table()) {
                    ("23P01", Some("rsvp"), Some("reservations")) => {
                        Error::ConflictReservation(err.detail().unwrap().parse().unwrap())
                    }
                    _ => Error::DbError(sqlx::Error::Database(e)),
                }
            }
            _ => Error::Unknown,
        }
    }
}
