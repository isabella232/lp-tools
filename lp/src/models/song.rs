use chrono::NaiveDateTime;

use schema::songs;
use models::ArtistCreditId;

pub type SongId = i32;

#[derive(Debug, Queryable)]
pub struct Song {
    pub id: SongId,
    pub artist_credit_id: ArtistCreditId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="songs"]
pub struct NewSong {
    pub artist_credit_id: ArtistCreditId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
