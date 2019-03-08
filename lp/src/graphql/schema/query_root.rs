use juniper::{graphql_object, ID};

use crate::graphql::Context;
use crate::models::Artist;
use crate::repositories::ArtistRepository;

pub struct QueryRoot;

graphql_object!(QueryRoot: Context as "Query" |&self| {
    field artist(&executor, id: ID) -> Option<Artist> {
        id.parse().ok().and_then(|id| {
            let ctx = executor.context();
            let repo = ArtistRepository::new(ctx.connection());
            repo.find(id)
        })
    }

    field artists(&executor, query: String) -> Vec<Artist> {
        let ctx = executor.context();
        let repo = ArtistRepository::new(ctx.connection());
        repo.search(&query)
    }
});
