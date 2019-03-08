use toml::Value;

use crate::readers::{self, Error};
use crate::Context;

pub fn create(ctx: &Context, root: &Value) -> Result<(), Error> {
    let values = root
        .get("medium-ids")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            Error::Parse(String::from("expected tracklist.medium-ids to be an array"))
        })?;

    for (i, value) in values.iter().enumerate() {
        let id = value.as_str().ok_or_else(|| {
            Error::Parse(format!(
                "expected tracklist.medium-ids[{}] to be a string",
                i
            ))
        })?;

        tracks(ctx, root, id, i)?;
    }

    Ok(())
}

fn tracks(ctx: &Context, root: &Value, medium_id: &str, i: usize) -> Result<(), Error> {
    let values = root
        .get("tracks")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::Parse(String::from("expected tracklist.tracks to be an array")))?;

    let medium = ctx.media.get(medium_id).ok_or_else(|| {
        Error::Map(format!(
            "invalid tracklist.medium-ids[{}] ({})",
            i, medium_id
        ))
    })?;

    for value in values {
        readers::track::create(ctx, value, medium_id, medium)?;
    }

    Ok(())
}
