use chrono::NaiveDateTime;

use models::{ArtistCreditId, ArtistId};
use schema::memberships;

pub type MembershipId = i32;

#[derive(Debug, Queryable)]
pub struct Membership {
    pub id: MembershipId,
    pub group_id: ArtistId,
    pub artist_credit_id: ArtistCreditId,
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
#[table_name="memberships"]
pub struct NewMembership {
    pub group_id: ArtistId,
    pub artist_credit_id: ArtistCreditId,
    pub started_on_year: Option<i16>,
    pub started_on_month: Option<i16>,
    pub started_on_day: Option<i16>,
    pub ended_on_year: Option<i16>,
    pub ended_on_month: Option<i16>,
    pub ended_on_day: Option<i16>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
