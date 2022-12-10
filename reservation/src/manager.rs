use crate::{ReservationError, ReservationId, ReservationManager, Rsvp};
use abi::convert_to_utc_time;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{postgres::types::PgRange, Row};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(
        &self,
        mut rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, ReservationError> {
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(ReservationError::InvalidTime);
        }
        let start = convert_to_utc_time(rsvp.start.as_ref().unwrap().clone());
        let end = convert_to_utc_time(rsvp.end.as_ref().unwrap().clone());

        let range: PgRange<DateTime<Utc>> = (start..end).into();

        let id = sqlx::query(
            "INSERT INTO rsvp.reservations \
          (user_id, status, resource_id, timespan, note) \
          VALUES ($1, $2, $3, $4, $5) \
          RETURNING id",
        )
        .bind(rsvp.user_id.clone())
        .bind(rsvp.status)
        .bind(rsvp.resource_id.clone())
        .bind(range)
        .bind(rsvp.note.clone())
        .fetch_one(&self.pool)
        .await?
        .get(0);
        rsvp.id = id;
        Ok(rsvp)
    }
    async fn change_status(
        &self,
        _id: ReservationId,
    ) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }
    async fn update_note(
        &self,
        _id: ReservationId,
        _note: String,
    ) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }
    async fn delete(&self, _id: ReservationId) -> Result<(), ReservationError> {
        todo!()
    }
    async fn get(&self, _id: ReservationId) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn query(
        &self,
        _query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, ReservationError> {
        todo!()
    }
}
