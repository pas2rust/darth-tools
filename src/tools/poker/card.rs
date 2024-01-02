#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Number {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub number: Number,
}

impl Number {
    pub fn value(&self) -> usize {
        *self as usize
    }
}

impl Card {
    pub fn new(suit: Suit, number: Number) -> Self {
        Self { suit, number }
    }
    pub fn set_number(&mut self, new: Number) {
        self.number = new;
    }
}
