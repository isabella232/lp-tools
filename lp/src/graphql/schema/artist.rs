use juniper::{graphql_object, ID};

use crate::graphql::Context;
use crate::models::{Album, Artist, ArtistKind, ArtistName, ArtistUrl};
use crate::repositories::{AlbumRepository, ArtistNameRepository, ArtistUrlRepository};

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

    field urls(&executor) -> Vec<ArtistUrl> {
        let ctx = executor.context();
        let repo = ArtistUrlRepository::new(ctx.connection());
        repo.find_by_artist_id(self.id)
    }
});
