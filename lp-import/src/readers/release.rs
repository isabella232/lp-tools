use chrono::NaiveDate;
use lp::models::{Album, Medium, Release};
use lp::repositories::{ReleaseRepository, ReleaseUrlRepository};
use toml::Value;

use ::{Context, readers};

pub fn create(ctx: &Context, root: &Value, album: &Album) -> (Release, Vec<Medium>) {
    let release = new(ctx, root, album);

    let media = if let Some(array) = root.get("media") {
        media(ctx, &release, array)
    } else {
        panic!("release.media is missing");
    };

    if let Some(array) = root.get("urls") {
        urls(ctx, array, &release);
    }

    (release, media)
}

fn new(ctx: &Context, root: &Value, album: &Album) -> Release {
    let released_on = root.get("released-on")
        .and_then(Value::as_str)
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .expect("release.released-on expected");

    let country = root.get("country").and_then(Value::as_str);
    let catalog_number = root.get("catalog-number").and_then(Value::as_str);
    let disambiguation = root.get("disambiguation").and_then(Value::as_str);

    let repo = ReleaseRepository::new(ctx.connection());
    repo.create(album.id, released_on, country, catalog_number, disambiguation)
}

fn media(ctx: &Context, release: &Release, array: &Value) -> Vec<Medium> {
     array.as_array()
         .expect("release.media is not an array")
         .iter()
         .enumerate()
         .map(|(i, value)| readers::medium::create(ctx, value, release, (i + 1) as i16))
         .collect()
}

fn urls(ctx: &Context, array: &Value, release: &Release) {
    let repo = ReleaseUrlRepository::new(ctx.connection());
    let values = array.as_array().expect("urls not an array");

    for value in values {
        let url = value.get("url").and_then(Value::as_str).expect("url.url expected");
        repo.create(release.id, url);
    }
}
