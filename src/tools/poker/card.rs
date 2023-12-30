#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Weight {
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
impl Weight {
    pub fn weight(&self) -> u8 {
        match *self {
            Weight::Ace => 14,
            Weight::Two => 2,
            Weight::Three => 3,
            Weight::Four => 4,
            Weight::Five => 5,
            Weight::Six => 6,
            Weight::Seven => 7,
            Weight::Eight => 8,
            Weight::Nine => 9,
            Weight::Ten => 10,
            Weight::Jack => 11,
            Weight::Queen => 12,
            Weight::King => 13,
        }
    }
}
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Suit {
    Hearts,   // copas
    Spades,   // espadas
    Clubs,    // paus
    Diamonds, // ouros
}
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub weight: Weight,
}
