use juniper::ID;

use graphql::Context;
use models::AlbumName;

graphql_object!(AlbumName: Context |&self| {
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
