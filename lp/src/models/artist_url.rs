use chrono::NaiveDateTime;

use models::ArtistId;
use schema::artist_urls;

pub type ArtistUrlId = i32;

#[derive(Debug, Queryable)]
pub struct ArtistUrl {
    pub id: ArtistUrlId,
    pub artist_id: ArtistId,
    pub url: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="artist_urls"]
pub struct NewArtistUrl<'a> {
    pub artist_id: ArtistId,
    pub url: &'a str,
    pub name: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
