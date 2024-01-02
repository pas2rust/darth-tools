use std::collections::HashMap;

use darth_tools::{poker::PokerTrait, DarthTools};

fn main() {
    let mut counts = HashMap::new();
    for _ in 0..100_000_000 {
        let mut deck = DarthTools::new_poker_deck_holdem();
        deck.shuffle();
        let cards = deck.draw(7);
        let combination = DarthTools::is_combination_holdem(cards).expect("is combination error");
        *counts.entry(combination.force()).or_insert(0) += 1;
    }

    // Aqui você pode verificar a ocorrência de cada combinação
    for (force, count) in &counts {
        let (min_expected, max_expected) = match force {
            10000 => (3200, 3300),               // RoyalFlush: 0.0032% (1 : 30940)
            9000..=9999 => (27900, 28000),       // StraightFlush: 0.0279% (1 : 3589.6)
            8000..=8999 => (168000, 169000),     // FourOfAKind: 0.168% (1 : 594)
            7000..=7999 => (2600000, 2610000),   // FullHouse: 2.60% (1 : 37.5)
            6000..=6999 => (3030000, 3040000),   // Flush: 3.03% (1 : 32.1)
            5000..=5999 => (4620000, 4630000),   // Straight: 4.62% (1 : 20.6)
            4000..=4999 => (4830000, 4840000),   // ThreeOfAKind: 4.83% (1 : 19.7)
            3000..=3999 => (23500000, 23510000), // TwoPair: 23.5% (1 : 3.26)
            2000..=2999 => (43800000, 43810000), // OnePair: 43.8% (1 : 1.28)
            1000..=1999 => (17400000, 17410000), // HighCard: 17.4% (1 : 4.74)
            _ => (0, 0),
        };

        assert!(
            *count >= min_expected && *count <= max_expected,
            "Combination with force {:#?} occurred {} times, which is outside the expected range.",
            force,
            count
        );
    }
}
