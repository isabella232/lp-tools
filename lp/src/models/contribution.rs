use chrono::NaiveDateTime;
use std::str::FromStr;

use crate::schema::contributions;
use crate::models::{ArtistCreditId, SongId};

pub type ContributionId = i32;

#[derive(Clone, Debug)]
pub enum ContributionKind {
    Performer,
    Arranger,
    Composer,
    Lyricist,
}

impl FromStr for ContributionKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "performer" => Ok(ContributionKind::Performer),
            "arranger" => Ok(ContributionKind::Arranger),
            "composer" => Ok(ContributionKind::Composer),
            "lyricist" => Ok(ContributionKind::Lyricist),
            _ => Err(())
        }
    }
}


#[derive(Debug, Queryable)]
pub struct Contribution {
    pub id: ContributionId,
    pub artist_credit_id: ArtistCreditId,
    pub song_id: SongId,
    pub kind: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="contributions"]
pub struct NewContribution {
    pub artist_credit_id: ArtistCreditId,
    pub song_id: SongId,
    pub kind: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
