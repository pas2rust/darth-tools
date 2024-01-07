use darth_rust::DarthRust;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize, Default)]
pub enum Suit {
    Clubs,
    #[default]
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize, Default)]
pub enum Number {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    #[default]
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, DarthRust, Deserialize, Serialize)]
pub struct Card {
    pub suit: Suit,
    pub number: Number,
}

impl Number {
    pub fn value(&self) -> usize {
        *self as usize
    }
}
