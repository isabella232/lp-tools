use lp::models::{Artist, ArtistKind};
use lp::repositories::{ArtistRepository, ArtistNameRepository, ArtistUrlRepository};
use toml::Value;

use ::{Context, readers};

pub fn create(ctx: &Context, root: &Value) -> Artist {
    let artist = new(ctx, root);
    names(ctx, root, &artist);

    if let Some(array) = root.get("urls") {
        urls(ctx, array, &artist);
    }

    if let Some(array) = root.get("members") {
        members(ctx, array, &artist);
    }

    artist
}

fn new(ctx: &Context, root: &Value) -> Artist {
    let kind = root.get("kind")
        .and_then(Value::as_str)
        .and_then(|s| s.parse::<ArtistKind>().ok())
        .expect("invalid artist.kind");

    let country = root.get("country")
        .and_then(Value::as_str)
        .expect("invalid artist.country");

    let started_on = root.get("started-on")
        .and_then(Value::as_str)
        .and_then(|s| s.parse().ok());

    let ended_on = root.get("ended-on")
        .and_then(Value::as_str)
        .and_then(|s| s.parse().ok());

    let disambiguation = root.get("disambiguation").and_then(Value::as_str);

    let repo = ArtistRepository::new(ctx.connection());
    repo.create(kind as i32, country, started_on, ended_on, disambiguation)
}

fn names(ctx: &Context, root: &Value, artist: &Artist) {
    let repo = ArtistNameRepository::new(ctx.connection());
    let values = root.get("names").and_then(Value::as_array).expect("artist.names is not an array");

    for value in values {
        let name = value.get("name").and_then(Value::as_str).expect("artist.names[_].name is not a string");
        let locale = value.get("locale").and_then(Value::as_str).expect("artist.names[_].locale is not a string");
        let default = value.get("default").and_then(Value::as_bool).unwrap_or(false);
        let original = value.get("original").and_then(Value::as_bool).unwrap_or(false);

        repo.create(artist.id, name, locale, default, original);
    }
}

fn members(ctx: &Context, array: &Value, artist: &Artist) {
    let values = array.as_array().expect("artist.members is not an array");

    for value in values {
        readers::membership::create(ctx, value, artist);
    }
}

fn urls(ctx: &Context, array: &Value, artist: &Artist) {
    let repo = ArtistUrlRepository::new(ctx.connection());
    let values = array.as_array().expect("artist.urls is not an array");

    for value in values {
        let url = value.get("url").and_then(Value::as_str).expect("artist.urls[_].url is not a string");
        // let name = value.get("name").and_then(Value::as_str).expect("artist.urls[_].name is not a string");
        repo.create(artist.id, url);
    }
}
