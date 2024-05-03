use std::fmt::Display;

use rand::prelude::{SliceRandom, thread_rng};
use sonyflake::Sonyflake;
use juniper::graphql_object;

use crate::card::{Card, Rank};
use crate::game_models::identifier::Id;
use crate::turn_info::{PlayBuildActionSource, PlayDiscardActionSource};

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Player {
    pub id: Id,
    pub team: Id,

    pub hand: Vec<Card>,
    pub stock: Vec<Card>,

    pub discard_a: Vec<Card>,
    pub discard_b: Vec<Card>,
    pub discard_c: Vec<Card>,
    pub discard_d: Vec<Card>
}
impl Player {
    pub fn new(snowflake: &Sonyflake) -> Self {
        let id = Id(snowflake.next_id().unwrap());
        Self {
            id,
            team: id,
            ..Default::default()
        }
    }
    pub fn set_team(&mut self, other: Id) {
        self.team = Id(self.id.0 ^ other.0);
    }
    pub fn draw_size(&self) -> usize {
        5 - self.hand.len()
    }
    pub fn draw(&mut self, drawn_hand: Vec<Card>) {
        let mut inner_hand = drawn_hand;
        self.hand.append(&mut inner_hand);
    }
    pub fn teammate(&self) -> Id {
        Id(self.id.0 ^ self.team.0)
    }
    pub fn get_card(&mut self, source: PlayBuildActionSource, index: Option<i32>) -> Result<Card, String> {
        match source {
            PlayBuildActionSource::Stock => {
                let card = self.stock.remove(0);
                Ok(card)
            },
            _ => {
                let index: usize = index.ok_or("Index must be specified when source is not Draw".to_string())?
                    .try_into().map_err(|_| "Not a valid positive integer".to_string())?;

                if source == PlayBuildActionSource::Hand {
                    if index >= self.hand.len() {
                        return Err("Out of bounds card request for hand".to_string());
                    }

                    Ok(self.hand.remove(index))
                } else {
                    let mut pile = match index {
                        0 => &mut self.discard_a,
                        1 => &mut self.discard_b,
                        2 => &mut self.discard_c,
                        3 => &mut self.discard_d,
                        _ => return Err("Out of bounds card request for discard".to_string()),
                    };

                    if pile.is_empty() {
                        return Err("Requested an empty discard pile".to_string());
                    }
                    Ok(pile.remove(0))
                }
            }
        }
    }
    pub fn discard(&mut self, source: PlayDiscardActionSource, index: Option<i32>, destination: i32) -> Result<(), String> {
        let destination_stack = match destination {
            0 => &mut self.discard_a,
            1 => &mut self.discard_b,
            2 => &mut self.discard_c,
            3 => &mut self.discard_d,
            _ => return Err("Out of bounds discard index".to_string()),
        };

        let source_card = match source {
            PlayDiscardActionSource::Stock => self.stock.remove(0),
            PlayDiscardActionSource::Hand => {
                let index: usize = index.ok_or("Must include a hand index to discard".to_string())?
                    .try_into().map_err(|_| "Not a valid positive integer".to_string())?;

                if index < self.hand.len() {
                    self.hand.remove(index)
                } else {
                    return Err("Out of bounds hand source index".to_string());
                }
            }
        };

        destination_stack.push(source_card);

        Ok(())
    }
}
#[graphql_object]
impl Player {
    pub fn id(&self) -> Id { self.id }
    pub fn team(&self) -> Id { self.team }
    pub fn discard_a(&self) -> Vec<Card> { self.discard_a.clone() }
    pub fn discard_b(&self) -> Vec<Card> { self.discard_b.clone() }
    pub fn discard_c(&self) -> Vec<Card> { self.discard_c.clone() }
    pub fn discard_d(&self) -> Vec<Card> { self.discard_d.clone() }

    pub fn stock_first(&self) -> Option<&Card> { self.stock.first() }

