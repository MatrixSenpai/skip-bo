use juniper::futures::StreamExt;
use juniper::graphql_object;
use mongodb::bson::{Bson, doc};
use serde::{Deserialize, Serialize};
use sonyflake::Sonyflake;
use crate::database::{MainContext, MongodbKey};
use crate::game_models::draft_player::DraftPlayer;
use crate::game_models::identifier::Id;

#[derive(Deserialize, Serialize)]
pub struct DraftGame {
    pub id: u64,
    pub game_name: String,
    pub players: Vec<u64>,
}
impl DraftGame {
    pub fn new(
        id_generator: &Sonyflake,
        players: Vec<u64>,
        game_name: String
    ) -> Self {
        Self {
            game_name,
            players,
            id: id_generator.next_id().unwrap(),
        }
    }
}

#[graphql_object]
impl DraftGame {
    fn id(&self) -> Id { Id(self.id) }
    fn game_name(&self) -> &str { &self.game_name }
    async fn players(&self, ctx: &MainContext) -> Result<Vec<DraftPlayer>, String> {
        let collection = ctx.0.get_collection::<DraftPlayer>(MongodbKey::Player);

        let ids: Bson = self.players.clone().into_iter()
            .map(|id| Bson::Int64(id as i64))
            .collect::<Vec<_>>().into();
        let query = doc! { "id": { "$in": ids } };
        let players = collection.find(query, None).await.unwrap().collect::<Vec<_>>().await;

        Ok(players.into_iter().map(|p| p.unwrap()).collect())
    }
}