use reqwest::Client;
use tauri::State;
use crate::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct CreateDraftGameVariables<'a> {
    pub data: CreateDraftGameRequest<'a>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "CreateDraftGameVariables")]
pub struct CreateDraftGame {
    #[arguments(data: $data)]
    pub create_draft_game: CreateDraftGameResponse,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct CreateDraftGameResponse {
    pub game_id: Id,
    pub game_code: String,
}

#[derive(cynic::InputObject, Debug)]
pub struct CreateDraftGameRequest<'a> {
    pub host_name: &'a str,
    pub game_name: Option<&'a str>,
    pub private_game: bool,
    pub teams: bool,
    pub computer_filled: bool,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct Id(pub String);

#[tauri::command]
pub async fn create_draft_game(state: State<'_, Client>) -> Result<(), String> {
    
    Ok(())
}