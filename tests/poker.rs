use darth_tools::{DarthTools, PokerTrait};

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
    assert_ne!(
        deck1.cards, deck2.cards,
        "The cards should be in a different order after shuffling."
    );
}

#[test]
fn test_poker_texas_holdem_card_occurrence() {
    let mut counts = [0; 52];
    for _ in 0..1000 {
        let mut deck = DarthTools::new_poker_texas_holdem_deck();
        deck.shuffle();
        for card in deck.cards {
            let index =
                card.suit as usize * 13 + card.weight as usize;
            counts[index] += 1;
        }
    }
    let avg = counts.iter().sum::<usize>() / counts.len();
    for &count in &counts {
        assert!(
            (count as i32 - avg as i32).abs() < 100,
            "The card distribution is not random enough."
        );
    }
}
