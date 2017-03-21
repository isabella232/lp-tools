extern crate glob;
extern crate lp_tag;

use glob::glob;
use lp_tag::{AttachedPictureFrame, File, TextIdentificationFrame};
use lp_tag::api::fetch_release;
use lp_tag::ffi::{PictureType, StringType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let release_id = args.get(1).and_then(|id| id.parse::<i32>().ok()).expect("missing release id");
    let working_dir = args.get(2).expect("missing working dir");

    let release = fetch_release(release_id);
    println!("{:?}", release);

    let disc_number = 1;
    let medium = release.media.iter()
        .find(|m| m.position == disc_number)
        .expect("media not found");
    let tracks = &medium.tracks;

    let pattern = format!("{}/*.mp3", glob::Pattern::escape(working_dir));

    let entries = glob(&pattern)
        .expect("bad glob pattern")
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    if entries.len() != tracks.len() {
        panic!("track count does not match working dir (expected {}, got {})",
            tracks.len(),
            entries.len());
    }

    let artwork = release.artwork();
    let genre = release.guess_genre();
    let year = release.year();

    for (pathname, track) in entries.iter().zip(tracks.iter()) {
        let pathname = pathname.to_str().unwrap();
        println!("{}", pathname);

        let file = File::new(&pathname);
        file.strip();

        let tag = file.tag();

        tag.set_title(&track.default_name());
        tag.set_artist(&track.artist_credit.default_name());
        tag.set_album(&release.album.default_name());
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
    }
}
