use std::collections::HashMap;
use tokio::sync::Mutex;
use sonyflake::Sonyflake;
use juniper::{
    Context,
    GraphQLScalar,
    GraphQLInputObject,
    InputValue,
    ScalarValue,
    ScalarToken,
    ParseScalarValue,
    ParseScalarResult,
    Value,
};

use crate::game::Game;

#[derive(GraphQLScalar, Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[graphql(with = Self)]
pub struct Id(pub u64);
impl Id {
    #[allow(clippy::wrong_self_convention)] /* shut up clippy, i dont make this rule */
    fn to_output<S: ScalarValue>(&self) -> Value<S> {
        format!("{}", self.0).into()
    }
    fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<Self, String> {
        let v = v.as_string_value().ok_or("missing id?".to_string())?;
        match v.parse::<u64>() {
            Ok(v) => Ok(Id(v)),
            Err(e) => Err(format!("{e:?}"))
        }
    }
    fn parse_token<S: ScalarValue>(value: ScalarToken<'_>) -> ParseScalarResult<S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
}

#[derive(GraphQLInputObject, Clone, Debug)]
pub struct Pair {
    pub first_index: i32,
    pub last_index: i32,
}

pub struct Database {
    pub snowflake: Sonyflake,
    pub game_states: Mutex<HashMap<Id, Game>>
}
impl Database {
    pub fn new() -> Self {
        Self {
            snowflake: Sonyflake::new().unwrap(),
            game_states: Mutex::new(HashMap::new()),
        }
    }
}
impl Context for Database {}
