use chrono::UTC;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel;

use models::{AlbumId, AlbumName, NewAlbumName};

pub struct AlbumNameRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> AlbumNameRepository<'a> {
    pub fn new(connection: &PgConnection) -> AlbumNameRepository {
        AlbumNameRepository { connection: connection }
    }

    pub fn find_by_album_id(&self, id: AlbumId) -> Vec<AlbumName> {
        use schema::album_names::dsl::{album_names, album_id};

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
        use schema::album_names;

        let now = UTC::now().naive_utc();

        let new_album_name = NewAlbumName {
            album_id: album_id,
            name: name,
            locale: locale,
            is_default: is_default,
            is_original: is_original,
            created_at: now,
            updated_at: now,
        };

        diesel::insert(&new_album_name)
            .into(album_names::table)
            .get_result(self.connection)
            .expect("Error creating new album")
    }
}
