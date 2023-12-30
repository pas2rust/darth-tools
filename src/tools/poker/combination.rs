#[derive(Clone, Debug)]
pub enum Combination {
    HighCard,      // 1
    OnePair,       // 2
    TwoPair,       // 3
    ThreeOfAKind,  // 4
    Straight,      // 5
    Flush,         // 6
    FullHouse,     // 7
    FourOfAKind,   // 8
    StraightFlush, // 9
    RoyalFlush,    // 10
}

impl Combination {
    pub fn weight(&self) -> u8 {
        match *self {
            Combination::HighCard => 1,
            Combination::OnePair => 2,
            Combination::TwoPair => 3,
            Combination::ThreeOfAKind => 4,
            Combination::Straight => 5,
            Combination::Flush => 6,
            Combination::FullHouse => 7,
            Combination::FourOfAKind => 8,
            Combination::StraightFlush => 9,
            Combination::RoyalFlush => 10,
        }
    }
}
