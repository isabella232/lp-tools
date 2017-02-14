use chrono::UTC;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel;

use models::{Track, ArtistCreditId, MediumId, NewTrack, SongId};

pub struct TrackRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> TrackRepository<'a> {
    pub fn new(connection: &PgConnection) -> TrackRepository {
        TrackRepository { connection: connection }
    }

    pub fn create(&self,
                  medium_id: MediumId,
                  artist_credit_id: ArtistCreditId,
                  song_id: SongId,
                  position: i16,
                  duration: Option<i32>) -> Track {
        use schema::tracks;

        let now = UTC::now().naive_utc();

        let new_track = NewTrack {
            medium_id: medium_id,
            artist_credit_id: artist_credit_id,
            song_id: song_id,
            position: position,
            duration: duration,
            created_at: now,
            updated_at: now,
        };

        diesel::insert(&new_track)
            .into(tracks::table)
            .get_result(self.connection)
            .expect("Error creating new track")
    }
}
