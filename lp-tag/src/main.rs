use std::{env, fs, path::PathBuf};

use glob::glob;
use lazy_static::lazy_static;
use lp_tag::api::fetch_release;
use lp_tag::ffi::{PictureType, StringType};
use lp_tag::{AttachedPictureFrame, File, FrameFactory, TextIdentificationFrame};
use regex::Regex;
use unidecode::unidecode;

fn sanitize_pathname(s: &str) -> String {
    lazy_static! {
        // https://msdn.microsoft.com/en-us/library/windows/desktop/aa365247(v=vs.85).aspx
        static ref RESERVED_CHARACTERS: Regex = Regex::new(r#"[<>:"/\\|?*]"#).unwrap();
        static ref TRAILING_DOTS: Regex = Regex::new(r"\.+$").unwrap();
    }

    let s = unidecode(s);
    let s = s.replace("/", "_");
    let s = RESERVED_CHARACTERS.replace_all(&s, "");
    let s = TRAILING_DOTS.replace_all(&s, "");

    s.into_owned()
}

#[test]
fn test_sanitize_pathname() {
    assert_eq!(sanitize_pathname("foo bar.mp3"), "foo bar.mp3");
    assert_eq!(sanitize_pathname("foo / bar"), "foo _ bar");
    assert_eq!(sanitize_pathname(r#"a<b>c:d"e/f\g|h?i*"#), "abcde_fghi");
    assert_eq!(sanitize_pathname("foo."), "foo");
    assert_eq!(sanitize_pathname("foo.."), "foo");
    assert_eq!(sanitize_pathname("foo..."), "foo");
}

fn main() {
    let mut args = env::args().skip(1);
    let release_id = args
        .next()
        .and_then(|id| id.parse::<i32>().ok())
        .expect("missing release id");
    let working_dir = args.next().expect("missing working dir");

    let release = fetch_release(release_id).unwrap();

    let disc_number = 1;
    let medium = release
        .media
        .iter()
        .find(|m| m.position == disc_number)
        .expect("media not found");
    let tracks = &medium.tracks;

    let pattern = format!("{}/*.mp3", glob::Pattern::escape(&working_dir));

    let entries = glob(&pattern)
        .expect("bad glob pattern")
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    if entries.len() != tracks.len() {
        panic!(
            "track count does not match working dir (expected {}, got {})",
            tracks.len(),
            entries.len()
        );
    }

    FrameFactory::instance().set_default_text_encoding(StringType::UTF16);

    let artwork = release.artwork().unwrap();
    let album = release.album.default_name();
    let genre = release.guess_genre();
    let year = release.year();

    for (src, track) in entries.iter().zip(tracks.iter()) {
        let file = File::new(src);
        file.strip();

        let tag = file.tag();

        let title = track.default_name();

        tag.set_title(&title);
        tag.set_artist(&track.artist_credit.default_name());
        tag.set_album(&album);
        tag.set_genre(genre);
        tag.set_year(year);

        let trck = TextIdentificationFrame::new("TRCK", StringType::Latin1);
        trck.set_text(&format!("{}/{}", track.position, tracks.len()));
        tag.add_frame(&trck);

        let apic = AttachedPictureFrame::new();
        apic.set_mime_type("image/jpeg");
        apic.set_type(PictureType::FrontCover);
        apic.set_picture(&artwork);
        tag.add_frame(&apic);

        file.save();

        let mut dst = src.clone();
        dst.pop();
        let basename = sanitize_pathname(&title);
        dst.push(&format!("{:02} {}.mp3", track.position, basename));

        fs::rename(&src, &dst).unwrap();
    }

    let mut dst = PathBuf::from(&working_dir);
    dst.pop();
    let release_date = release.released_on.replace("-", ".");
    let name = sanitize_pathname(&album);
    dst.push(&format!("[{}] {}", release_date, name));

    fs::rename(&working_dir, &dst).unwrap();
}
