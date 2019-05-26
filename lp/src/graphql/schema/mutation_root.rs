use juniper::GraphQLInputObject;

use crate::graphql::Context;
use crate::models::{Artist, ArtistKind};
use crate::repositories::ArtistRepository;

#[derive(GraphQLInputObject)]
struct CreateArtistInput {
    kind: String,
    country: Option<String>,
    disambiguation: Option<String>,
}

pub struct MutationRoot;

#[juniper::object(
    name = "Mutation",
    Context = Context,
)]
impl MutationRoot {
    fn create_artist(executor: &Executor, input: CreateArtistInput) -> Artist {
        let ctx = executor.context();
        let repo = ArtistRepository::new(ctx.connection());

        let kind: ArtistKind = input.kind.to_lowercase().parse().unwrap();
        let country = input.country.unwrap_or_else(|| String::from("ZZ"));
        let disambiguation = input.disambiguation.as_ref().map(String::as_ref);

        repo.create(kind as i32, &country, None, None, disambiguation)
    }
}
