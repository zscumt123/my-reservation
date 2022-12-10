use async_trait::async_trait;
use sqlx::PgPool;

mod error;
mod manager;

pub use error::ReservationError;

pub type ReservationId = String;

#[derive(Debug)]
pub struct ReservationManager {
    pool: PgPool,
}

#[async_trait]
pub trait Rsvp {
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError>;
    async fn change_status(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;
    async fn update_note(
        &self,
        id: ReservationId,
        note: String,
    ) -> Result<abi::Reservation, ReservationError>;
    async fn delete(&self, id: ReservationId) -> Result<(), ReservationError>;
    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;

    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, ReservationError>;
}
