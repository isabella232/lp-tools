use ::{Root, Release};
use reqwest::Client;

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

pub fn fetch_release(id: i32) -> Release {
    let client = Client::new();

    let payload = json!({
        "query": QUERY,
        "variables": {
            "id": id,
        }
    });

    let mut response = client.post(ENDPOINT)
        .json(&payload)
        .send()
        .unwrap();

    let root: Root = response.json().unwrap();
    root.data.release
}
