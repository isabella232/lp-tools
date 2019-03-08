use lp::models::{ArtistCredit, Contribution, ContributionKind, Song};
use lp::repositories::ContributionRepository;
use toml::Value;

use crate::readers::{self, Error};
use crate::Context;

pub fn create(ctx: &Context, root: &Value, song: &Song) -> Result<Contribution, Error> {
    let artist_credit = artist_credit(ctx, root)?;
    new(ctx, root, &artist_credit, song)
}

fn artist_credit(ctx: &Context, root: &Value) -> Result<ArtistCredit, Error> {
    readers::artist_credit::create(ctx, root)
}

fn new(
    ctx: &Context,
    root: &Value,
    artist_credit: &ArtistCredit,
    song: &Song,
) -> Result<Contribution, Error> {
    let kind = root
        .get("kind")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::Parse(String::from("expected contribution.kind to be a string")))
        .and_then(|s| {
            s.parse::<ContributionKind>()
                .map_err(|_| Error::Parse(format!("invalid contribution.kind ({})", s)))
        })?;

    let repo = ContributionRepository::new(ctx.connection());
    let contribution = repo.create(artist_credit.id, song.id, kind as i32);

    Ok(contribution)
}
