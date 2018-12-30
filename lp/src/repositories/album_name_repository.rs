use chrono::Utc;
use diesel::{self, PgConnection};
use diesel::prelude::*;

use crate::models::{AlbumId, AlbumName, NewAlbumName};

pub struct AlbumNameRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> AlbumNameRepository<'a> {
    pub fn new(connection: &PgConnection) -> AlbumNameRepository {
        AlbumNameRepository { connection }
    }

    pub fn find_by_album_id(&self, id: AlbumId) -> Vec<AlbumName> {
        use crate::schema::album_names::dsl::{album_names, album_id};

        album_names
            .filter(album_id.eq(id))
            .load(self.connection)
            .expect("failed to load names")
    }


    pub fn create(&self,
                  album_id: AlbumId,
                  name: &str,
                  locale: &str,
                  is_default: bool,
                  is_original: bool) -> AlbumName {
        use crate::schema::album_names;

        let now = Utc::now().naive_utc();

        let new_album_name = NewAlbumName {
            album_id,
            name,
            locale,
            is_default,
            is_original,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(album_names::table)
            .values(&new_album_name)
            .get_result(self.connection)
            .expect("Error creating new album")
    }
}
