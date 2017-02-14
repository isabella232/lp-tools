use chrono::NaiveDateTime;
use std::str::FromStr;

use schema::media;
use models::ReleaseId;

pub type MediumId = i32;

#[derive(Clone, Debug)]
pub enum MediumKind {
    CD,
    DVD,
    BluRay,
    Digital,
    Vinyl,
}

impl FromStr for MediumKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cd" => Ok(MediumKind::CD),
            "dvd" => Ok(MediumKind::DVD),
            "blu-ray" => Ok(MediumKind::BluRay),
            "digital" => Ok(MediumKind::Digital),
            "vinyl" => Ok(MediumKind::Vinyl),
            _ => Err(())
        }
    }
}

#[derive(Debug, Queryable)]
pub struct Medium {
    pub id: MediumId,
    pub release_id: ReleaseId,
    pub kind: i32,
    pub position: i16,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="media"]
pub struct NewMedium<'a> {
    pub release_id: ReleaseId,
    pub kind: i32,
    pub position: i16,
    pub name: Option<&'a str>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
