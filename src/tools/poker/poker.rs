use crate::{DarthTools, Number, Suit};

use super::{card::Card, combination::Combination, deck::Deck};

pub trait PokerTrait {
    fn new_poker_texas_holdem_deck() -> Deck;
    fn new_card(suit: Suit, number: Number) -> Card;
    fn best_possible_combination_texas_holdem(cards: &[Card; 7]) -> Option<Combination>;
}

impl PokerTrait for DarthTools {
    fn new_poker_texas_holdem_deck() -> Deck {
        Deck::new_texas_holdem()
    }
    fn new_card(suit: Suit, number: Number) -> Card {
        Card { suit, number }
    }
    fn best_possible_combination_texas_holdem(cards: &[Card; 7]) -> Option<Combination> {
        Combination::best_possible_combination_texas_holdem(cards)
    }
}
