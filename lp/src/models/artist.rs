use chrono::NaiveDateTime;
use std::str::FromStr;

use schema::artists;

pub type ArtistId = i32;

#[derive(Clone, Debug, GraphQLEnum)]
pub enum ArtistKind {
    Person,
    Group,
}

impl ArtistKind {
    pub fn from_i32(n: i32) -> Option<ArtistKind> {
        match n {
            0 => Some(ArtistKind::Person),
            1 => Some(ArtistKind::Group),
            _ => None,
        }
    }
}

impl FromStr for ArtistKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "person" => Ok(ArtistKind::Person),
            "group" => Ok(ArtistKind::Group),
            _ => Err(())
        }
    }
}

#[derive(Associations, Debug, Identifiable, Queryable)]
pub struct Artist {
    pub id: ArtistId,
    pub kind: i32,
    pub country: String,
    pub disambiguation: Option<String>,
    pub started_on_year: Option<i16>,
    pub started_on_month: Option<i16>,
    pub started_on_day: Option<i16>,
    pub ended_on_year: Option<i16>,
    pub ended_on_month: Option<i16>,
    pub ended_on_day: Option<i16>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="artists"]
pub struct NewArtist<'a> {
    pub kind: i32,
    pub country: &'a str,
    pub disambiguation: Option<&'a str>,
    pub started_on_year: Option<i16>,
    pub started_on_month: Option<i16>,
    pub started_on_day: Option<i16>,
    pub ended_on_year: Option<i16>,
    pub ended_on_month: Option<i16>,
    pub ended_on_day: Option<i16>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
