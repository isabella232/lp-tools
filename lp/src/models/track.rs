use chrono::NaiveDateTime;

use crate::models::{ArtistCreditId, MediumId, SongId};
use crate::schema::tracks;

pub type TrackId = i32;

#[derive(Debug, Queryable)]
pub struct Track {
    pub id: TrackId,
    pub medium_id: MediumId,
    pub artist_credit_id: ArtistCreditId,
    pub song_id: SongId,
    pub position: i16,
    pub duration: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="tracks"]
pub struct NewTrack {
    pub medium_id: MediumId,
    pub artist_credit_id: ArtistCreditId,
    pub song_id: SongId,
    pub position: i16,
    pub duration: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
