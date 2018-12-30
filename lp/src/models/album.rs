use std::str::FromStr;

use chrono::NaiveDateTime;
use juniper::GraphQLEnum;

use crate::schema::albums;
use crate::models::ArtistCreditId;

pub type AlbumId = i32;

#[derive(Clone, Debug, GraphQLEnum)]
pub enum AlbumKind {
    Single,
    EP,
    LP,
}

impl AlbumKind {
    pub fn from_i32(n: i32) -> Option<AlbumKind> {
        match n {
             0 => Some(AlbumKind::Single),
             1 => Some(AlbumKind::EP),
             2 => Some(AlbumKind::LP),
             _ => None
        }
    }
}

impl FromStr for AlbumKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(AlbumKind::Single),
            "ep" => Ok(AlbumKind::EP),
            "lp" => Ok(AlbumKind::LP),
            _ => Err(())
        }
    }
}

#[derive(Debug, Queryable)]
pub struct Album {
    pub id: AlbumId,
    pub artist_credit_id: ArtistCreditId,
    pub kind: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="albums"]
pub struct NewAlbum {
    pub artist_credit_id: ArtistCreditId,
    pub kind: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
