use lp::models::{ArtistCredit, Song};
use lp::repositories::{SongRepository, SongNameRepository, SongUrlRepository};
use toml::value::{Table, Value};

use ::{Context, readers};

pub fn create(ctx: &Context, root: &Value, artist_id: &str) -> Song {
    let artist_credit = artist_credit(ctx, root, artist_id);
    let song = new(ctx, &artist_credit);

    names(ctx, root, &song);

    if let Some(array) = root.get("contributions") {
        contributions(ctx, array, &song);
    }

    if let Some(array) = root.get("urls") {
        urls(ctx, array, &song);
    }

    song
}

fn artist_credit(ctx: &Context, root: &Value, artist_id: &str) -> ArtistCredit {
    if let Some(_) = root.get("artist-credits") {
        readers::artist_credit::create(ctx, root)
    } else {
        let mut table = Table::new();
        let key = String::from("artist-id");
        let artist_id_value = Value::String(artist_id.into());
        table.insert(key, artist_id_value);
        let value = Value::Table(table);
        readers::artist_credit::create(ctx, &value)
    }
}

fn new(ctx: &Context, artist_credit: &ArtistCredit) -> Song {
    let repo = SongRepository::new(ctx.connection());
    repo.create(artist_credit.id)
}

fn names(ctx: &Context, root: &Value, song: &Song) {
    let repo = SongNameRepository::new(ctx.connection());
    let values = root.get("names").and_then(Value::as_array).expect("song.names is not an array");

    for value in values {
        let name = value.get("name").and_then(Value::as_str).expect("song.names[_].name is not a string");
        let locale = value.get("locale").and_then(Value::as_str).expect("song.names[_].locale is not a string");
        let default = value.get("default").and_then(Value::as_bool).unwrap_or(false);
        let original = value.get("original").and_then(Value::as_bool).unwrap_or(false);

        repo.create(song.id, name, locale, default, original);
    }
}

fn contributions(ctx: &Context, array: &Value, song: &Song) {
    let values = array.as_array().expect("song.contributions is not an array");

    for value in values {
        readers::contribution::create(ctx, &value, song);
    }
}

fn urls(ctx: &Context, array: &Value, song: &Song) {
    let repo = SongUrlRepository::new(ctx.connection());
    let values = array.as_array().expect("song.urls is not an array");

    for value in values {
        let url = value.get("url").and_then(Value::as_str).expect("song.urls[_].url is not a string");
        let name = value.get("name").and_then(Value::as_str).expect("song.urls[_].name is not a string");

        repo.create(song.id, url, name);
    }
}
