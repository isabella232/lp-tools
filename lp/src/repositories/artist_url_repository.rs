use chrono::UTC;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel;

use models::{ArtistId, ArtistUrl, NewArtistUrl};

pub struct ArtistUrlRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> ArtistUrlRepository<'a> {
    pub fn new(connection: &PgConnection) -> ArtistUrlRepository {
        ArtistUrlRepository { connection: connection }
    }

    pub fn create(&self, artist_id: ArtistId, url: &str) -> ArtistUrl {
        use schema::artist_urls;

        let now = UTC::now().naive_utc();

        let new_artist_url = NewArtistUrl {
            artist_id: artist_id,
            url: url,
            name: "[untitled]",
            created_at: now,
            updated_at: now,
        };

        diesel::insert(&new_artist_url)
            .into(artist_urls::table)
            .get_result(self.connection)
            .expect("Error creating new artist url")
    }
}
