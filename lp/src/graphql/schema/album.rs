use juniper::ID;

use graphql::Context;
use models::{Album, AlbumKind, AlbumName, ArtistCredit};
use repositories::{ArtistCreditRepository, AlbumNameRepository};

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
