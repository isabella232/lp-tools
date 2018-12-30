use juniper::{graphql_object, ID};

use crate::graphql::Context;
use crate::models::ArtistUrl;

graphql_object!(ArtistUrl: Context |&self| {
    field id() -> ID {
        ID::from(format!("{}", self.id))
    }

    field url() -> &str {
        &self.url
    }

    field name() -> &str {
        &self.name
    }
});
