use juniper::graphql_object;
use serde::{Deserialize, Serialize};
use crate::models::identifier::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftPlayer {
    pub id: u64,
    pub name: String,
    pub team: u64,
}
#[graphql_object]
impl DraftPlayer {
    fn id(&self) -> Id { Id(self.id) }
    fn name(&self) -> &str { &self.name }
    fn team(&self) -> Id { Id(self.team) }
}