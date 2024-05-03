use super::Mutation;
use juniper::graphql_object;
use crate::database::MainContext;
use crate::models::graphql::client_draft_game::{CreateDraftGameRequest, CreateDraftGameResponse, UpdateDraftGameRequest};
use crate::models::identifier::Id;

#[graphql_object]
impl Mutation {
    async fn create_draft_game(&self, ctx: &MainContext, data: CreateDraftGameRequest) -> Result<CreateDraftGameResponse, String> {
        todo!()
    }
    async fn update_draft_game(&self, ctx: &MainContext, data: UpdateDraftGameRequest) -> Result<(), String> {
        todo!()
    }
    async fn join_draft_team(&self, ctx: &MainContext, player_id: Id) -> Result<(), String> {
        todo!()
    }
}