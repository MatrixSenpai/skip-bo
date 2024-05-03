mod query;
mod mutation;

use juniper::{RootNode, EmptySubscription, EmptyMutation};

use super::database::MainContext;

pub use query::Query;
pub use mutation::Mutation;
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<MainContext>>;