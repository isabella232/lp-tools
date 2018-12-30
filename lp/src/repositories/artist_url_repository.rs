use chrono::Utc;
use diesel::{self, PgConnection};
use diesel::prelude::*;

use crate::models::{ArtistId, ArtistUrl, NewArtistUrl};

pub struct ArtistUrlRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> ArtistUrlRepository<'a> {
    pub fn new(connection: &PgConnection) -> ArtistUrlRepository {
        ArtistUrlRepository { connection }
    }

    pub fn find_by_artist_id(&self, id: ArtistId) -> Vec<ArtistUrl> {
        use crate::schema::artist_urls::dsl::{artist_urls, artist_id};

        artist_urls
            .filter(artist_id.eq(id))
            .load(self.connection)
            .expect("failed to load names")
    }

    pub fn create(&self, artist_id: ArtistId, url: &str) -> ArtistUrl {
        use crate::schema::artist_urls;

        let now = Utc::now().naive_utc();

        let new_artist_url = NewArtistUrl {
            artist_id,
            url,
            name: "[untitled]",
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(artist_urls::table)
            .values(&new_artist_url)
            .get_result(self.connection)
            .expect("Error creating new artist url")
    }
}
