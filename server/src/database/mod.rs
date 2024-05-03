mod mongodb_keys;

use actix_web::http::header::HeaderMap;
use actix_web::web::Data;
use juniper::Context;
use sonyflake::Sonyflake;
use mongodb::{
    Client, Collection,
    options::ClientOptions,
};

pub use mongodb_keys::*;

pub struct MainContext(pub Data<Database>, pub HeaderMap);
impl Context for MainContext {}

pub struct Database {
    pub id_generator: Sonyflake,
    internal_connection: Client,
}
impl Database {
    pub fn new(database_options: ClientOptions) -> Self {
        Self {
            id_generator: Sonyflake::new().unwrap(),
            internal_connection: Client::with_options(database_options).unwrap(),
        }
    }

    pub fn get_collection<T>(&self, collection_key: MongodbKey) -> Collection<T> {
       let db = self.internal_connection.database(&*MongodbKey::MainDatabase.to_string());
        db.collection::<T>(&*collection_key.to_string())
    }
}