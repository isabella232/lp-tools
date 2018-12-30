use chrono::Utc;
use diesel::{self, PgConnection};
use diesel::prelude::*;

use crate::models::{SongId, SongUrl, NewSongUrl};

pub struct SongUrlRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> SongUrlRepository<'a> {
    pub fn new(connection: &PgConnection) -> SongUrlRepository {
        SongUrlRepository { connection }
    }

    pub fn create(&self, song_id: SongId, url: &str, name: &str) -> SongUrl {
        use crate::schema::song_urls;

        let now = Utc::now().naive_utc();

        let new_song_url = NewSongUrl {
            song_id,
            url,
            name,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(song_urls::table)
            .values(&new_song_url)
            .get_result(self.connection)
            .expect("Error creating new song url")
    }
}
