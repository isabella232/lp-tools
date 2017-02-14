use lp::models::{Medium, MediumKind, Release};
use lp::repositories::MediumRepository;
use toml::Value;

use ::Context;

pub fn create(ctx: &Context, root: &Value, release: &Release, i: i16) -> Medium {
    let kind = root.get("kind")
        .and_then(Value::as_str)
        .and_then(|s| s.parse::<MediumKind>().ok())
        .expect("invalid medium kind");

    let position = if let Some(p) = root.get("position").and_then(Value::as_integer) {
        p as i16
    } else {
        i
    };

    let name = root.get("name").and_then(Value::as_str);

    let repo = MediumRepository::new(ctx.connection());
    repo.create(release.id, kind as i32, position, name)
}
