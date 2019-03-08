use lp::models::{Artist, ArtistCredit, ArtistName};
use lp::repositories::{ArtistCreditNameRepository, ArtistNameRepository};
use toml::Value;

use crate::readers::Error;
use crate::Context;

pub fn create(
    ctx: &Context,
    root: &Value,
    artist_credit: &ArtistCredit,
    position: i16,
) -> Result<(), Error> {
    let artist_id = root
        .get("artist-id")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::Parse(String::from("expected artist-id to be a string")))?;

    let separator = root.get("separator").and_then(Value::as_str).unwrap_or("");

    let artist = ctx.artists.get(artist_id).ok_or_else(|| {
        Error::Map(format!(
            "invalid artist-credits[{}].artist-id ({})",
            position, artist_id,
        ))
    })?;

    if let Some(raw_names) = root.get("names").and_then(Value::as_array) {
        new_from_raw_names(ctx, &artist, artist_credit, position, raw_names, separator)?;
    } else {
        let repo = ArtistNameRepository::new(ctx.connection());
        let artist_names = repo.find_by_artist_id(artist.id);
        let names: Vec<ArtistName> = artist_names
            .into_iter()
            .filter(|n| n.is_default || n.is_original)
            .collect();
        new_from_artist_names(ctx, &artist, artist_credit, position, &names, separator)?;
    }

    Ok(())
}

fn new_from_raw_names(
    ctx: &Context,
    artist: &Artist,
    artist_credit: &ArtistCredit,
    position: i16,
    names: &[Value],
    separator: &str,
) -> Result<(), Error> {
    let repo = ArtistCreditNameRepository::new(ctx.connection());

    for (i, value) in names.iter().enumerate() {
        let name = value.get("name").and_then(Value::as_str).ok_or_else(|| {
            Error::Parse(format!(
                "expected artist-credit.names[{}].name to be a string",
                i
            ))
        })?;

        let locale = value.get("locale").and_then(Value::as_str).ok_or_else(|| {
            Error::Parse(format!(
                "expected artist-credit.names[{}].locale to be a string",
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

        repo.create(
            artist.id,
            artist_credit.id,
            position,
            name,
            locale,
            default,
            original,
            separator,
        );
    }

    Ok(())
}

fn new_from_artist_names(
    ctx: &Context,
    artist: &Artist,
    artist_credit: &ArtistCredit,
    position: i16,
    names: &[ArtistName],
    separator: &str,
) -> Result<(), Error> {
    let repo = ArtistCreditNameRepository::new(ctx.connection());

    for name in names {
        repo.create(
            artist.id,
            artist_credit.id,
            position,
            &name.name,
            &name.locale,
            name.is_default,
            name.is_original,
            separator,
        );
    }

    Ok(())
}
