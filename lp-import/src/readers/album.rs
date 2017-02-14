use lp::models::{Album, AlbumKind, Medium, Release};
use lp::repositories::{AlbumRepository, AlbumNameRepository};
use toml::Value;

use ::{Context, readers};

pub fn create(ctx: &Context, root: &Value) -> (Album, Vec<(Release, Vec<Medium>)>) {
    let album = new(ctx, root);
    names(ctx, root, &album);
    let releases = releases(ctx, root, &album);
    (album, releases)
}

fn new(ctx: &Context, root: &Value) -> Album {
    let artist_credit = readers::artist_credit::create(ctx, root);

    let kind = root.get("kind")
        .and_then(Value::as_str)
        .and_then(|s| s.parse::<AlbumKind>().ok())
        .expect("invalid album.kind");

    let repo = AlbumRepository::new(ctx.connection());
    repo.create(artist_credit.id, kind as i32)
}

fn names(ctx: &Context, root: &Value, album: &Album) {
    let repo = AlbumNameRepository::new(ctx.connection());
    let values = root.get("names").and_then(Value::as_array).expect("album.names is not an array");

    for value in values {
        let name = value.get("name").and_then(Value::as_str).expect("album.names[_].name is not a string");
        let locale = value.get("locale").and_then(Value::as_str).expect("album.names[_].locale is not a string");
        let default = value.get("default").and_then(Value::as_bool).unwrap_or(false);
        let original = value.get("original").and_then(Value::as_bool).unwrap_or(false);

        repo.create(album.id, name, locale, default, original);
    }
}

fn releases(ctx: &Context, root: &Value, album: &Album) -> Vec<(Release, Vec<Medium>)> {
    root.get("releases")
        .and_then(Value::as_array)
        .map(|values| {
            values.iter()
                .map(|value| readers::release::create(ctx, value, album))
                .collect()
        })
        .expect("invalid releases")
}
