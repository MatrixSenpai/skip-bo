use super::database::{Id, Database};
use juniper::graphql_object;

use crate::game::Game;

pub struct Query;
#[graphql_object(context = Database)]
impl Query {
    fn api_version() -> &'static str {
        "game_states.0.1-alpha.1"
    }

    async fn active_games<'db>(&self, ctx: &'db Database) -> Vec<Id> {
        let state = ctx.game_states.lock().await;
        state.keys().copied().collect()
    }

    async fn game_state<'db>(&self, ctx: &'db Database, game_id: Id) -> Result<Game, String> {
        let state = ctx.game_states.lock().await;

        let game = state.iter().find(|(id, _)| id == &&game_id);

        match game {
            Some(game) => Ok(game.1.clone()),
            None => Err("Unknown game id".to_string()),
        }
    }
}
