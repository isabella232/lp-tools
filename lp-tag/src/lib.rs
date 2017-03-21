extern crate libc;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use std::io::Read;

pub use taglib::{AttachedPictureFrame, File, TextIdentificationFrame};

pub mod api;
pub mod ffi;
pub mod taglib;

#[derive(Debug, Deserialize)]
pub struct Root {
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub release: Release,
}

#[derive(Debug, Deserialize)]
pub struct ArtistCredit {
    pub names: Vec<ArtistCreditName>,
}

impl ArtistCredit {
    pub fn default_name(&self) -> String {
        let mut names = self.names.clone();
        names.sort_by_key(|n| n.position);

        names.iter()
            .filter(|n| n.is_default)
            .map(|n| format!("{}{}", n.name, n.separator))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistCreditName {
    pub position: i32,
    pub name: String,
    pub is_default: bool,
    pub separator: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub artist_credit: ArtistCredit,
    pub names: Vec<AlbumName>,
}

impl Album {
    pub fn default_name(&self) -> String {
        self.names.iter()
            .find(|n| n.is_default)
            .expect("missing default album name")
            .name
            .clone()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumName {
    pub name: String,
    pub is_default: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub country: String,
    pub released_on: String,
    pub artwork_url: String,
    pub album: Album,
    pub media: Vec<Medium>,
}

impl Release {
    pub fn artwork(&self) -> Vec<u8> {
        let url = format!("http://localhost:8000{}", self.artwork_url);
        let mut response = reqwest::get(&url).unwrap();

        let mut data = Vec::new();
        response.read_to_end(&mut data).unwrap();;

        data
    }

    pub fn guess_genre(&self) -> &'static str {
        match self.country.as_ref() {
            "JP" => "Japanese Pop",
            "KR" => "Korean Pop",
            _ => panic!("unknown country"),
        }
    }

    pub fn year(&self) -> u32 {
        self.released_on
            .split("-")
            .next()
            .and_then(|y| y.parse().ok())
            .expect("invalid release date")
    }
}

#[derive(Debug, Deserialize)]
pub struct Medium {
    pub position: i32,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub position: i32,
    pub artist_credit: ArtistCredit,
    pub names: Vec<TrackName>,
}

impl Track {
    pub fn default_name(&self) -> String {
        self.names.iter()
            .find(|n| n.is_default)
            .expect("missing default track name")
            .name
            .clone()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackName {
    pub name: String,
    pub is_default: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artist_credit_default_name() {
        let names = vec![
            ArtistCreditName { position: 2, name: "소미".into(), is_default: false, separator: "".into() },
            ArtistCreditName { position: 2, name: "Somi".into(), is_default: true, separator: "".into() },
            ArtistCreditName { position: 1, name: "에릭남".into(), is_default: false, separator: " X ".into() },
            ArtistCreditName { position: 1, name: "Eric Nam".into(), is_default: true, separator: " X ".into() },
        ];

        let artist_credit = ArtistCredit { names: names };

        assert_eq!(artist_credit.default_name(), "Eric Nam X Somi");
    }

    #[test]
    fn test_album_default_name() {
        let names = vec![
            AlbumName { name: "From. 우주소녀".into(), is_default: false },
            AlbumName { name: "From. WJSN".into(), is_default: true },
        ];

        let artist_credit = ArtistCredit { names: vec![] };
        let album = Album { artist_credit: artist_credit, names: names };

        assert_eq!(album.default_name(), "From. WJSN")
    }

    fn build_release() -> Release {
        let artist_credit = ArtistCredit { names: vec![] };
        let album = Album { artist_credit: artist_credit, names: vec![] };

        Release {
            country: "KR".into(),
            released_on: "2017-03-13".into(),
            artwork_url: "http://localhost/artwork.jpg".into(),
            album: album,
            media: vec![],
        }
    }

    #[test]
    fn test_release_guess_genre() {
        let release = build_release();
        assert_eq!(release.guess_genre(), "Korean Pop");
    }

    #[test]
    fn test_release_year() {
        let release = build_release();
        assert_eq!(release.year(), 2017);
    }
}
