use lp::models::ArtistCredit;
use lp::repositories::ArtistCreditRepository;
use toml::value::{Table, Value};

use crate::Context;
use crate::readers::{self, Error};

pub fn create(ctx: &Context, root: &Value) -> Result<ArtistCredit, Error> {
    if let Some(array) = root.get("artist-credits").and_then(Value::as_array) {
        new(ctx, array)
    } else if let Some(artist_id) = root.get("artist-id").and_then(Value::as_str) {
        let mut table = Table::new();
        let key = String::from("artist-id");
        let value = Value::String(String::from(artist_id));
        table.insert(key, value);
        new(ctx, &[Value::Table(table)])
    } else {
        Err(Error::Parse(String::from("artist-credits or artist-id missing")))
    }
}

fn new(ctx: &Context, array: &[Value]) -> Result<ArtistCredit, Error> {
    let repo = ArtistCreditRepository::new(ctx.connection());
    let artist_credit = repo.create();

    for (i, value) in array.iter().enumerate() {
        readers::artist_credit_name::create(ctx, value, &artist_credit, i as i16)?;
    }

    Ok(artist_credit)
}
