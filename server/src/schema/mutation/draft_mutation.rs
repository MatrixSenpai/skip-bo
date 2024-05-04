use super::Mutation;
use juniper::graphql_object;
use crate::database::{MainContext, MongodbKey};
use crate::models::database::draft_game::DraftGame;
use crate::models::database::draft_player::DraftPlayer;
use crate::models::graphql::client_draft_game::{CreateDraftGameRequest, CreateDraftGameResponse, UpdateDraftGameRequest};
use crate::models::identifier::Id;

#[graphql_object]
impl Mutation {
    async fn create_draft_game(&self, ctx: &MainContext, data: CreateDraftGameRequest) -> Result<CreateDraftGameResponse, String> {
        let snowflake_gen = &ctx.0.id_generator;

        let host_id = snowflake_gen.next_id().unwrap();
        let host = DraftPlayer {
            id: host_id,
            name: data.host_name.clone(),
            team: 0,
        };
        let player_collection = ctx.0.get_collection::<DraftPlayer>(MongodbKey::Player);
        player_collection.insert_one(host, None).await.unwrap();

        let game_id = snowflake_gen.next_id().unwrap();
        let game_code = nanoid::nanoid!(4);
        let game = DraftGame {
            id: game_id,
            game_code: game_code.clone(),
            game_name: data.game_name.unwrap_or(format!("{}'s game", data.host_name)),
            private: data.private_game,
            teams: data.teams,
            computer_filled: data.computer_filled,
            players: vec![host_id]
        };
        let game_collection = ctx.0.get_collection::<DraftGame>(MongodbKey::Game);
        game_collection.insert_one(game, None).await.unwrap();

        let response = CreateDraftGameResponse {
            game_code,
            game_id: Id(game_id),
            host_player_id: Id(host_id),
        };

        Ok(response)
    }
    // async fn update_draft_game(&self, ctx: &MainContext, data: UpdateDraftGameRequest) -> Result<(), String> {
    //     todo!()
    // }
    // async fn join_draft_team(&self, ctx: &MainContext, player_id: Id) -> Result<(), String> {
    //     todo!()
    // }
}