use chrono::UTC;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel;

use models::{SongId, SongUrl, NewSongUrl};

pub struct SongUrlRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> SongUrlRepository<'a> {
    pub fn new(connection: &PgConnection) -> SongUrlRepository {
        SongUrlRepository { connection: connection }
    }

    pub fn create(&self, song_id: SongId, url: &str, name: &str) -> SongUrl {
        use schema::song_urls;

        let now = UTC::now().naive_utc();

        let new_song_url = NewSongUrl {
            song_id: song_id,
            url: url,
            name: name,
            created_at: now,
            updated_at: now,
        };

        diesel::insert(&new_song_url)
            .into(song_urls::table)
            .get_result(self.connection)
            .expect("Error creating new song url")
    }
}
