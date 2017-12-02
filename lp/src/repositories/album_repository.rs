use chrono::Utc;
use diesel::{self, PgConnection};
use diesel::prelude::*;

use models::{Album, ArtistCreditId, ArtistId, NewAlbum};

pub struct AlbumRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> AlbumRepository<'a> {
    pub fn new(connection: &PgConnection) -> AlbumRepository {
        AlbumRepository { connection: connection }
    }

    pub fn find_by_artist_id(&self, artist_id: ArtistId) -> Vec<Album> {
        use schema::{albums, artist_credit_names};
        use diesel::pg::expression::dsl::any;

        let name_ids = artist_credit_names::table
            .select(artist_credit_names::artist_credit_id)
            .filter(artist_credit_names::artist_id.eq(artist_id));

        albums::table
            .filter(albums::artist_credit_id.eq(any(name_ids)))
            .get_results(self.connection)
            .expect("failed to load albums")
    }

    pub fn create(&self, artist_credit_id: ArtistCreditId, kind: i32) -> Album {
        use schema::albums;

        let now = Utc::now().naive_utc();

        let new_album = NewAlbum {
            artist_credit_id: artist_credit_id,
            kind: kind,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(albums::table)
            .values(&new_album)
            .get_result(self.connection)
            .expect("Error creating new album")
    }
}
