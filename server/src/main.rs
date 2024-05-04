use actix_web::{*, web::*, http::header::*};
use mongodb::options::ClientOptions;

#[macro_use]
extern crate log;

mod schema;
mod turn_info;
mod database;
mod models;

use schema::{
    Schema, Query, Mutation,
};
use database::Database;
use crate::database::MainContext;

async fn graphql(
    req: HttpRequest,
    payload: Payload,
    schema: Data<Schema>,
    db: Data<Database>,
) -> Result<HttpResponse, Error> {
    let context = MainContext(db, req.headers().clone());

    let mut response = juniper_actix::graphql_handler(
        &schema,
        &context,
        req,
        payload,
    ).await?;

    response.headers_mut().insert(HeaderName::from_static("x-cow"), HeaderValue::from_static("moo"));

    Ok(response)
}
async fn playground() -> Result<HttpResponse, Error> {
    juniper_actix::playground_handler("/graphql", None).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(
                Schema::new(Query, Mutation, juniper::EmptySubscription::new())
            ))
            .app_data(Data::new(
                Database::new(db_options.clone())
            ))
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![
                        ACCEPT,
                        CONTENT_TYPE,
                        HeaderName::from_static("x-cmac"),
                        HeaderName::from_static("x-current-player"),
                        HeaderName::from_static("x-current-lobby"),
                    ])
                    .max_age(64000)
            )
            .wrap(middleware::Compress::default())
            .service(
                resource("/graphql")
                    .route(post().to(graphql))
                    .route(get().to(graphql))
            )
            .default_service(to(playground))
    })
    .bind("127.0.0.1:3001")?
    .run()
    .await
}
