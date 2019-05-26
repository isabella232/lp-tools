use juniper::ID;

use crate::models::ArtistCredit;

#[juniper::object]
impl ArtistCredit {
    fn id(&self) -> ID {
        ID::from(format!("{}", self.id))
    }
}
