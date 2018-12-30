use chrono::Utc;
use diesel::{self, PgConnection};
use diesel::prelude::*;

use crate::models::{Contribution, ArtistCreditId, NewContribution, SongId};

pub struct ContributionRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> ContributionRepository<'a> {
    pub fn new(connection: &PgConnection) -> ContributionRepository {
        ContributionRepository { connection }
    }

    pub fn create(&self,
                  artist_credit_id: ArtistCreditId,
                  song_id: SongId,
                  kind: i32) -> Contribution {
        use crate::schema::contributions;

        let now = Utc::now().naive_utc();

        let new_contribution = NewContribution {
            artist_credit_id,
            song_id,
            kind,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(contributions::table)
            .values(&new_contribution)
            .get_result(self.connection)
            .expect("Error creating new contribution")
    }
}
