use chrono::NaiveDate;
use lp::models::{Album, Medium, Release};
use lp::repositories::{ReleaseRepository, ReleaseUrlRepository};
use toml::Value;

use ::Context;
use ::readers::{self, Error};

pub fn create(ctx: &Context, root: &Value, album: &Album) -> Result<(Release, Vec<Medium>), Error> {
    let release = new(ctx, root, album)?;

    let media = root.get("media")
        .ok_or_else(|| {
            Error::Parse(String::from("expected release.media to exist"))
        })
        .and_then(|array| {
            media(ctx, &release, array)
        })?;

    if let Some(array) = root.get("urls") {
        urls(ctx, array, &release)?;
    }

    Ok((release, media))
}

fn new(ctx: &Context, root: &Value, album: &Album) -> Result<Release, Error> {
    let released_on = root.get("released-on")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            Error::Parse(String::from("expected release.released-on to be a string"))
        })
        .and_then(|s| {
            NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|_| {
                Error::Parse(format!("invalid release.released-on ({})", s))
            })
        })?;

    let country = root.get("country").and_then(Value::as_str);
    let catalog_number = root.get("catalog-number").and_then(Value::as_str);
    let disambiguation = root.get("disambiguation").and_then(Value::as_str);

    let repo = ReleaseRepository::new(ctx.connection());
    let release = repo.create(album.id, released_on, country, catalog_number, disambiguation);

    Ok(release)
}

fn media(ctx: &Context, release: &Release, array: &Value) -> Result<Vec<Medium>, Error> {
     array.as_array()
        .ok_or_else(|| {
            Error::Parse(String::from("expected release.media to be an array"))
        })?
        .iter()
        .enumerate()
        .map(|(i, value)| readers::medium::create(ctx, value, release, (i + 1) as i16))
        .collect()
}

fn urls(ctx: &Context, array: &Value, release: &Release) -> Result<(), Error> {
    let repo = ReleaseUrlRepository::new(ctx.connection());

    let values = array.as_array().ok_or_else(|| {
        Error::Parse(String::from("expected release.urls to be an array"))
    })?;

    for (i, value) in values.iter().enumerate() {
        let url = value.get("url")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                Error::Parse(format!("expected release.urls[{}].url to be a string", i))
            })?;

        repo.create(release.id, url);
    }

    Ok(())
}
