use juniper::graphql_object;
use serde::{Deserialize, Serialize};
use sonyflake::Sonyflake;
use crate::game_models::identifier::Id;

#[derive(Deserialize, Serialize, Clone)]
pub struct DraftPlayer {
    pub id: u64,
    pub team_id: Option<u64>,
    pub name: String,
}
impl DraftPlayer {
    pub fn new(id_generator: &Sonyflake, name: String) -> Self {
        Self {
            name,
            id: id_generator.next_id().unwrap(),
            team_id: None,
        }
    }
    pub fn set_team(&mut self, other: Option<u64>) {
        if let Some(other) = other {
            self.team_id = Some(
                self.id ^ other
            );
        }
    }
}

#[graphql_object]
impl DraftPlayer {
    fn id(&self) -> Id { Id(self.id) }
    fn team(&self) -> Option<Id> { self.team_id.map(|id| Id(id)) }
    fn name(&self) -> &str { &self.name }
}