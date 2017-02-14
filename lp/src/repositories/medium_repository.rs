use chrono::UTC;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel;

use models::{Medium, NewMedium, ReleaseId};

pub struct MediumRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> MediumRepository<'a> {
    pub fn new(connection: &PgConnection) -> MediumRepository {
        MediumRepository { connection: connection }
    }

    pub fn create(&self,
                  release_id: ReleaseId,
                  kind: i32,
                  position: i16,
                  name: Option<&str>) -> Medium {
        use schema::media;

        let now = UTC::now().naive_utc();

        let new_medium = NewMedium {
            release_id: release_id,
            kind: kind,
            position: position,
            name: name,
            created_at: now,
            updated_at: now,
        };

        diesel::insert(&new_medium)
            .into(media::table)
            .get_result(self.connection)
            .expect("Error creating new medium")
    }
}
