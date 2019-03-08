use chrono::Utc;
use diesel::prelude::*;
use diesel::{self, PgConnection};

use crate::models::{NewReleaseUrl, ReleaseId, ReleaseUrl};

pub struct ReleaseUrlRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> ReleaseUrlRepository<'a> {
    pub fn new(connection: &PgConnection) -> ReleaseUrlRepository {
        ReleaseUrlRepository { connection }
    }

    pub fn create(&self, release_id: ReleaseId, url: &str) -> ReleaseUrl {
        use crate::schema::release_urls;

        let now = Utc::now().naive_utc();

        let new_release_url = NewReleaseUrl {
            release_id,
            url,
            name: "[untitled]",
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(release_urls::table)
            .values(&new_release_url)
            .get_result(self.connection)
            .expect("Error creating new release url")
    }
}
