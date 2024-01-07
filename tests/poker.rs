use std::collections::HashMap;

use darth_tools::{
    card::{Number, Suit},
    combination::Combination,
    poker::PokerTrait,
    DarthTools,
};

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
fn test_poker_holdem_high_card() {
    let cards = [
        DarthTools::new_poker_card(Suit::Clubs, Number::Two),
        DarthTools::new_poker_card(Suit::Hearts, Number::Four),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Six),
        DarthTools::new_poker_card(Suit::Spades, Number::Eight),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Ten),
        DarthTools::new_poker_card(Suit::Hearts, Number::Queen),
        DarthTools::new_poker_card(Suit::Hearts, Number::Ace),
    ];
    let combination =
        DarthTools::is_combination_holdem(cards.to_vec()).expect("is combination error");
    assert_eq!(
        combination,
        Combination::HighCard(
            [cards[6], cards[5], cards[4], cards[3], cards[2]],
            [cards[1], cards[0]]
        )
    )
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
    assert_eq!(
        combination,
        Combination::OnePair(
            [cards[0], cards[1], cards[6], cards[5], cards[4]],
            [cards[3], cards[2]]
        )
    )
}

#[test]
fn test_poker_holdem_two_pair() {
    let cards = [
        DarthTools::new_poker_card(Suit::Clubs, Number::Ace),
        DarthTools::new_poker_card(Suit::Hearts, Number::Ace),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Two),
        DarthTools::new_poker_card(Suit::Spades, Number::Six),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Eight),
        DarthTools::new_poker_card(Suit::Hearts, Number::Queen),
        DarthTools::new_poker_card(Suit::Hearts, Number::Queen),
    ];
    let combination =
        DarthTools::is_combination_holdem(cards.to_vec()).expect("is combination error");
    assert_eq!(
        combination,
        Combination::TwoPair(
            [cards[0], cards[1], cards[5], cards[6], cards[4]],
            [cards[3], cards[2]]
        )
    )
}

#[test]
fn test_poker_holdem_three_of_a_kind() {
    let cards = [
        DarthTools::new_poker_card(Suit::Clubs, Number::Ace),
        DarthTools::new_poker_card(Suit::Hearts, Number::Ace),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Ace),
        DarthTools::new_poker_card(Suit::Spades, Number::Six),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Eight),
        DarthTools::new_poker_card(Suit::Hearts, Number::Ten),
        DarthTools::new_poker_card(Suit::Hearts, Number::Queen),
    ];
    let combination =
        DarthTools::is_combination_holdem(cards.to_vec()).expect("is combination error");
    assert_eq!(
        combination,
        Combination::ThreeOfAKind(
            [cards[0], cards[1], cards[2], cards[6], cards[5]],
            [cards[4], cards[3]]
        )
    )
}

#[test]
fn test_poker_holdem_straight() {
    let cards = [
        DarthTools::new_poker_card(Suit::Clubs, Number::Two),
        DarthTools::new_poker_card(Suit::Hearts, Number::Three),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Four),
        DarthTools::new_poker_card(Suit::Spades, Number::Five),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Six),
        DarthTools::new_poker_card(Suit::Hearts, Number::Ten),
        DarthTools::new_poker_card(Suit::Hearts, Number::Queen),
    ];
    let combination =
        DarthTools::is_combination_holdem(cards.to_vec()).expect("is combination error");
    assert_eq!(
        combination,
        Combination::Straight([cards[4], cards[3], cards[2], cards[1], cards[0]])
    )
}

#[test]
fn test_poker_holdem_flush() {
    let cards = [
        DarthTools::new_poker_card(Suit::Hearts, Number::Two),
        DarthTools::new_poker_card(Suit::Hearts, Number::Four),
        DarthTools::new_poker_card(Suit::Hearts, Number::Six),
        DarthTools::new_poker_card(Suit::Hearts, Number::Eight),
        DarthTools::new_poker_card(Suit::Hearts, Number::Ten),
        DarthTools::new_poker_card(Suit::Hearts, Number::Queen),
        DarthTools::new_poker_card(Suit::Hearts, Number::Ace),
    ];
    let combination =
        DarthTools::is_combination_holdem(cards.to_vec()).expect("is combination error");
    assert_eq!(combination, Combination::Flush([cards[6], cards[5], cards[4], cards[3], cards[2]]))
}

#[test]
fn test_poker_holdem_full_house() {
    let cards = [
        DarthTools::new_poker_card(Suit::Clubs, Number::Ace),
        DarthTools::new_poker_card(Suit::Hearts, Number::Ace),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Ace),
        DarthTools::new_poker_card(Suit::Spades, Number::Six),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Six),
        DarthTools::new_poker_card(Suit::Hearts, Number::Six),
        DarthTools::new_poker_card(Suit::Hearts, Number::Queen),
    ];
    let combination =
        DarthTools::is_combination_holdem(cards.to_vec()).expect("is combination error");
    assert_eq!(
        combination,
        Combination::FullHouse([cards[0], cards[1], cards[2], cards[3], cards[4]])
    )
}

#[test]
fn test_poker_holdem_four_of_a_kind() {
    let cards = [
        DarthTools::new_poker_card(Suit::Clubs, Number::Ace),
        DarthTools::new_poker_card(Suit::Hearts, Number::Ace),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Ace),
        DarthTools::new_poker_card(Suit::Spades, Number::Ace),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Six),
        DarthTools::new_poker_card(Suit::Hearts, Number::Ten),
        DarthTools::new_poker_card(Suit::Hearts, Number::Queen),
    ];
    let combination =
        DarthTools::is_combination_holdem(cards.to_vec()).expect("is combination error");
    assert_eq!(
        combination,
        Combination::FourOfAKind([cards[0], cards[1], cards[2], cards[3], cards[4]],)
    )
}

#[test]
fn test_poker_holdem_royal_flush() {
    let cards = [
        DarthTools::new_poker_card(Suit::Hearts, Number::Ten),
        DarthTools::new_poker_card(Suit::Hearts, Number::Jack),
        DarthTools::new_poker_card(Suit::Hearts, Number::Queen),
        DarthTools::new_poker_card(Suit::Hearts, Number::King),
        DarthTools::new_poker_card(Suit::Hearts, Number::Ace),
        DarthTools::new_poker_card(Suit::Clubs, Number::Two),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Three),
    ];
    let combination =
        DarthTools::is_combination_holdem(cards.to_vec()).expect("is combination error");
    assert_eq!(
        combination,
        Combination::RoyalFlush([cards[4], cards[3], cards[2], cards[1], cards[0]])
    )
}
