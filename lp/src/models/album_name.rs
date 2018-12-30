use chrono::NaiveDateTime;

use crate::models::AlbumId;
use crate::schema::album_names;

pub type AlbumNameId = i32;

#[derive(Debug, Queryable)]
pub struct AlbumName {
    pub id: AlbumNameId,
    pub album_id: AlbumId,
    pub name: String,
    pub locale: String,
    pub is_default: bool,
    pub is_original: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="album_names"]
pub struct NewAlbumName<'a> {
    pub album_id: AlbumId,
    pub name: &'a str,
    pub locale: &'a str,
    pub is_default: bool,
    pub is_original: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
