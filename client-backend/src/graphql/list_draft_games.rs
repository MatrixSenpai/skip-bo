use cynic::{GraphQlResponse, QueryBuilder};
use reqwest::Client;
use serde::Serialize;
use tauri::State;

use crate::{GRAPHQL_URL, schema};

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct PublicGamesQuery {
    pub public_games: Vec<DraftGame>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
pub struct DraftGame {
    pub game_code: String,
    pub game_name: String,
    pub player_count: i32,
}

#[tauri::command]
pub async fn list_public_draft_games(reqwest_client: State<'_, Client>) -> Result<Vec<DraftGame>, String> {
    let operation = PublicGamesQuery::build(());

    let response = reqwest_client.post(GRAPHQL_URL)
        .json(&operation)
        .send().await.unwrap()
        .json::<GraphQlResponse<PublicGamesQuery>>().await.unwrap();

    let data = response.data.unwrap().public_games;
    Ok(data)
}