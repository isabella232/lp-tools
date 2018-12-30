use juniper::ID;

use crate::graphql::Context;
use crate::models::ArtistCredit;

graphql_object!(ArtistCredit: Context |&self| {
    field id() -> ID {
        ID::from(format!("{}", self.id))
    }
});
