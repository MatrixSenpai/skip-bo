use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};
use crate::models::identifier::Id;

#[derive(GraphQLInputObject, Debug, Serialize, Deserialize)]
pub struct CreateDraftGameRequest {
    pub host_name: String,
    pub game_name: Option<String>,
    pub private_game: bool,
    pub teams: bool,
    pub computer_filled: bool,
}

#[derive(GraphQLObject, Debug, Serialize, Deserialize)]
pub struct CreateDraftGameResponse {
    pub game_code: String,
    pub game_id: Id,
    pub host_player_id: Id,
}

#[derive(GraphQLInputObject, Debug, Serialize, Deserialize)]
pub struct UpdateDraftGameRequest {
    host_name: Option<String>,
    game_name: Option<String>,
    private_game: Option<bool>,
    teams: Option<bool>,
    computer_filled: Option<bool>,
    publish_game: Option<bool>,
}

#[derive(GraphQLObject, Debug, Serialize, Deserialize)]
pub struct GetDraftGameResponse {
    pub id: Id,
    pub game_name: String,
    pub game_code: String,
    pub private_game: bool,
    pub teams: bool,
    pub computer_filled: bool,
}
