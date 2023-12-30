use crate::DarthTools;
use rand::Rng;

use super::{
    card::Card, combination::Combination, deck::Deck,
    position::Position,
};

pub trait PokerTrait {
    fn new_poker_texas_holdem_deck() -> Deck;
}

impl PokerTrait for DarthTools {
    fn new_poker_texas_holdem_deck() -> Deck {
        Deck::new_texas_holdem()
    }
}
