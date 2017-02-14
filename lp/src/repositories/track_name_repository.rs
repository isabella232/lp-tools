use chrono::UTC;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel;

use models::{TrackId, TrackName, NewTrackName};

pub struct TrackNameRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> TrackNameRepository<'a> {
    pub fn new(connection: &PgConnection) -> TrackNameRepository {
        TrackNameRepository { connection: connection }
    }

    pub fn find_by_track_id(&self, id: TrackId) -> Vec<TrackName> {
        use schema::track_names::dsl::{track_names, track_id};

        track_names
            .filter(track_id.eq(id))
            .load(self.connection)
            .expect("failed to load names")
    }

    pub fn create(&self,
                  track_id: TrackId,
                  name: &str,
                  locale: &str,
                  is_default: bool,
                  is_original: bool) -> TrackName {
        use schema::track_names;

        let now = UTC::now().naive_utc();

        let new_track_name = NewTrackName {
            track_id: track_id,
            name: name,
            locale: locale,
            is_default: is_default,
            is_original: is_original,
            created_at: now,
            updated_at: now,
        };

        diesel::insert(&new_track_name)
            .into(track_names::table)
            .get_result(self.connection)
            .expect("Error creating new track")
    }
}
