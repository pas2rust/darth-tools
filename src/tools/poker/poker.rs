use super::{
    super::darth_tools::DarthTools,
    card::{Card, Number, Suit},
    combination::Combination,
    deck::Deck,
};

pub trait PokerTrait {
    fn new_poker_deck_holdem() -> Deck;
    fn new_poker_card(suit: Suit, number: Number) -> Card;
    fn is_combination_holdem(cards: Vec<Card>) -> Result<Combination, String>;
}

impl PokerTrait for DarthTools {
    fn new_poker_deck_holdem() -> Deck {
        Deck::new_holdem()
    }
    fn new_poker_card(suit: Suit, number: Number) -> Card {
        Card::new(suit, number)
    }
    fn is_combination_holdem(cards: Vec<Card>) -> Result<Combination, String> {
        Combination::is(cards)
    }
}
