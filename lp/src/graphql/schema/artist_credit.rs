use juniper::ID;

use graphql::Context;
use models::ArtistCredit;

graphql_object!(ArtistCredit: Context |&self| {
    field id() -> ID {
        ID::from(format!("{}", self.id))
    }
});
