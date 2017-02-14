use toml::Value;

use ::{Context, readers};

pub fn create(ctx: &Context, root: &Value) {
    let values = root.get("medium-ids").and_then(Value::as_array).expect("medium-ids invalid");

    for value in values {
        let id = value.as_str().expect("medium id not a string");
        tracks(ctx, root, id);
    }
}

fn tracks(ctx: &Context, root: &Value, medium_id: &str) {
    let values = root.get("tracks").and_then(Value::as_array).expect("tracks invalid");
    let medium = ctx.media.get(medium_id.into()).expect("media missing");

    for value in values {
        readers::track::create(ctx, value, medium_id, medium);
    }
}
