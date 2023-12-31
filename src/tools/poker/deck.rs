use super::card::{Card, Number, Suit};
use rand::Rng;
#[derive(Clone, Debug, PartialEq)]
pub struct Deck {
    pub cards: Vec<Card>,
}
impl Deck {
    pub fn new_texas_holdem() -> Self {
        let mut deck = Vec::new();
        let suits = [Suit::Hearts, Suit::Spades, Suit::Clubs, Suit::Diamonds];
        let values = [
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
        suits.iter().for_each(|suit| {
            values.iter().for_each(|value| {
                deck.push(Card { number: *value, suit: *suit });
            })
        });
        Self { cards: deck }
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for i in (1..self.cards.len()).rev() {
            let j = rng.gen_range(0..=i);
            self.cards.swap(i, j);
        }
    }
    pub fn draw(&mut self, draws: u8) -> Vec<Card> {
        let mut run = 0;
        let mut result: Vec<Card> = Vec::new();
        let mut rng = rand::thread_rng();
        loop {
            run += 1;
            let random = rng.gen_range(0..self.cards.len());
            let card = self.cards.remove(random);
            result.push(card);
            if run == draws {
                break;
            }
        }
        result
    }
}
