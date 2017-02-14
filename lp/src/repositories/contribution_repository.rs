use chrono::UTC;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel;

use models::{Contribution, ArtistCreditId, NewContribution, SongId};

pub struct ContributionRepository<'a> {
    connection: &'a PgConnection,
}

impl<'a> ContributionRepository<'a> {
    pub fn new(connection: &PgConnection) -> ContributionRepository {
        ContributionRepository { connection: connection }
    }

    pub fn create(&self,
                  artist_credit_id: ArtistCreditId,
                  song_id: SongId,
                  kind: i32) -> Contribution {
        use schema::contributions;

        let now = UTC::now().naive_utc();

        let new_contribution = NewContribution {
            artist_credit_id: artist_credit_id,
            song_id: song_id,
            kind: kind,
            created_at: now,
            updated_at: now,
        };

        diesel::insert(&new_contribution)
            .into(contributions::table)
            .get_result(self.connection)
            .expect("Error creating new contribution")
    }
}
