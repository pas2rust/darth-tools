use crate::card::Card;
use darth_rust::DarthRust;
use serde::{Serialize, Deserialize};
use super::player::Players;


#[derive(Debug, Serialize, Deserialize)]
pub enum Phase {
    Thinking,
    Flop(Card, Card, Card),
    River(Card),
    Turn(Card)
}
#[derive(DarthRust, Debug, Serialize, Deserialize)]
pub struct Round {
    pub id: String,
    pub participants: Players,
    pub winners: Players,
    pub phase: Phase,
    pub pot: u128
}

impl Default for Phase {
    fn default() -> Self {
        Self::Thinking
    }
}

impl Default for Round {
    fn default() -> Self {
        Self { id: "".to_string(), participants: vec![], winners: vec![], phase: Phase::default(), pot: 0 }
    }
}