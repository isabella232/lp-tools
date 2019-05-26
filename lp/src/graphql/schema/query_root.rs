use juniper::ID;

use crate::graphql::Context;
use crate::models::Artist;
use crate::repositories::ArtistRepository;

pub struct QueryRoot;

#[juniper::object(
    name = "Query",
    Context = Context,
)]
impl QueryRoot {
    fn artist(executor: &Executor, id: ID) -> Option<Artist> {
        id.parse().ok().and_then(|id| {
            let ctx = executor.context();
            let repo = ArtistRepository::new(ctx.connection());
            repo.find(id)
        })
    }

    fn artists(executor: &Executor, query: String) -> Vec<Artist> {
        let ctx = executor.context();
        let repo = ArtistRepository::new(ctx.connection());
        repo.search(&query)
    }
}
