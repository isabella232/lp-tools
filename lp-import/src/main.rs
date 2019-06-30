use std::{
    env,
    fs::{self, File},
    io::Read,
    path::Path,
};

use diesel::prelude::*;
use futures::Future;
use futures_cpupool::CpuPool;
use glob::glob;
use lp::models::ReleaseId;
use lp_import::{parameterize, readers, Context};
use lp_magick::resize;
use rand::distributions::Uniform;
use rand::prelude::*;
use toml::Value;

const ID_LEN: usize = 32;

static ARTIST_KINDS: [&'static str; 2] = ["people", "groups"];
static HEX_CHARSET: &'static [u8] = b"0123456789abcdef";

pub struct HexGenerator {
    range: Uniform<usize>,
}

impl HexGenerator {
    fn new() -> HexGenerator {
        HexGenerator {
            range: Uniform::new(0, HEX_CHARSET.len()),
        }
    }
}

impl Distribution<char> for HexGenerator {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        let i = self.range.sample(rng);
        HEX_CHARSET[i] as char
    }
}

fn read_toml<F>(pattern: &str, mut callback: F)
where
    F: FnMut(&Path, Value),
{
    let entries = glob(pattern)
        .expect("bad glob pattern")
        .filter_map(Result::ok);

    for entry in entries {
        let mut file = File::open(&entry).expect("could not open file");
        let mut toml = String::new();
        file.read_to_string(&mut toml).expect("could not read file");

        let data = toml.parse().unwrap_or_else(|e| {
            panic!("{}: {}", entry.display(), e);
        });

        callback(&entry, data);
    }
}

fn medium_kind_to_label(kind: i32) -> &'static str {
    match kind {
        0 => "cd",
        1 => "dvd",
        2 => "blu-ray",
        3 => "digital",
        4 => "vinyl",
        _ => unreachable!(),
    }
}

fn update_release_artwork_data(release_id: ReleaseId, original_id: &str, thumbnail_id: &str) {
    use lp::schema::releases::dsl::*;

    let ctx = Context::new();

    let data = format!(
        r#"{{"original":{{"id":"{}"}},"thumbnail":{{"id":"{}"}}}}"#,
        original_id, thumbnail_id,
    );

    diesel::update(releases.find(release_id))
        .set(artwork_data.eq(&data))
        .execute(ctx.connection())
        .unwrap();
}

fn main() {
    let mut args = env::args().skip(1);
    let pathname = args.next().expect("missing working directory");
    let store_dir = args.next();

    if let Some(ref dst) = store_dir {
        fs::create_dir_all(dst).expect("failed to create store_dir");
    }

    let pool = CpuPool::new_num_cpus();
    let mut tasks = Vec::new();

    let mut rng = rand::thread_rng();
    let hex_generator = HexGenerator::new();

    let mut ctx = Context::new();
    let suffix = ".toml";

    // artists

    for kind in &ARTIST_KINDS {
        let prefix = format!("{}/artists/{}/", pathname, kind);
        let pattern = format!("{}**/*{}", prefix, suffix);

        read_toml(&pattern, |path, value| {
            let path = path.to_str().unwrap();
            let start = prefix.len();
            let end = path.len() - suffix.len();
            let id = &path[start..end];

            let artist = match readers::artist::create(&ctx, &value) {
                Ok(a) => a,
                Err(e) => panic!("{}: {:?}", path, e),
            };

            ctx.artists.insert(String::from(id), artist);
        });
    }

    // albums

    let prefix = format!("{}/albums/", pathname);
    let start = prefix.len();
    let pattern = format!("{}**/*{}", prefix, suffix);

    read_toml(&pattern, |path, value| {
        let path = path.to_str().unwrap();
        let end = path.len() - suffix.len();
        let id = &path[start..end];

        let (_, releases) = match readers::album::create(&ctx, &value) {
            Ok(a) => a,
            Err(e) => panic!("{}: {:?}", path, e),
        };

        for (release, media) in releases {
            let disambiguation = if let Some(ref d) = release.disambiguation {
                parameterize(d)
            } else {
                String::from("default")
            };

            let first_format = medium_kind_to_label(media[0].kind);

            for medium in media {
                let medium_kind = medium_kind_to_label(medium.kind);
                let medium_id = format!(
                    "{}/{}/{}{}",
                    id, disambiguation, medium_kind, medium.position
                );
                ctx.media.insert(medium_id, medium);
            }

            if let Some(ref dst_prefix) = store_dir {
                let id = id.to_string();
                let release_id = release.id;
                let dst_prefix = dst_prefix.clone();
                let pathname = pathname.to_string();
                let disambiguation = disambiguation.to_string();

                let original_id: String = (&hex_generator)
                    .sample_iter(&mut rng)
                    .take(ID_LEN)
                    .collect();
                let thumbnail_id: String = (&hex_generator)
                    .sample_iter(&mut rng)
                    .take(ID_LEN)
                    .collect();

                let task = pool.spawn_fn(move || {
                    let res: Result<(), ()> = Ok(());

                    let prefix = format!("{}/-attachments/albums/", pathname);

                    let mut pathname = format!("{}/{}-{}.jpg", id, disambiguation, first_format);
                    let mut src = format!("{}{}", prefix, pathname);

                    if !Path::new(&src).exists() {
                        pathname = format!("{}/{}.jpg", id, disambiguation);
                        src = format!("{}{}", prefix, pathname);

                        if !Path::new(&src).exists() {
                            panic!("missing artwork: {}", src);
                        }
                    }

                    let dst = format!("{}/{}.jpg", dst_prefix, original_id);
                    fs::copy(&src, &dst).expect("artwork copy failed");

                    let thumbnail_src = format!("tmp/cache/{}", pathname);
                    let dst = format!("{}/{}.jpg", dst_prefix, thumbnail_id);

                    if !Path::new(&thumbnail_src).exists() {
                        let cache_dir = format!("tmp/cache/{}", id);
                        fs::create_dir_all(cache_dir).unwrap();
                        resize(&src, &thumbnail_src, 256, 256);
                    }

                    fs::copy(&thumbnail_src, &dst).expect("thumbnail copy failed");

                    update_release_artwork_data(release_id, &original_id, &thumbnail_id);

                    res
                });

                tasks.push(task);
            }
        }
    });

    // songs

    let prefix = format!("{}/songs/", pathname);
    let start = prefix.len();
    let pattern = format!("{}**/*{}", prefix, suffix);

    read_toml(&pattern, |path, value| {
        let mut p = path.to_path_buf();
        p.pop();
        let p = p.to_str().unwrap();
        let artist_id = &p[start..];

        let path = path.to_str().unwrap();
        let end = path.len() - suffix.len();
        let id = &path[start..end];

        let song = match readers::song::create(&ctx, &value, artist_id) {
            Ok(s) => s,
            Err(e) => panic!("{}: {:?}", path, e),
        };

        ctx.songs.insert(String::from(id), song);
    });

    // tracklists

    let pattern = format!("{}/tracklists/**/*.toml", pathname);

    read_toml(&pattern, |path, value| {
        if let Err(e) = readers::tracklist::create(&ctx, &value) {
            let path = path.to_str().unwrap();
            panic!("{}: {:?}", path, e);
        }
    });

    // attachments

    for task in tasks {
        task.wait().unwrap();
    }
}
