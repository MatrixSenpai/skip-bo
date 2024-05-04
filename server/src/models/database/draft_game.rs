use futures::TryStreamExt;
use juniper::graphql_object;
use mongodb::bson::{Bson, doc};
use serde::{Deserialize, Serialize};
use crate::database::{MainContext, MongodbKey};
use crate::models::database::draft_player::DraftPlayer;
use crate::models::identifier::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftGame {
    pub id: u64,
    pub game_code: String,
    pub game_name: String,
    pub players: Vec<u64>,
    pub private: bool,
    pub teams: bool,
    pub computer_filled: bool,
}
#[graphql_object]
impl DraftGame {
    fn id(&self) -> Id { Id(self.id) }
    fn game_code(&self) -> &str { &self.game_code }
    fn game_name(&self) -> &str { &self.game_name }
    fn private(&self) -> bool { self.private }
    fn teams(&self) -> bool { self.teams }
    fn player_count(&self) -> Result<i32, String> { self.players.len().try_into().map_err(|_| "Not a sized number".to_string()) }

    async fn players(&self, ctx: &MainContext) -> Result<Vec<DraftPlayer>, String> {
        let current_lobby_header = match ctx.1.get("x-current-lobby") {
            Some(l) => l.to_str().unwrap(),
            None => return Err("Cannot request info for a lobby player is not in".to_string()),
        }.parse::<u64>().map_err(|_| "Invalid game lobby id".to_string())?;
        let current_player_header = match ctx.1.get("x-current-player") {
            Some(p) => p.to_str().unwrap(),
            None => return Err("Cannot request info for a lobby without a player id".to_string()),
        }.parse::<u64>().map_err(|_| "Invalid player id".to_string())?;

        if current_lobby_header != self.id || !self.players.contains(&current_player_header) {
            return Err("Cannot request info for a lobby player is not in".to_string());
        }

        let player_collection = ctx.0.get_collection::<DraftPlayer>(MongodbKey::Player);
        let players = self.players.clone().into_iter().map(|id| Bson::Int64(id.try_into().unwrap()))
            .collect::<Vec<_>>();
        let query = doc! { "id": { "$in": players } };
        let results = player_collection.find(query, None).await.unwrap()
            .try_collect::<Vec<DraftPlayer>>().await.unwrap();

        Ok(results)
    }
}