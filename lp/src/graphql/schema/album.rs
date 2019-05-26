use juniper::ID;

use crate::graphql::Context;
use crate::models::{Album, AlbumKind, AlbumName, ArtistCredit};
use crate::repositories::{AlbumNameRepository, ArtistCreditRepository};

#[juniper::object(Context = Context)]
impl Album {
    fn id(&self) -> ID {
        ID::from(format!("{}", self.id))
    }

    fn artist_credit(executor: &Executor) -> ArtistCredit {
        let ctx = executor.context();
        let repo = ArtistCreditRepository::new(ctx.connection());
        repo.find(self.artist_credit_id).unwrap()
    }

    fn kind(&self) -> AlbumKind {
        AlbumKind::from_i32(self.kind).unwrap()
    }

    fn names(executor: &Executor) -> Vec<AlbumName> {
        let ctx = executor.context();
        let repo = AlbumNameRepository::new(ctx.connection());
        repo.find_by_album_id(self.id)
    }
}
