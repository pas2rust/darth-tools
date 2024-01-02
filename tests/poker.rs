use std::collections::HashMap;

use darth_tools::{poker::PokerTrait, DarthTools};

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
