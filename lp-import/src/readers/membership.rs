use lp::models::{Artist, Membership};
use lp::repositories::MembershipRepository;
use toml::Value;

use ::Context;
use ::readers::{self, Error};

pub fn create(ctx: &Context, root: &Value, artist: &Artist) -> Result<Membership, Error> {
    new(ctx, root, artist)
}

fn new(ctx: &Context, root: &Value, artist: &Artist) -> Result<Membership, Error> {
    let artist_credit = readers::artist_credit::create(ctx, root)?;

    let started_on = root.get("started-on")
        .and_then(Value::as_str)
        .and_then(|s| s.parse().ok());

    let ended_on = root.get("ended-on")
        .and_then(Value::as_str)
        .and_then(|s| s.parse().ok());

    let repo = MembershipRepository::new(ctx.connection());
    let membership = repo.create(artist.id, artist_credit.id, started_on, ended_on);

    Ok(membership)
}
