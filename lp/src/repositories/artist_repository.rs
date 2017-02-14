use chrono::UTC;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel;

use ::PartialDate;
use models::{Artist, ArtistId, NewArtist};
use models::ArtistName;

pub struct ArtistRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> ArtistRepository<'a> {
    pub fn new(connection: &PgConnection) -> ArtistRepository {
        ArtistRepository { connection: connection }
    }

    pub fn find(&self, id: ArtistId) -> Option<Artist> {
        use schema::artists::dsl::artists;
        artists.find(id).first(self.connection).ok()
    }

    pub fn search(&self, query: &str) -> Vec<Artist> {
        use schema::{artists, artist_names};

        let pattern = format!("%{}%", query);

        artists::table
            .inner_join(artist_names::table)
            .filter(artist_names::name.like(&pattern))
            .load::<(Artist, ArtistName)>(self.connection)
            .unwrap()
            .into_iter()
            .map(|(a, _)| a)
            .collect()
    }

    pub fn create(
        &self,
        kind: i32,
        country: &str,
        started_on: Option<PartialDate>,
        ended_on: Option<PartialDate>,
        disambiguation: Option<&str>,
    ) -> Artist {
        use schema::artists;

        let started_on = started_on.unwrap_or(PartialDate::default());
        let ended_on = ended_on.unwrap_or(PartialDate::default());

        let now = UTC::now().naive_utc();

        let new_artist = NewArtist {
            kind: kind,
            country: country,
            disambiguation: disambiguation,
            started_on_year: started_on.year,
            started_on_month: started_on.month,
            started_on_day: started_on.day,
            ended_on_year: ended_on.year,
            ended_on_month: ended_on.month,
            ended_on_day: ended_on.day,
            created_at: now,
            updated_at: now,
        };

        diesel::insert(&new_artist)
            .into(artists::table)
            .get_result(self.connection)
            .expect("Error creating new artist")
    }
}
