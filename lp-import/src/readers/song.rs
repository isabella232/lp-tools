use lp::models::{ArtistCredit, Song};
use lp::repositories::{SongRepository, SongNameRepository, SongUrlRepository};
use toml::value::{Table, Value};

use ::Context;
use ::readers::{self, Error};

pub fn create(ctx: &Context, root: &Value, artist_id: &str) -> Result<Song, Error> {
    let artist_credit = artist_credit(ctx, root, artist_id)?;
    let song = new(ctx, &artist_credit);

    names(ctx, root, &song)?;

    if let Some(array) = root.get("contributions") {
        contributions(ctx, array, &song)?;
    }

    if let Some(array) = root.get("urls") {
        urls(ctx, array, &song)?;
    }

    Ok(song)
}

fn artist_credit(ctx: &Context, root: &Value, artist_id: &str) -> Result<ArtistCredit, Error> {
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

fn names(ctx: &Context, root: &Value, song: &Song) -> Result<(), Error> {
    let repo = SongNameRepository::new(ctx.connection());

    let values = root.get("names")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            Error::Parse(String::from("expected song.names to be an array"))
        })?;

    for (i, value) in values.iter().enumerate() {
        let name = value.get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                Error::Parse(format!("expected song.names[{}].name to be a string", i))
            })?;

        let locale = value.get("locale")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                Error::Parse(format!("expected song.names[{}].locale to be a string", i))
            })?;

        let default = value.get("default")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        let original = value.get("original")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        repo.create(song.id, name, locale, default, original);
    }

    Ok(())
}

fn contributions(ctx: &Context, array: &Value, song: &Song) -> Result<(), Error> {
    let values = array.as_array().ok_or_else(|| {
        Error::Parse(String::from("expected song.contributions to be an array"))
    })?;

    for value in values {
        readers::contribution::create(ctx, &value, song)?;
    }

    Ok(())
}

fn urls(ctx: &Context, array: &Value, song: &Song) -> Result<(), Error> {
    let repo = SongUrlRepository::new(ctx.connection());

    let values = array.as_array().ok_or_else(|| {
        Error::Parse(String::from("expected song.urls to be an array"))
    })?;

    for (i, value) in values.iter().enumerate() {
        let url = value.get("url")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                Error::Parse(format!("expected artist.urls[{}].url to be a string", i))
            })?;

        let name = value.get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                Error::Parse(format!("expected artist.urls[{}].name to be a string", i))
            })?;

        repo.create(song.id, url, name);
    }

    Ok(())
}
