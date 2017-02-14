use chrono::UTC;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel;

use models::{Song, ArtistCreditId, NewSong};

pub struct SongRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> SongRepository<'a> {
    pub fn new(connection: &PgConnection) -> SongRepository {
        SongRepository { connection: connection }
    }

    pub fn create(&self, artist_credit_id: ArtistCreditId) -> Song {
        use schema::songs;

        let now = UTC::now().naive_utc();

        let new_song = NewSong {
            artist_credit_id: artist_credit_id,
            created_at: now,
            updated_at: now,
        };

        diesel::insert(&new_song)
            .into(songs::table)
            .get_result(self.connection)
            .expect("Error creating new song")
    }
}
