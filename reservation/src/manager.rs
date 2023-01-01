use crate::{Error, ReservationId, ReservationManager, Rsvp};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{postgres::types::PgRange, PgPool, Row};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, Error> {
        rsvp.validate()?;

        let status = abi::ReservationStatus::from_i32(rsvp.status)
            .unwrap_or(abi::ReservationStatus::Pending);

        let range: PgRange<DateTime<Utc>> = rsvp.get_timespan().into();

        let id = sqlx::query(
            "INSERT INTO rsvp.reservations \
          (user_id, status, resource_id, timespan, note) \
          VALUES ($1, $2::rsvp.reservation_status, $3, $4, $5) \
          RETURNING id",
        )
        .bind(rsvp.user_id.clone())
        .bind(status.to_string())
        .bind(rsvp.resource_id.clone())
        .bind(range)
        .bind(rsvp.note.clone())
        .fetch_one(&self.pool)
        .await?
        .get(0);
        rsvp.id = id;
        Ok(rsvp)
    }

    async fn change_status(&self, _id: ReservationId) -> Result<abi::Reservation, Error> {
        todo!()
    }
    async fn update_note(
        &self,
        _id: ReservationId,
        _note: String,
    ) -> Result<abi::Reservation, Error> {
        todo!()
    }
    async fn delete(&self, _id: ReservationId) -> Result<(), Error> {
        todo!()
    }
    async fn get(&self, _id: ReservationId) -> Result<abi::Reservation, Error> {
        todo!()
    }

    async fn query(&self, _query: abi::ReservationQuery) -> Result<Vec<abi::Reservation>, Error> {
        todo!()
    }
}
impl ReservationManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[cfg(test)]
mod tests {
    use abi::ReservationConflictInfo;
    use chrono::FixedOffset;

    use super::*;

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_should_work_valid_window() {
        let manager = ReservationManager::new(migrated_pool.clone());
        let start: DateTime<FixedOffset> = "2022-12-25T15:00:00-0700".parse().unwrap();
        let end: DateTime<FixedOffset> = "2022-12-28T12:00:00-0700".parse().unwrap();
        let rsvp = abi::Reservation::new_pending("a", "b", start, end, "I arrived at 3pm");
        let rsvp = manager.reserve(rsvp).await.unwrap();
        assert!(rsvp.id != 0);
    }
    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_conflict_reservation_should_reject() {
        let manager = ReservationManager::new(migrated_pool.clone());
        let start: DateTime<FixedOffset> = "2022-12-25T15:00:00-0700".parse().unwrap();
        let end: DateTime<FixedOffset> = "2022-12-28T12:00:00-0700".parse().unwrap();
        let rsvp1 = abi::Reservation::new_pending("u1", "r1", start, end, "hello,world");
        let rsvp2 = abi::Reservation::new_pending(
            "u2",
            "r1",
            "2022-12-26T15:00:00-0700".parse().unwrap(),
            "2022-12-30T12:00:00-0700".parse().unwrap(),
            "hello,world",
        );
        let _rsvp1 = manager.reserve(rsvp1).await.unwrap();
        let err = manager.reserve(rsvp2).await.unwrap_err();

        if let abi::Error::ConflictReservation(ReservationConflictInfo::Parsed(info)) = err {
            assert_eq!(info.old.rid, "r1");
            assert_eq!(info.old.start.to_rfc3339(), "2022-12-25T22:00:00+00:00");
            assert_eq!(info.old.end.to_rfc3339(), "2022-12-28T19:00:00+00:00");
        } else {
            panic!("expect conflict reservation error");
        }
    }
}
