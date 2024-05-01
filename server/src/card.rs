use juniper::{GraphQLObject, GraphQLEnum};
use std::fmt::Display;

#[derive(GraphQLEnum, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Wild,
}
impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = match self {
            Rank::Wild => "Wild".to_string(),
            _ => {
                let r: usize = <Rank as std::convert::Into<usize>>::into(*self);
                format!("{r}")
            },
        };
        write!(f, "{r}")
    }
}
impl From<usize> for Rank {
    fn from(val: usize) -> Self {
        match val {
            1 => Rank::One,
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Eleven,
            12 => Rank::Twelve,
            13 => Rank::Wild,

            _ => unreachable!(),
        }
    }
}
impl From<Rank> for usize {
    fn from(value: Rank) -> Self {
        match value {
            Rank::One    => 1,
            Rank::Two    => 2,
            Rank::Three  => 3,
            Rank::Four   => 4,
            Rank::Five   => 5,
            Rank::Six    => 6,
            Rank::Seven  => 7,
            Rank::Eight  => 8,
            Rank::Nine   => 9,
            Rank::Ten    => 10,
            Rank::Eleven => 11,
            Rank::Twelve => 12,
            Rank::Wild   => 13,
        }
    }
}

#[derive(GraphQLObject, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Card {
    pub rank: Rank, 
}
impl Card {
    pub fn new(rank: Rank) -> Self {
        Self { rank }
    }
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rank)
    }
}
