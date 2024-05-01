use std::{collections::HashMap, fmt::Display};
use juniper::{GraphQLEnum, GraphQLObject, graphql_object};

use rand::{thread_rng, seq::SliceRandom};
use sonyflake::Sonyflake;
use crate::{
    player::*,
    card::*,
    turn_info::*,
    schema::database::{Id, Database},
};

#[derive(Clone, Debug)]
pub struct Game {
    pub game_id: u64,

    pub phase: TurnPhase,
    pub current_player: u64,
    pub player_order: Vec<u64>,

    pub dealer: Dealer,
    pub players: HashMap<u64, Player>,
}
impl Game {
    pub fn new(
        num_players: usize, 
        pairs: Option<Vec<(usize, usize)>>, 
        snowflake: &Sonyflake
    ) -> Self {
        let game_id = snowflake.next_id().unwrap();

        let (card_count, wild_count) = if num_players <= 6 {
            (12, 18)
        } else {
            (
                (12 + (4 * (num_players - 6))),
                (18 + (3 * (num_players - 6)))
            )
        };

        let mut card_vec: Vec<Card> = Vec::new();
        let initial_card_array = core::array::from_fn::<_, 13, _>(|i| i + 1);
        for card in initial_card_array {
            let inner_card_count = if card == 13 { wild_count } else { card_count };

            let rank: Rank = card.into();
            for _ in 0..inner_card_count {
                card_vec.push(Card::new(rank));
            }

            debug!("{inner_card_count} cards created for {rank}");
        }

        card_vec.shuffle(&mut thread_rng());

        let mut players = Vec::new();
        for _ in 0..num_players {
            players.push(Player::new(snowflake));
        }

        if let Some(pairs) = pairs {
            for (l, r) in pairs.into_iter() {
                if l == r {
                    let mut player = players.get_mut(l).unwrap();
                    player.set_team(player.id);
                } else {
                    unsafe {
                        // Note this is unsafe because we are creating multiple mutiple borrows
                        // to pointers inside a vec, which IS legal so long as the indicies do
                        // not match, which we are ensuring here
                        let mut l_player = players.get_mut(l).unwrap() as *mut Player;
                        let mut r_player = players.get_mut(r).unwrap() as *mut Player;

                        (*l_player).set_team((*r_player).id);
                        (*r_player).set_team((*l_player).id);
                    }
                }
            }
        }

        for _ in 0..20  {
            for player in players.iter_mut() {
                let card = card_vec.pop().unwrap();
                player.stock.push(card);
            }
        }

        let player_order = players.iter().map(|p| p.id.0).collect::<Vec<_>>();
        let first_player_id = player_order.first().unwrap().clone();
        let players = players.into_iter()
            .map(|p| (p.id.0, p))
            .collect();

        Self {
            players, game_id, player_order,
            phase: TurnPhase::Draw,
            current_player: first_player_id,
            dealer: Dealer::new(card_vec),
        }
    }

    fn validate_game_state(&self, intended_player: Id, intended_action: TurnPhase) -> Result<(), String> {
        if self.phase != intended_action {
            error!("requested action out of sync, bad client state? intention Draw vs phase {:?}", self.phase);
            return Err(format!("Intended action Draw is not in sync with current game phase {:?}", self.phase));
        }
        if intended_player.0 != self.current_player {
            error!("requested player out of sync, bad client state? player {} vs current player {}", intended_player.0, self.current_player);
            return Err(format!("Intended player {} is not in sync with current player {}", intended_player.0, self.current_player));
        }
        Ok(())
    }

    pub fn handle_draw(&mut self, intended_player: Id) -> Result<&Player, String> {
        self.validate_game_state(intended_player, TurnPhase::Draw)?;

        let mut current_player = self.players.get_mut(&self.current_player).unwrap();

        let num_cards = current_player.draw_size();
        let cards = self.dealer.draw(num_cards);
        current_player.draw(cards);

        self.phase = TurnPhase::Play;

        let return_player = self.players.get(&self.current_player).unwrap();
        Ok(return_player)
    }

    pub fn handle_build(&mut self, intended_player: Id, action: PlayBuildAction) -> Result<(), String> {
        self.validate_game_state(intended_player, TurnPhase::Play)?;

        let target_player_id = match action.player {
            PlayBuildPlayerSource::Own => self.current_player,
            PlayBuildPlayerSource::Teammate => {
                let current_player = self.players.get(&self.current_player).unwrap();
                current_player.teammate().0
            }
        };
        let mut target_player = self.players.get_mut(&target_player_id).unwrap();
        let card = target_player.get_card(action.source, action.index)?;
        match self.dealer.place_card(card, action.destination) {
            Ok(_) => (),
            Err(e) => {
                error!("Tried to place an illegal card: {card}");
                target_player.draw(vec![card]);
                return Err(e);
            }
        };

        if target_player.id.0 == self.current_player && target_player.hand.is_empty() {
            let cards = self.dealer.draw(5);
            target_player.draw(cards);
        }

        Ok(())
    }

    pub fn handle_discard(&mut self, intended_player: Id, action: PlayDiscardAction) -> Result<(), String> {
        self.validate_game_state(intended_player, TurnPhase::Play)?;

        let mut current_player = self.players.get_mut(&self.current_player).unwrap();
        current_player.discard(action.source, action.index, action.destination)?;

        self.phase = TurnPhase::Draw;
        let next_player_index = self.player_order.iter().position(|i| i == &self.current_player).unwrap() + 1;
        let next_player_index = if next_player_index < self.player_order.len() {
            next_player_index
        } else { 0 };

        self.current_player = self.player_order[next_player_index];
        info!("moved to next player {}", self.current_player);

        Ok(())
    }
}

#[graphql_object(context = Database)]
impl Game {
    fn id(&self) -> Id {
        Id(self.game_id)
    }
    fn current_phase(&self) -> TurnPhase {
        self.phase
    }
    fn dealer(&self) -> Dealer {
        self.dealer.clone()
    }
    fn players(&self) -> Vec<Player> {
        self.players.clone().into_values().collect()
    }
}
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Game ({}) - {} phase for {}", self.game_id, self.phase, self.current_player)?;
        write!(f, "{}", self.dealer)?;

        for (_, player) in self.players.iter() {
            write!(f, "{player}")?;
        }
        
        Ok(())
    }
}
