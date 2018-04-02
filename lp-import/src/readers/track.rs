use lp::Duration;
use lp::models::{Medium, Track};
use lp::repositories::{SongNameRepository, TrackRepository, TrackNameRepository};
use toml::Value;

use ::Context;
use ::readers::{self, Error};

pub fn create(ctx: &Context, root: &Value, medium_id: &str, medium: &Medium) -> Result<Track, Error> {
    let track = new(ctx, root, medium_id, medium)?;
    names(ctx, root, &track)?;
    Ok(track)
}

fn new(ctx: &Context, root: &Value, medium_id: &str, medium: &Medium) -> Result<Track, Error> {
    let position = root.get("position")
        .and_then(Value::as_integer)
        .ok_or_else(|| {
            Error::Parse(String::from("expected track.position to be an integer"))
        })?;

    let duration = root.get("duration")
        .and_then(Value::as_str)
        .and_then(|s| s.parse::<Duration>().ok())
        .map(|d| d.into());

    let song_ref;

    let song = if let Some(id) = root.get("song-id").and_then(Value::as_str) {
        ctx.songs.get(id).ok_or_else(|| {
            Error::Map(format!("invalid track.song-id ({})", id))
        })?
    } else {
        let pieces: Vec<&str> = medium_id.rsplitn(4, '/').collect();
        let artist_id = pieces.last().unwrap();
        song_ref = readers::song::create(ctx, root, artist_id)?;
        &song_ref
    };

    let artist_credit_id = if let Some(_) = root.get("artist-credits") {
        let artist_credit = readers::artist_credit::create(ctx, root)?;
        artist_credit.id
    } else {
        song.artist_credit_id
    };

	let repo = TrackRepository::new(ctx.connection());
    let track = repo.create(medium.id, artist_credit_id, song.id, position as i16, duration);

    Ok(track)
}

fn names(ctx: &Context, root: &Value, track: &Track) -> Result<(), Error> {
    let repo = TrackNameRepository::new(ctx.connection());

    if let Some(array) = root.get("names") {
        let values = array.as_array()
            .ok_or_else(|| {
                Error::Parse(String::from("expected track.names to be an array"))
            })?;

        for (i, value) in values.iter().enumerate() {
            let name = value.get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    Error::Parse(format!("expected track.names[{}].name to be a string", i))
                })?;

            let locale = value.get("locale")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    Error::Parse(format!("expected track.names[{}].locale to be a string", i))
                })?;

            let default = value.get("default")
                .and_then(Value::as_bool)
                .unwrap_or(false);

            let original = value.get("original")
                .and_then(Value::as_bool)
                .unwrap_or(false);

            repo.create(track.id, name, locale, default, original);
        }
    } else {
        let names = SongNameRepository::new(ctx.connection()).find_by_song_id(track.song_id);

        for name in names {
            repo.create(track.id, &name.name, &name.locale, name.is_default, name.is_original);
        }
    }

    Ok(())
}
