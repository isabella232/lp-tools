use chrono::NaiveDateTime;

use crate::models::SongId;
use crate::schema::song_urls;

pub type SongUrlId = i32;

#[derive(Debug, Queryable)]
pub struct SongUrl {
    pub id: SongUrlId,
    pub song_id: SongId,
    pub url: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="song_urls"]
pub struct NewSongUrl<'a> {
    pub song_id: SongId,
    pub url: &'a str,
    pub name: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
