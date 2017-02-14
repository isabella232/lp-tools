use chrono::UTC;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel;

use models::{ArtistId, ArtistName, NewArtistName};

pub struct ArtistNameRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> ArtistNameRepository<'a> {
    pub fn new(connection: &PgConnection) -> ArtistNameRepository {
        ArtistNameRepository { connection: connection }
    }

    pub fn find_by_artist_id(&self, id: ArtistId) -> Vec<ArtistName> {
        use schema::artist_names::dsl::{artist_names, artist_id};

        artist_names
            .filter(artist_id.eq(id))
            .load(self.connection)
            .expect("failed to load names")
    }

    pub fn create(&self,
                  artist_id: ArtistId,
                  name: &str,
                  locale: &str,
                  is_default: bool,
                  is_original: bool) -> ArtistName {
        use schema::artist_names;

        let now = UTC::now().naive_utc();

        let new_artist_name = NewArtistName {
            artist_id: artist_id,
            name: name,
            locale: locale,
            is_default: is_default,
            is_original: is_original,
            created_at: now,
            updated_at: now,
        };

        diesel::insert(&new_artist_name)
            .into(artist_names::table)
            .get_result(self.connection)
            .expect("Error creating new artist")
    }
}
