use juniper::{graphql_object, ID};

use crate::graphql::Context;
use crate::models::{Album, AlbumKind, AlbumName, ArtistCredit};
use crate::repositories::{AlbumNameRepository, ArtistCreditRepository};

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
