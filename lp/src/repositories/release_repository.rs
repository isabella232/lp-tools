use chrono::{NaiveDate, UTC};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel;

use models::{AlbumId, Release, NewRelease};

pub struct ReleaseRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> ReleaseRepository<'a> {
    pub fn new(connection: &PgConnection) -> ReleaseRepository {
        ReleaseRepository { connection: connection }
    }

    pub fn create(&self,
                  album_id: AlbumId,
                  released_on: NaiveDate,
                  country: Option<&str>,
                  catalog_number: Option<&str>,
                  disambiguation: Option<&str>) -> Release {
        use schema::releases;

        let now = UTC::now().naive_utc();

        let new_release = NewRelease {
            album_id: album_id,
            released_on: released_on,
            country: country,
            catalog_number: catalog_number,
            disambiguation: disambiguation,
            artwork_data: None,
            created_at: now,
            updated_at: now,
        };

        diesel::insert(&new_release)
            .into(releases::table)
            .get_result(self.connection)
            .expect("Error creating new release")
    }
}
