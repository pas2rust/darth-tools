use std::collections::HashMap;

use darth_tools::{
    card::{Number, Suit},
    combination::Combination,
    poker::PokerTrait,
    DarthTools,
};
use itertools::Itertools;

#[test]
fn test_poker_deck_new_holdem_card_unique_cards() {
    let mut deck = DarthTools::new_poker_deck_holdem();
    deck.shuffle();

    let mut card_counts = HashMap::new();
    for card in deck.cards.iter() {
        *card_counts.entry(card).or_insert(0) += 1;
    }

    for count in card_counts.values() {
        assert_eq!(*count, 1, "Each card should appear exactly once");
    }
}

#[test]
fn test_poker_deck_new_holdem_card_ocurrence() {
    let mut counts = HashMap::new();
    for _ in 0..1_000_000 {
        let mut deck = DarthTools::new_poker_deck_holdem();
        deck.shuffle();
        let card = deck.draw(1);
        *counts.entry(card).or_insert(0) += 1;
    }

    let avg = 1_000_000 / 52; // assumindo um baralho padrão de 52 cartas
    let tolerance = avg / 100 * 5; // 5% de tolerância

    for (card, count) in &counts {
        assert!(
            ((*count as f64) - (avg as f64)).abs() <= (tolerance as f64),
            "Card {:#?} occurred {} times, which is outside the tolerance",
            card,
            count
        );
    }
}

#[test]
fn test_poker_holdem_one_pair() {
    let cards = [
        DarthTools::new_poker_card(Suit::Clubs, Number::Ace),
        DarthTools::new_poker_card(Suit::Hearts, Number::Ace),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Two),
        DarthTools::new_poker_card(Suit::Spades, Number::Six),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Eight),
        DarthTools::new_poker_card(Suit::Hearts, Number::Ten),
        DarthTools::new_poker_card(Suit::Hearts, Number::Queen),
    ];
    let combination =
        DarthTools::is_combination_holdem(cards.to_vec()).expect("is combination error");
    assert_eq!(combination, Combination::OnePair(cards[0], cards[6], cards[5], cards[4]))
}

#[test]
fn test_poker_holdem_combination_ocurrence() {
    let mut counts = HashMap::new();
    for _ in 0..100_000 {
        let mut deck = DarthTools::new_poker_deck_holdem();
        deck.shuffle();
        let cards = deck.draw(7);
        let combination = DarthTools::is_combination_holdem(cards).expect("is combination error");
        *counts.entry(combination.force()).or_insert(0) += 1;
    }

    // Aqui você pode verificar a ocorrência de cada combinação
    for (force, count) in &counts {
        // Estes são apenas valores de exemplo. Você deve ajustá-los de acordo com as
        // probabilidades esperadas para cada combinação no Texas Hold'em.
        let (min_expected, max_expected) = match force {
            10000 => (31, 32),             // RoyalFlush: 0.031% (1 : 30940) [1]
            9000..=9999 => (28, 29),       // StraightFlush: 0.028% (1 : 3589.6) [1]
            8000..=8999 => (168, 169),     // FourOfAKind: 0.168% (1 : 594) [1]
            7000..=7999 => (2600, 2610),   // FullHouse: 2.60% (1 : 37.5) [1]
            6000..=6999 => (3030, 3040),   // Flush: 3.03% (1 : 32.1) [1]
            5000..=5999 => (4620, 4630),   // Straight: 4.62% (1 : 20.6) [1]
            4000..=4999 => (4830, 4840),   // ThreeOfAKind: 4.83% (1 : 19.7) [1]
            3000..=3999 => (23500, 23510), // TwoPair: 23.5% (1 : 3.26) [1]
            2000..=2999 => (43800, 43810), // OnePair: 43.8% (1 : 1.28) [1]
            1000..=1999 => (17400, 17410), // HighCard: 17.4% (1 : 4.74) [1]
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
