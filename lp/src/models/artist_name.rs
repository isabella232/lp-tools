use chrono::NaiveDateTime;

use schema::artist_names;
use models::{Artist, ArtistId};

pub type ArtistNameId = i32;

#[derive(Associations, Debug, Identifiable, Queryable)]
#[belongs_to(Artist)]
pub struct ArtistName {
    pub id: ArtistNameId,
    pub artist_id: ArtistId,
    pub name: String,
    pub locale: String,
    pub is_default: bool,
    pub is_original: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="artist_names"]
pub struct NewArtistName<'a> {
    pub artist_id: ArtistId,
    pub name: &'a str,
    pub locale: &'a str,
    pub is_default: bool,
    pub is_original: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
