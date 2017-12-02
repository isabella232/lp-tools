use chrono::Utc;
use diesel::{self, PgConnection};
use diesel::prelude::*;

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

        let now = Utc::now().naive_utc();

        let new_track = NewTrack {
            medium_id: medium_id,
            artist_credit_id: artist_credit_id,
            song_id: song_id,
            position: position,
            duration: duration,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(tracks::table)
            .values(&new_track)
            .get_result(self.connection)
            .expect("Error creating new track")
    }
}
