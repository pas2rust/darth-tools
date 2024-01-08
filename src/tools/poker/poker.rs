use crate::{UuidTrait, tools::poker::round::Phase};

use super::{
    super::darth_tools::DarthTools,
    card::{Card, Number, Suit},
    combination::Combination,
    deck::Deck, round::Round, player::Players,
};

pub trait PokerTrait {
    fn new_poker_deck_holdem() -> Deck;
    fn new_poker_card(suit: Suit, number: Number) -> Card;
    fn is_combination_holdem(cards: Vec<Card>) -> Result<Combination, String>;
    fn new_poker_round(participants: Players, pot: u128) -> Round;
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
    fn new_poker_round(participants: Players, pot: u128) -> Round {
        let id = DarthTools::new_uuid();
        Round {
            id, participants, winners: vec![], phase: Phase::Thinking, pot
        }
    }
}
