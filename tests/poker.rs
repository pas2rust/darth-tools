use darth_tools::{Combination, DarthTools, Number, PokerTrait, Suit};

#[test]
fn test_poker_texas_holdem_deck() {
    let mut deck = DarthTools::new_poker_texas_holdem_deck();
    deck.shuffle();
    assert_eq!(deck.cards.len(), 52);
    let flop = deck.draw(3);
    let river = deck.draw(1);
    let showdown = deck.draw(1);
    assert_eq!(deck.cards.len(), 47);
    assert_eq!(flop.len(), 3);
    assert_eq!(river.len(), 1);
    assert_eq!(showdown.len(), 1);
}

#[test]
fn test_poker_texas_holdem_deck_shuffle() {
    let mut deck1 = DarthTools::new_poker_texas_holdem_deck();
    let deck2 = deck1.clone();
    deck1.shuffle();
    assert_ne!(deck1.cards, deck2.cards, "The cards should be in a different order after shuffling.");
}

#[test]
fn test_poker_texas_holdem_card_occurrence() {
    let mut counts = [0; 52];
    for _ in 0..1000 {
        let mut deck = DarthTools::new_poker_texas_holdem_deck();
        deck.shuffle();
        for card in deck.cards {
            let index = card.suit as usize * 13 + card.number.weight() as usize;
            counts[index] += 1;
        }
    }
    let avg = counts.iter().sum::<usize>() / counts.len();
    for &count in &counts {
        assert!((count as i32 - avg as i32).abs() < 100, "The card distribution is not random enough.");
    }
}

#[test]
fn test_best_possible_combination_texas_holdem_royal_flush() {
    let cards = [
        DarthTools::new_card(Suit::Spades, Number::Ace),
        DarthTools::new_card(Suit::Spades, Number::King),
        DarthTools::new_card(Suit::Spades, Number::Queen),
        DarthTools::new_card(Suit::Spades, Number::Jack),
        DarthTools::new_card(Suit::Spades, Number::Ten),
        DarthTools::new_card(Suit::Hearts, Number::Two),
        DarthTools::new_card(Suit::Clubs, Number::Three),
    ];
    assert_eq!(DarthTools::best_possible_combination_texas_holdem(&cards), Some(Combination::RoyalFlush));
}
#[test]
fn test_best_possible_combination_texas_holdem_straight_flush() {
    let cards = [
        DarthTools::new_card(Suit::Spades, Number::Nine),
        DarthTools::new_card(Suit::Spades, Number::Ten),
        DarthTools::new_card(Suit::Spades, Number::Jack),
        DarthTools::new_card(Suit::Spades, Number::Queen),
        DarthTools::new_card(Suit::Spades, Number::King),
        DarthTools::new_card(Suit::Hearts, Number::Two),
        DarthTools::new_card(Suit::Clubs, Number::Three),
    ];
    assert_eq!(
        DarthTools::best_possible_combination_texas_holdem(&cards),
        Some(Combination::StraightFlush(Number::King.weight()))
    );
}

#[test]
fn test_best_possible_combination_texas_holdem_four_of_a_kind() {
    let cards = [
        DarthTools::new_card(Suit::Spades, Number::Ace),
        DarthTools::new_card(Suit::Hearts, Number::Ace),
        DarthTools::new_card(Suit::Diamonds, Number::Ace),
        DarthTools::new_card(Suit::Clubs, Number::Ace),
        DarthTools::new_card(Suit::Spades, Number::Ten),
        DarthTools::new_card(Suit::Hearts, Number::Two),
        DarthTools::new_card(Suit::Clubs, Number::Three),
    ];
    assert_eq!(
        DarthTools::best_possible_combination_texas_holdem(&cards),
        Some(Combination::FourOfAKind(Number::Ace.weight()))
    );
}

#[test]
fn test_best_possible_combination_texas_holdem_full_house() {
    let cards = [
        DarthTools::new_card(Suit::Spades, Number::Ace),
        DarthTools::new_card(Suit::Hearts, Number::Ace),
        DarthTools::new_card(Suit::Diamonds, Number::Ace),
        DarthTools::new_card(Suit::Clubs, Number::King),
        DarthTools::new_card(Suit::Spades, Number::King),
        DarthTools::new_card(Suit::Hearts, Number::Two),
        DarthTools::new_card(Suit::Clubs, Number::Three),
    ];
    assert_eq!(
        DarthTools::best_possible_combination_texas_holdem(&cards),
        Some(Combination::FullHouse(Number::Ace.weight()))
    );
}

