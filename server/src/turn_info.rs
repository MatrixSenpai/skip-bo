use juniper::{GraphQLEnum, GraphQLInputObject};
use std::fmt::Display;

use crate::game_models::identifier::Id;

#[derive(GraphQLEnum, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TurnPhase {
    Draw,
    Play,
    Discard,
}
impl Display for TurnPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = match self {
            Self::Draw => "Draw",
            Self::Play => "Play",
            Self::Discard => "Discard",
        };
        write!(f, "{p}")
    }
}

pub enum PlayAction {
    Build(PlayBuildAction),
    Discard(PlayDiscardAction),
}

#[derive(GraphQLInputObject)]
pub struct PlayDiscardAction {
    pub source: PlayDiscardActionSource,
    pub index: Option<i32>,
    pub destination: i32,
}
#[derive(GraphQLEnum, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlayDiscardActionSource {
    Hand,
    Stock,
}

#[derive(GraphQLInputObject)]
pub struct PlayBuildAction {
    pub player: PlayBuildPlayerSource,
    pub source: PlayBuildActionSource,
    pub index: Option<i32>,
    pub destination: i32,
}
#[derive(GraphQLEnum, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlayBuildActionSource {
    Stock,
    Discard,
    Hand,
}
#[derive(GraphQLEnum, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlayBuildPlayerSource {
    Own,
    Teammate,
}
