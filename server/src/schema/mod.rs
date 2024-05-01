pub mod database;
pub mod query;
pub mod mutation;

use juniper::{RootNode, EmptySubscription};

use database::Database;
use query::Query;
use mutation::Mutation;

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Database>>;
