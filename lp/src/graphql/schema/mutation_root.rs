use crate::graphql::Context;
use crate::models::{Artist, ArtistKind};
use crate::repositories::ArtistRepository;

pub struct MutationRoot;

#[derive(GraphQLInputObject)]
struct CreateArtistInput {
    kind: String,
    country: Option<String>,
    disambiguation: Option<String>,
}

graphql_object!(MutationRoot: Context as "Mutation" |&self| {
    field create_artist(&executor, input: CreateArtistInput) -> Artist {
        let ctx = executor.context();
        let repo = ArtistRepository::new(ctx.connection());

        let kind: ArtistKind = input.kind.to_lowercase().parse().unwrap();
        let country = input.country.unwrap_or(String::from("ZZ"));
        let disambiguation = input.disambiguation.as_ref().map(String::as_ref);

        repo.create(kind as i32, &country, None, None, disambiguation)
    }
});
