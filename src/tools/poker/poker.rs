use super::{
    super::darth_tools::DarthTools,
    card::{Card, Number, Suit},
    deck::Deck,
};

pub trait PokerTrait {
    fn new_poker_deck_holdem() -> Deck;
    fn new_poker_card(suit: Suit, number: Number) -> Card;
}

impl PokerTrait for DarthTools {
    fn new_poker_deck_holdem() -> Deck {
        Deck::new_holdem()
    }
    fn new_poker_card(suit: Suit, number: Number) -> Card {
        Card::new(suit, number)
    }
}
