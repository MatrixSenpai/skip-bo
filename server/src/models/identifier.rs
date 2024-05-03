use juniper::{GraphQLScalar, InputValue, ParseScalarResult, ParseScalarValue, ScalarToken, ScalarValue, Value};
use serde::{Deserialize, Serialize};

#[derive(GraphQLScalar, Serialize, Deserialize, Hash, Default, Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
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
