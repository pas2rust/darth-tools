use super::card::{Card, Number, Suit};
use rand::{seq::SliceRandom, thread_rng};
use darth_rust::DarthRust;
use serde::{Serialize, Deserialize};

pub type Cards = Vec<Card>;

#[derive(Debug, DarthRust, Deserialize, Serialize)]
pub struct Deck {
    pub cards: Cards,
}

impl Deck {
    pub fn new_holdem() -> Self {
        let mut cards = [Card::new(Suit::Clubs, Number::Ace); 52];
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let numbers = [
            Number::Ace,
            Number::Two,
            Number::Three,
            Number::Four,
            Number::Five,
            Number::Six,
            Number::Seven,
            Number::Eight,
            Number::Nine,
            Number::Ten,
            Number::Jack,
            Number::Queen,
            Number::King,
        ];

        let mut card_index = 0;
        for suit in &suits {
            for number in &numbers {
                cards[card_index] = Card::new(*suit, *number);
                card_index += 1;
                if card_index >= 52 {
                    break;
                }
            }
            if card_index >= 52 {
                break;
            }
        }

        Self { cards: cards.to_vec() }
    }
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    pub fn draw(&mut self, num: usize) -> Vec<Card> {
        let mut drawn_cards = Vec::new();
        for _ in 0..num {
            if let Some(card) = self.cards.pop() {
                drawn_cards.push(card);
            } else {
                break;
            }
        }
        drawn_cards
    }
}
