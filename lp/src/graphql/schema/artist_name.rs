use juniper::ID;

use crate::graphql::Context;
use crate::models::ArtistName;

graphql_object!(ArtistName: Context |&self| {
    field id() -> ID {
        ID::from(format!("{}", self.id))
    }

    field name() -> &str {
        &self.name
    }

    field locale() -> &str {
        &self.locale
    }

    field is_default() -> bool {
        self.is_default
    }

    field is_original() -> bool {
        self.is_original
    }
});
