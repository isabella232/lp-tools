use chrono::NaiveDateTime;

use models::ReleaseId;
use schema::release_urls;

pub type ReleaseUrlId = i32;

#[derive(Debug, Queryable)]
pub struct ReleaseUrl {
    pub id: ReleaseUrlId,
    pub release_id: ReleaseId,
    pub url: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="release_urls"]
pub struct NewReleaseUrl<'a> {
    pub release_id: ReleaseId,
    pub url: &'a str,
    pub name: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
