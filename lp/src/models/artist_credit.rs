use chrono::NaiveDateTime;

use crate::schema::artist_credits;

pub type ArtistCreditId = i32;

#[derive(Debug, Queryable)]
pub struct ArtistCredit {
    pub id: ArtistCreditId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="artist_credits"]
pub struct NewArtistCredit {
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
