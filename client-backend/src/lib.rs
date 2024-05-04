mod graphql;

const GRAPHQL_URL: &'static str = "http://127.0.0.1:3001/graphql";

#[cynic::schema("skip_bo")]
mod schema {}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let client = reqwest::Client::builder().build().unwrap();

    tauri::Builder::default()
        .manage(client)
        .invoke_handler(tauri::generate_handler![
            graphql::list_draft_games::list_public_draft_games,
            graphql::create_draft_game::create_draft_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
