#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    Ace,   // Ás
    Two,   // Dois
    Three, // Três
    Four,  // Quatro
    Five,  // Cinco
    Six,   // Seis
    Seven, // Sete
    Eight, // Oito
    Nine,  // Nove
    Ten,   // Dez
    Jack,  // Valete
    Queen, // Dama
    King,  // Rei
}
impl Number {
    pub fn weight(&self) -> u8 {
        match *self {
            Number::Ace => 14,
            Number::Two => 2,
            Number::Three => 3,
            Number::Four => 4,
            Number::Five => 5,
            Number::Six => 6,
            Number::Seven => 7,
            Number::Eight => 8,
            Number::Nine => 9,
            Number::Ten => 10,
            Number::Jack => 11,
            Number::Queen => 12,
            Number::King => 13,
        }
    }
}
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Suit {
    Hearts,   // copas
    Spades,   // espadas
    Clubs,    // paus
    Diamonds, // ouros
}
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    pub suit: Suit,
    pub number: Number,
}
