use lp::models::{Album, AlbumKind, Medium, Release};
use lp::repositories::{AlbumRepository, AlbumNameRepository};
use toml::Value;

use ::Context;
use ::readers::{self, Error};

pub type Releases = Vec<(Release, Vec<Medium>)>;

pub fn create(ctx: &Context, root: &Value) -> Result<(Album, Releases), Error> {
    let album = new(ctx, root)?;
    names(ctx, root, &album)?;
    let releases = releases(ctx, root, &album)?;
    Ok((album, releases))
}

fn new(ctx: &Context, root: &Value) -> Result<Album, Error> {
    let artist_credit = readers::artist_credit::create(ctx, root)?;

    let kind = root.get("kind")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            Error::Parse(String::from("expected album.kind to be a string"))
        })
        .and_then(|s| {
            s.parse::<AlbumKind>().map_err(|_| {
                Error::Parse(format!("invalid album.kind ({})", s))
            })
        })?;

    let repo = AlbumRepository::new(ctx.connection());
    let album = repo.create(artist_credit.id, kind as i32);

    Ok(album)
}

fn names(ctx: &Context, root: &Value, album: &Album) -> Result<(), Error> {
    let repo = AlbumNameRepository::new(ctx.connection());

    let values = root.get("names")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            Error::Parse(String::from("expected album.names to be an array"))
        })?;

    for (i, value) in values.iter().enumerate() {
        let name = value.get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                Error::Parse(format!("expected album.names[{}].name to be a string", i))
            })?;

        let locale = value.get("locale")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                Error::Parse(format!("expected album.names[{}].locale to be a string", i))
            })?;

        let default = value.get("default")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        let original = value.get("original")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        repo.create(album.id, name, locale, default, original);
    }

    Ok(())
}

fn releases(ctx: &Context, root: &Value, album: &Album) -> Result<Releases, Error> {
    root.get("releases")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            Error::Parse(String::from("expected release.media to be an array"))
        })?
        .iter()
        .map(|value| readers::release::create(ctx, value, album))
        .collect()
}
