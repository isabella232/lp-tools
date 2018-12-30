use chrono::NaiveDateTime;

use crate::schema::artist_credit_names;
use crate::models::{ArtistCreditId, ArtistId};

pub type ArtistCreditNameId = i32;

#[derive(Debug, Queryable)]
pub struct ArtistCreditName {
    pub id: ArtistCreditNameId,
    pub artist_id: ArtistId,
    pub artist_credit_id: ArtistCreditId,
    pub position: i16,
    pub name: String,
    pub locale: String,
    pub is_default: bool,
    pub is_original: bool,
    pub separator: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="artist_credit_names"]
pub struct NewArtistCreditName<'a> {
    pub artist_id: ArtistId,
    pub artist_credit_id: ArtistCreditId,
    pub position: i16,
    pub name: &'a str,
    pub locale: &'a str,
    pub is_default: bool,
    pub is_original: bool,
    pub separator: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
