use chrono::Utc;
use diesel::{self, PgConnection};
use diesel::prelude::*;

use crate::models::{SongId, SongName, NewSongName};

pub struct SongNameRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> SongNameRepository<'a> {
    pub fn new(connection: &PgConnection) -> SongNameRepository {
        SongNameRepository { connection }
    }

    pub fn find_by_song_id(&self, id: SongId) -> Vec<SongName> {
        use crate::schema::song_names::dsl::{song_names, song_id};

        song_names
            .filter(song_id.eq(id))
            .load(self.connection)
            .expect("failed to load names")
    }

    pub fn create(&self,
                  song_id: SongId,
                  name: &str,
                  locale: &str,
                  is_default: bool,
                  is_original: bool) -> SongName {
        use crate::schema::song_names;

        let now = Utc::now().naive_utc();

        let new_song_name = NewSongName {
            song_id,
            name,
            locale,
            is_default,
            is_original,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(song_names::table)
            .values(&new_song_name)
            .get_result(self.connection)
            .expect("Error creating new song")
    }
}
