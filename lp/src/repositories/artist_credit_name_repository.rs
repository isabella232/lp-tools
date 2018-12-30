use chrono::Utc;
use diesel::{self, PgConnection};
use diesel::prelude::*;

use crate::models::{ArtistId, ArtistCreditId, ArtistCreditName, NewArtistCreditName};

pub struct ArtistCreditNameRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> ArtistCreditNameRepository<'a> {
    pub fn new(connection: &PgConnection) -> ArtistCreditNameRepository {
        ArtistCreditNameRepository { connection }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create(&self,
                  artist_id: ArtistId,
                  artist_credit_id: ArtistCreditId,
                  position: i16,
                  name: &str,
                  locale: &str,
                  is_default: bool,
                  is_original: bool,
                  separator: &'a str) -> ArtistCreditName {
        use crate::schema::artist_credit_names;

        let now = Utc::now().naive_utc();

        let new_artist_credit_name = NewArtistCreditName {
            artist_id,
            artist_credit_id,
            position,
            name,
            locale,
            is_default,
            is_original,
            separator,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(artist_credit_names::table)
            .values(&new_artist_credit_name)
            .get_result(self.connection)
            .expect("Error creating new artist credit name")
    }
}
