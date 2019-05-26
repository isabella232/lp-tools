use juniper::ID;

use crate::models::ArtistUrl;

#[juniper::object]
impl ArtistUrl {
    fn id(&self) -> ID {
        ID::from(format!("{}", self.id))
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn name(&self) -> &str {
        &self.name
    }
}
