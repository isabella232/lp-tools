use lp::models::{ArtistCredit, Contribution, ContributionKind, Song};
use lp::repositories::ContributionRepository;
use toml::Value;

use ::{Context, readers};

pub fn create(ctx: &Context, root: &Value, song: &Song) -> Contribution {
    let artist_credit = artist_credit(ctx, root);
    new(ctx, root, &artist_credit, song)
}

fn artist_credit(ctx: &Context, root: &Value) -> ArtistCredit {
    readers::artist_credit::create(ctx, root)
}

fn new(ctx: &Context, root: &Value, artist_credit: &ArtistCredit, song: &Song) -> Contribution {
    let kind = root.get("kind")
        .and_then(Value::as_str)
        .and_then(|s| s.parse::<ContributionKind>().ok())
        .expect("invalid contribution.kind");

    let repo = ContributionRepository::new(ctx.connection());
    repo.create(artist_credit.id, song.id, kind as i32)
}
