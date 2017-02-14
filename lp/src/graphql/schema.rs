use juniper::ID;

use graphql::Context;
use models::{Album, AlbumKind, AlbumName, Artist, ArtistCredit, ArtistKind, ArtistName};
use repositories::{
    ArtistRepository, AlbumRepository, AlbumNameRepository,
    ArtistCreditRepository, ArtistNameRepository,
};
use std::str::FromStr;

pub struct MutationRoot;
pub struct QueryRoot;

graphql_enum!(ArtistKind {
    ArtistKind::Person => "PERSON",
    ArtistKind::Group => "GROUP",
});

graphql_object!(Artist: Context |&self| {
    field id() -> ID {
        ID::from(format!("{}", self.id))
    }

    field kind() -> ArtistKind {
        ArtistKind::from_i32(self.kind).unwrap()
    }

    field country() -> &str {
        &self.country
    }

    field disambiguation() -> Option<&String> {
        self.disambiguation.as_ref()
    }

    field names(&executor) -> Vec<ArtistName> {
        let ctx = executor.context();
        let repo = ArtistNameRepository::new(ctx.connection());
        repo.find_by_artist_id(self.id)
    }

    field albums(&executor) -> Vec<Album> {
        let ctx = executor.context();
        let repo = AlbumRepository::new(ctx.connection());
        repo.find_by_artist_id(self.id)
    }
});

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

graphql_object!(ArtistCredit: Context |&self| {
    field id() -> ID {
        ID::from(format!("{}", self.id))
    }
});

graphql_enum!(AlbumKind {
    AlbumKind::Single => "SINGLE",
    AlbumKind::EP => "EP",
    AlbumKind::LP => "LP",
});

graphql_object!(Album: Context |&self| {
    field id() -> ID {
        ID::from(format!("{}", self.id))
    }

    field artist_credit(&executor) -> ArtistCredit {
        let ctx = executor.context();
        let repo = ArtistCreditRepository::new(ctx.connection());
        repo.find(self.artist_credit_id).unwrap()
    }

    field kind() -> AlbumKind {
        AlbumKind::from_i32(self.kind).unwrap()
    }

    field names(&executor) -> Vec<AlbumName> {
        let ctx = executor.context();
        let repo = AlbumNameRepository::new(ctx.connection());
        repo.find_by_album_id(self.id)
    }
});

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

graphql_input_object!(
    struct CreateArtistInput {
        kind: String,
        country: Option<String>,
        disambiguation: Option<String>,
    }
);

graphql_object!(MutationRoot: Context as "Mutation" |&self| {
    field create_artist(&executor, input: CreateArtistInput) -> Artist {
        let ctx = executor.context();
        let repo = ArtistRepository::new(ctx.connection());

        let kind = ArtistKind::from_str(&input.kind.to_lowercase()).unwrap();
        let country = input.country.unwrap_or(String::from("ZZ"));
        let disambiguation = input.disambiguation.as_ref().map(String::as_ref);

        repo.create(kind as i32, &country, None, None, disambiguation)
    }
});
