use super::card::{Card, Suit, Weight};
use rand::Rng;
#[derive(Clone, Debug, PartialEq)]
pub struct Deck {
    pub cards: Vec<Card>,
}
impl Deck {
    pub fn new_texas_holdem() -> Self {
        let mut deck = Vec::new();
        let suits =
            [Suit::Hearts, Suit::Spades, Suit::Clubs, Suit::Diamonds];
        let values = [
            Weight::Ace,
            Weight::Two,
            Weight::Three,
            Weight::Four,
            Weight::Five,
            Weight::Six,
            Weight::Seven,
            Weight::Eight,
            Weight::Nine,
            Weight::Ten,
            Weight::Jack,
            Weight::Queen,
            Weight::King,
        ];
        suits.iter().for_each(|suit| {
            values.iter().for_each(|value| {
                deck.push(Card { weight: *value, suit: *suit });
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
    pub fn draw(&mut self, draws: u16) -> Vec<Card> {
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