#[test]
fn test_best_possible_combination_texas_holdem_flush() {
    let cards = [
        DarthTools::new_card(Suit::Spades, Number::Ace),
        DarthTools::new_card(Suit::Spades, Number::King),
        DarthTools::new_card(Suit::Spades, Number::Queen),
        DarthTools::new_card(Suit::Spades, Number::Jack),
        DarthTools::new_card(Suit::Spades, Number::Nine),
        DarthTools::new_card(Suit::Hearts, Number::Two),
        DarthTools::new_card(Suit::Clubs, Number::Three),
    ];
    assert_eq!(
        DarthTools::best_possible_combination_texas_holdem(&cards),
        Some(Combination::Flush(Number::Ace.weight()))
    );
}

#[test]
fn test_best_possible_combination_texas_holdem_straight() {
    let cards = [
        DarthTools::new_card(Suit::Hearts, Number::Two),
        DarthTools::new_card(Suit::Diamonds, Number::Three),
        DarthTools::new_card(Suit::Clubs, Number::Four),
        DarthTools::new_card(Suit::Spades, Number::Five),
        DarthTools::new_card(Suit::Hearts, Number::Six),
        DarthTools::new_card(Suit::Diamonds, Number::King),
        DarthTools::new_card(Suit::Clubs, Number::Ace),
    ];
    assert_eq!(Combination::best_possible_combination_texas_holdem(&cards), Some(Combination::Straight(6)));
}

#[test]
fn test_best_possible_combination_texas_holdem_three_of_a_kind() {
    let cards = [
        DarthTools::new_card(Suit::Spades, Number::Ace),
        DarthTools::new_card(Suit::Hearts, Number::Ace),
        DarthTools::new_card(Suit::Diamonds, Number::Ace),
        DarthTools::new_card(Suit::Clubs, Number::King),
        DarthTools::new_card(Suit::Spades, Number::Queen),
        DarthTools::new_card(Suit::Hearts, Number::Two),
        DarthTools::new_card(Suit::Clubs, Number::Three),
    ];
    assert_eq!(
        DarthTools::best_possible_combination_texas_holdem(&cards),
        Some(Combination::ThreeOfAKind(Number::Ace.weight()))
    );
}

#[test]
fn test_best_possible_combination_texas_holdem_two_pair() {
    let cards = [
        DarthTools::new_card(Suit::Spades, Number::Ace),
        DarthTools::new_card(Suit::Hearts, Number::Ace),
        DarthTools::new_card(Suit::Diamonds, Number::King),
        DarthTools::new_card(Suit::Clubs, Number::King),
        DarthTools::new_card(Suit::Spades, Number::Queen),
        DarthTools::new_card(Suit::Hearts, Number::Two),
        DarthTools::new_card(Suit::Clubs, Number::Three),
    ];
    assert_eq!(
        DarthTools::best_possible_combination_texas_holdem(&cards),
        Some(Combination::TwoPair(Number::Ace.weight()))
    );
}

#[test]
fn test_best_possible_combination_texas_holdem_one_pair() {
    let cards = [
        DarthTools::new_card(Suit::Spades, Number::Ace),
        DarthTools::new_card(Suit::Hearts, Number::Ace),
        DarthTools::new_card(Suit::Diamonds, Number::King),
        DarthTools::new_card(Suit::Clubs, Number::Queen),
        DarthTools::new_card(Suit::Spades, Number::Jack),
        DarthTools::new_card(Suit::Hearts, Number::Two),
        DarthTools::new_card(Suit::Clubs, Number::Three),
    ];
    assert_eq!(
        DarthTools::best_possible_combination_texas_holdem(&cards),
        Some(Combination::OnePair(Number::Ace.weight()))
    );
}

#[test]
fn test_best_possible_combination_texas_holdem_high_card() {
    let cards = [
        DarthTools::new_card(Suit::Hearts, Number::Two),
        DarthTools::new_card(Suit::Diamonds, Number::Four),
        DarthTools::new_card(Suit::Clubs, Number::Six),
        DarthTools::new_card(Suit::Spades, Number::Eight),
        DarthTools::new_card(Suit::Hearts, Number::Ten),
        DarthTools::new_card(Suit::Diamonds, Number::King),
        DarthTools::new_card(Suit::Clubs, Number::Ace),
    ];
    assert_eq!(
        Combination::best_possible_combination_texas_holdem(&cards),
        Some(Combination::HighCard(Number::Ace.weight()))
    );
}
