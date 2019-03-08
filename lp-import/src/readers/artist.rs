use lp::models::{Artist, ArtistKind};
use lp::repositories::{ArtistNameRepository, ArtistRepository, ArtistUrlRepository};
use toml::Value;

use crate::readers::{self, Error};
use crate::Context;

pub fn create(ctx: &Context, root: &Value) -> Result<Artist, Error> {
    let artist = new(ctx, root)?;

    names(ctx, root, &artist)?;

    if let Some(array) = root.get("urls") {
        urls(ctx, array, &artist)?;
    }

    if let Some(array) = root.get("members") {
        members(ctx, array, &artist)?;
    }

    Ok(artist)
}

fn new(ctx: &Context, root: &Value) -> Result<Artist, Error> {
    let kind = root
        .get("kind")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::Parse(String::from("expected artist.kind to be a string")))
        .and_then(|s| {
            s.parse::<ArtistKind>()
                .map_err(|_| Error::Parse(format!("invalid artist.kind ({})", s)))
        })?;

    let country = root
        .get("country")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::Parse(String::from("invalid artist.country")))?;

    let started_on = root
        .get("started-on")
        .and_then(Value::as_str)
        .and_then(|s| s.parse().ok());

    let ended_on = root
        .get("ended-on")
        .and_then(Value::as_str)
        .and_then(|s| s.parse().ok());

    let disambiguation = root.get("disambiguation").and_then(Value::as_str);

    let repo = ArtistRepository::new(ctx.connection());
    let artist = repo.create(kind as i32, country, started_on, ended_on, disambiguation);

    Ok(artist)
}

fn names(ctx: &Context, root: &Value, artist: &Artist) -> Result<(), Error> {
    let repo = ArtistNameRepository::new(ctx.connection());

    let values = root
        .get("names")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::Parse(String::from("expected artist.names to be an array")))?;

    for (i, value) in values.iter().enumerate() {
        let name = value.get("name").and_then(Value::as_str).ok_or_else(|| {
            Error::Parse(format!("expected artist.names[{}].name to be a string", i))
        })?;

        let locale = value.get("locale").and_then(Value::as_str).ok_or_else(|| {
            Error::Parse(format!(
                "expected artist.names[{}].locale to be a string",
                i
            ))
        })?;

        let default = value
            .get("default")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        let original = value
            .get("original")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        repo.create(artist.id, name, locale, default, original);
    }

    Ok(())
}

fn members(ctx: &Context, array: &Value, artist: &Artist) -> Result<(), Error> {
    let values = array
        .as_array()
        .ok_or_else(|| Error::Parse(String::from("expected artist.members to be an array")))?;

    for value in values {
        readers::membership::create(ctx, value, artist)?;
    }

    Ok(())
}

fn urls(ctx: &Context, array: &Value, artist: &Artist) -> Result<(), Error> {
    let repo = ArtistUrlRepository::new(ctx.connection());

    let values = array
        .as_array()
        .ok_or_else(|| Error::Parse(String::from("expected artist.urls to be an array")))?;

    for (i, value) in values.iter().enumerate() {
        let url = value.get("url").and_then(Value::as_str).ok_or_else(|| {
            Error::Parse(format!("expected artist.urls[{}].url to be a string", i))
        })?;

        repo.create(artist.id, url);
    }

    Ok(())
}
