use abi::Error;
use async_trait::async_trait;
use sqlx::PgPool;

mod manager;

pub type ReservationId = String;

#[derive(Debug)]
pub struct ReservationManager {
    pool: PgPool,
}

#[async_trait]
pub trait Rsvp {
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, Error>;
    async fn change_status(&self, id: ReservationId) -> Result<abi::Reservation, Error>;
    async fn update_note(&self, id: ReservationId, note: String)
        -> Result<abi::Reservation, Error>;
    async fn delete(&self, id: ReservationId) -> Result<(), Error>;
    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, Error>;

    async fn query(&self, query: abi::ReservationQuery) -> Result<Vec<abi::Reservation>, Error>;
}
