use lp::models::{Medium, MediumKind, Release};
use lp::repositories::MediumRepository;
use toml::Value;

use crate::readers::Error;
use crate::Context;

pub fn create(ctx: &Context, root: &Value, release: &Release, i: i16) -> Result<Medium, Error> {
    let kind = root
        .get("kind")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::Parse(String::from("expected medium.kind to be a string")))
        .and_then(|s| {
            s.parse::<MediumKind>()
                .map_err(|_| Error::Parse(format!("invalid medium.kind ({})", s)))
        })?;

    let position = root
        .get("position")
        .and_then(Value::as_integer)
        .map(|p| p as i16)
        .unwrap_or(i);

    let name = root.get("name").and_then(Value::as_str);

    let repo = MediumRepository::new(ctx.connection());
    let medium = repo.create(release.id, kind as i32, position, name);

    Ok(medium)
}
