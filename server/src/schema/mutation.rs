use juniper::graphql_object;

use super::database::{Id, Database, Pair};
use crate::{game::Game, player::Player, turn_info::{PlayAction, PlayBuildAction, PlayDiscardAction, TurnPhase}};

pub struct Mutation;
#[graphql_object(context = Database)]
impl Mutation {
    async fn new_game<'db>(
        &self, 
        ctx: &'db Database, 
        num_players: i32, 
        pair_information: Option<Vec<Pair>>,
    ) -> Result<Id, String> {
        let mut state = ctx.game_states.lock().await;
        let snowflake = &ctx.snowflake;

        if let Some(pairs) = &pair_information {
            for pair in pairs.iter() {
                if
                    pair.first_index < 0 || pair.first_index >= num_players ||
                    pair.last_index < 0 || pair.last_index >= num_players
                {
                    error!("Out of bounds pair info: {num_players} players, pair info: {pairs:?}");
                    return Err("Pair information references an out of index player".to_string());
                }
            }
        }
        let pair_map = pair_information.map(|inner| {
            inner.into_iter()
                .map(|pair| (pair.first_index as usize, pair.last_index as usize))
                .collect()
        });
        let new_game = Game::new(
            num_players as usize,
            pair_map,
            snowflake,
        );
        let new_game_id = Id(new_game.game_id);

        debug!("New game created: {new_game}");
        state.insert(new_game_id, new_game);

        Ok(new_game_id)
    }

    async fn draw_cards(&self, ctx: &Database, game: Id, player: Id) -> Result<Player, String> {
        let mut state = ctx.game_states.lock().await;
        let mut game = state.get_mut(&game).ok_or("Game ID not found".to_string())?;

        let player = game.handle_draw(player)?;
        Ok(player.clone())
    }

    async fn build_card(&self, ctx: &Database, game: Id, player: Id, action: PlayBuildAction) -> Result<i32, String> {
        let mut state = ctx.game_states.lock().await;
        let mut game = state.get_mut(&game).ok_or("Game ID not found".to_string())?;

        game.handle_build(player, action)?;
        Ok(0)
    }

    async fn discard_card(&self, ctx: &Database, game: Id, player: Id, action: PlayDiscardAction) -> Result<i32, String> {
        let mut state = ctx.game_states.lock().await;
        let mut game = state.get_mut(&game).ok_or("Game ID not found".to_string())?;

        game.handle_discard(player, action)?;
        Ok(0)
    }
}
