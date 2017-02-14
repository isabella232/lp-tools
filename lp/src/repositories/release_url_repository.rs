use chrono::UTC;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel;

use models::{ReleaseId, ReleaseUrl, NewReleaseUrl};

pub struct ReleaseUrlRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> ReleaseUrlRepository<'a> {
    pub fn new(connection: &PgConnection) -> ReleaseUrlRepository {
        ReleaseUrlRepository { connection: connection }
    }

    pub fn create(&self, release_id: ReleaseId, url: &str) -> ReleaseUrl {
        use schema::release_urls;

        let now = UTC::now().naive_utc();

        let new_release_url = NewReleaseUrl {
            release_id: release_id,
            url: url,
            name: "[untitled]",
            created_at: now,
            updated_at: now,
        };

        diesel::insert(&new_release_url)
            .into(release_urls::table)
            .get_result(self.connection)
            .expect("Error creating new release url")
    }
}
