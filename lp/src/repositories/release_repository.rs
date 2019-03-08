use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel::{self, PgConnection};

use crate::models::{AlbumId, NewRelease, Release};

pub struct ReleaseRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> ReleaseRepository<'a> {
    pub fn new(connection: &PgConnection) -> ReleaseRepository {
        ReleaseRepository { connection }
    }

    pub fn create(
        &self,
        album_id: AlbumId,
        released_on: NaiveDate,
        country: Option<&str>,
        catalog_number: Option<&str>,
        disambiguation: Option<&str>,
    ) -> Release {
        use crate::schema::releases;

        let now = Utc::now().naive_utc();

        let new_release = NewRelease {
            album_id,
            released_on,
            country,
            catalog_number,
            disambiguation,
            artwork_data: None,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(releases::table)
            .values(&new_release)
            .get_result(self.connection)
            .expect("Error creating new release")
    }
}
