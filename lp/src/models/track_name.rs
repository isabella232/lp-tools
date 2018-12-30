use chrono::NaiveDateTime;

use crate::models::TrackId;
use crate::schema::track_names;

pub type TrackNameId = i32;

#[derive(Debug, Queryable)]
pub struct TrackName {
    pub id: TrackNameId,
    pub track_id: TrackId,
    pub name: String,
    pub locale: String,
    pub is_default: bool,
    pub is_original: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="track_names"]
pub struct NewTrackName<'a> {
    pub track_id: TrackId,
    pub name: &'a str,
    pub locale: &'a str,
    pub is_default: bool,
    pub is_original: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
