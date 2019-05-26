use juniper::ID;

use crate::graphql::Context;
use crate::models::{Album, Artist, ArtistKind, ArtistName, ArtistUrl};
use crate::repositories::{AlbumRepository, ArtistNameRepository, ArtistUrlRepository};

#[juniper::object(Context = Context)]
impl Artist {
    fn id(&self) -> ID {
        ID::from(format!("{}", self.id))
    }

    fn kind(&self) -> ArtistKind {
        ArtistKind::from_i32(self.kind).unwrap()
    }

    fn country(&self) -> &str {
        &self.country
    }

    fn disambiguation(&self) -> Option<&String> {
        self.disambiguation.as_ref()
    }

    fn names(executor: &Executor) -> Vec<ArtistName> {
        let ctx = executor.context();
        let repo = ArtistNameRepository::new(ctx.connection());
        repo.find_by_artist_id(self.id)
    }

    fn albums(executor: &Executor) -> Vec<Album> {
        let ctx = executor.context();
        let repo = AlbumRepository::new(ctx.connection());
        repo.find_by_artist_id(self.id)
    }

    fn urls(executor: &Executor) -> Vec<ArtistUrl> {
        let ctx = executor.context();
        let repo = ArtistUrlRepository::new(ctx.connection());
        repo.find_by_artist_id(self.id)
    }
}
