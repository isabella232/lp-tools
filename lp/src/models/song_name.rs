use chrono::NaiveDateTime;

use crate::models::SongId;
use crate::schema::song_names;

pub type SongNameId = i32;

#[derive(Debug, Queryable)]
pub struct SongName {
    pub id: SongNameId,
    pub song_id: SongId,
    pub name: String,
    pub locale: String,
    pub is_default: bool,
    pub is_original: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "song_names"]
pub struct NewSongName<'a> {
    pub song_id: SongId,
    pub name: &'a str,
    pub locale: &'a str,
    pub is_default: bool,
    pub is_original: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
