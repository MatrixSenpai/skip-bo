use actix_web::{*, web::*, http::header::*};

#[macro_use]
extern crate log;

mod card;
mod player;
mod game;
mod schema;
mod turn_info;

use schema::{
    Schema,
    database::Database,
    query::Query,
    mutation::Mutation,
};

async fn graphql(
    req: HttpRequest,
    payload: Payload,
    schema: Data<Schema>,
    db: Data<Database>,
) -> Result<HttpResponse, Error> {
    juniper_actix::graphql_handler(
        &schema,
        &db,
        req,
        payload,
    ).await
}
async fn playground() -> Result<HttpResponse, Error> {
    juniper_actix::playground_handler("/graphql", None).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(
                Schema::new(Query, Mutation, juniper::EmptySubscription::new())
            ))
            .app_data(Data::new(Database::new()))
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![
                        ACCEPT,
                        CONTENT_TYPE,
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
