use chrono::Utc;
use diesel::prelude::*;
use diesel::{self, PgConnection};

use crate::models::{ArtistCredit, ArtistCreditId, NewArtistCredit};

pub struct ArtistCreditRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> ArtistCreditRepository<'a> {
    pub fn new(connection: &PgConnection) -> ArtistCreditRepository<'_> {
        ArtistCreditRepository { connection }
    }

    pub fn find(&self, id: ArtistCreditId) -> Option<ArtistCredit> {
        use crate::schema::artist_credits::dsl::artist_credits;
        artist_credits.find(id).first(self.connection).ok()
    }

    pub fn create(&self) -> ArtistCredit {
        use crate::schema::artist_credits;

        let now = Utc::now().naive_utc();

        let new_artist_credit = NewArtistCredit {
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(artist_credits::table)
            .values(&new_artist_credit)
            .get_result(self.connection)
            .expect("Error creating new artist credit")
    }
}
