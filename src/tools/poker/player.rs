use darth_rust::DarthRust;
use serde::{Serialize, Deserialize};


use crate::card::Card;

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Fold,
    Check,
    Call(u128),
    Rise(u128),
    AllIn(u128)
}

#[derive(DarthRust, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub hand: (Card, Card),
    pub action: Action,
    pub bankroll: u128,
    pub stack: u128
}

pub type Players = Vec<Player>;

impl Default for Card {
    fn default() -> Self {
      Self {
        number: crate::card::Number::Ace,
        suit: crate::card::Suit::Diamonds
      }
    }
}

impl Default for Action {
    fn default() -> Self {
       Self::Fold
    }
}