    // enabling for debug purposes but will need to be restricted eventually
    pub fn hand(&self) -> Vec<Card> { self.hand.clone() }
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Player ID: {} / Team ID: {}", self.id.0, self.team.0)?;
        writeln!(f, "Hand: {}", self.hand.iter().map(Card::to_string).collect::<Vec<String>>().join(" / "))?;
        writeln!(f, "Stock: {}", self.stock.iter().map(Card::to_string).collect::<Vec<String>>().join(" / "))?;

        writeln!(f, "Discard A: {}", self.discard_a.iter().map(Card::to_string).collect::<Vec<String>>().join(" / "))?;
        writeln!(f, "Discard B: {}", self.discard_b.iter().map(Card::to_string).collect::<Vec<String>>().join(" / "))?;
        writeln!(f, "Discard C: {}", self.discard_c.iter().map(Card::to_string).collect::<Vec<String>>().join(" / "))?;
        writeln!(f, "Discard D: {}", self.discard_d.iter().map(Card::to_string).collect::<Vec<String>>().join(" / "))?;

        Ok(())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dealer {
    pub draw: Vec<Card>,
    pub discard: Vec<Card>,

    pub building_a: Vec<Card>,
    pub building_b: Vec<Card>,
    pub building_c: Vec<Card>,
    pub building_d: Vec<Card>
}
impl Dealer {
    pub fn new(draw: Vec<Card>) -> Self {
        Self {
            draw,
            ..Default::default()
        }
    }
    pub fn draw(&mut self, draw_size: usize) -> Vec<Card> {
        if draw_size > self.draw.len() {
            self.shuffle_discard();
        }

        self.draw.drain(0..draw_size).collect()
    }
    fn shuffle_discard(&mut self) {
        self.discard.shuffle(&mut thread_rng());
        self.draw.append(&mut self.discard);
    }
    pub fn place_card(&mut self, card: Card, pile: i32) -> Result<(), String> {
        let mut card_pile = match pile {
            0 => &mut self.building_a,
            1 => &mut self.building_b,
            2 => &mut self.building_c,
            3 => &mut self.building_d,
            _ => return Err("Out of bounds placement for destination of card".to_string()),
        };
        let next_rank = (card_pile.len() + 1).into();

        info!("Attempting to play card {card}. Next expected rank: {next_rank}");
        let is_same_rank = card.rank == next_rank;
        let is_wild = card.rank == Rank::Wild;
        if !is_same_rank && !is_wild {
            return Err("That card cannot be placed on the selected build pile".to_string());
        }

        card_pile.push(card);

        if card_pile.len() >= 12 {
            self.discard.append(card_pile);
        }

        Ok(())
    }
}

#[graphql_object]
impl Dealer {
    pub fn draw_first(&self) -> Option<&Card> { self.draw.first() }
    pub fn discard_first(&self) -> Option<&Card> { self.discard.first() }

    pub fn building_a(&self) -> Option<&Card> { self.building_a.first() }
    pub fn building_b(&self) -> Option<&Card> { self.building_b.first() }
    pub fn building_c(&self) -> Option<&Card> { self.building_c.first() }
    pub fn building_d(&self) -> Option<&Card> { self.building_d.first() }
}

impl Display for Dealer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Draw: {}", self.draw.iter().map(Card::to_string).collect::<Vec<String>>().join(" / "))?;
        writeln!(f, "Discard: {}", self.discard.iter().map(Card::to_string).collect::<Vec<String>>().join(" / "))?;

        writeln!(f, "Building A: {}", self.building_a.iter().map(Card::to_string).collect::<Vec<String>>().join(" / "))?;
        writeln!(f, "Building B: {}", self.building_b.iter().map(Card::to_string).collect::<Vec<String>>().join(" / "))?;
        writeln!(f, "Building C: {}", self.building_c.iter().map(Card::to_string).collect::<Vec<String>>().join(" / "))?;
        writeln!(f, "Building D: {}", self.building_d.iter().map(Card::to_string).collect::<Vec<String>>().join(" / "))?;

        Ok(())
    }
}
