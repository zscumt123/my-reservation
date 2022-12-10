use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReservationError {
    #[error("db error {0}")]
    DbError(#[from] sqlx::Error),
    #[error("invalid start or end time")]
    InvalidTime,
    #[error("unknown error")]
    Unknown,
}
