use serde_json::json;

use crate::{Release, Root};

static ENDPOINT: &'static str = "http://localhost:8000/graphql";

static QUERY: &'static str = r#"
    query GetRelease($id: ID!) {
        release(id: $id) {
            country
            releasedOn

            artworkUrls {
                original
            }

            album {
                names {
                    name
                    isDefault
                }

                artistCredit {
                    ...artistCreditFields
                }
            }

            media {
                position

                tracks {
                    position

                    names {
                        name
                        isDefault
                    }

                    artistCredit {
                        ...artistCreditFields
                    }
                }
            }
        }
    }

    fragment artistCreditFields on ArtistCredit {
        names {
            position
            name
            isDefault
            separator
        }
    }
"#;

pub fn fetch_release(id: i32) -> reqwest::Result<Release> {
    let client = reqwest::blocking::Client::new();

    let payload = json!({
        "query": QUERY,
        "variables": {
            "id": id,
        }
    });

    client
        .post(ENDPOINT)
        .json(&payload)
        .send()
        .and_then(|res| res.json::<Root>())
        .map(|root| root.data.release)
}
