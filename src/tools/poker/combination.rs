use std::collections::HashMap;

use itertools::Itertools;

use super::card::{Card, Number};

type Combo = [Card; 5];
type TwoKickers = [Card; 2];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Combination {
    RoyalFlush(Combo),               // A Coroa Real
    StraightFlush(Combo),            // Escada Real
    FourOfAKind(Combo, TwoKickers),  // Quadra ou Poker
    FullHouse(Combo),                // Full ou Casa Cheia
    Flush(Combo),                    // Flush ou Cor
    Straight(Combo),                 // Sequência ou Escada
    ThreeOfAKind(Combo, TwoKickers), // Trinca
    TwoPair(Combo, TwoKickers),      // Dois Pares
    OnePair(Combo, TwoKickers),      // Um Par
    HighCard(Combo, TwoKickers),     // Carta Alta
}

fn sort_in_descending_order(cards: &mut Vec<Card>) {
    cards.sort_by(|a, b| b.number.value().cmp(&a.number.value()));
}

fn sort_in_ascending_order(cards: &mut Vec<Card>) {
    cards.sort_by(|a, b| a.number.value().cmp(&b.number.value()));
}

impl Combination {
    pub fn force(&self) -> usize {
        match *self {
            Self::RoyalFlush(cards) => 10000,
            Self::StraightFlush(cards) => {
                9000 + cards.iter().map(|c| c.number.value()).sum::<usize>()
            }
            Self::FourOfAKind(cards, kickers) => {
                8000 + cards.iter().map(|c| c.number.value()).sum::<usize>()
            }
            Self::FullHouse(cards) => 7000 + cards.iter().map(|c| c.number.value()).sum::<usize>(),
            Self::Flush(cards) => 6000 + cards.iter().map(|c| c.number.value()).sum::<usize>(),
            Self::Straight(cards) => 5000 + cards.iter().map(|c| c.number.value()).sum::<usize>(),
            Self::ThreeOfAKind(cards, kickers) => {
                4000 + cards.iter().map(|c| c.number.value()).sum::<usize>()
                    + kickers.iter().map(|c| c.number.value()).sum::<usize>()
            }
            Self::TwoPair(cards, kickers) => {
                3000 + cards.iter().map(|c| c.number.value()).sum::<usize>()
                    + kickers.iter().map(|c| c.number.value()).sum::<usize>()
            }
            Self::OnePair(cards, kickers) => {
                2000 + cards.iter().map(|c| c.number.value()).sum::<usize>()
                    + kickers.iter().map(|c| c.number.value()).sum::<usize>()
            }
            Self::HighCard(cards, kickers) => {
                1000 + cards.iter().map(|c| c.number.value()).sum::<usize>()
                    + kickers.iter().map(|c| c.number.value()).sum::<usize>()
            }
        }
    }

    pub fn to_str(&self) -> &str {
        match *self {
            Combination::RoyalFlush(_) => "Royal Flush",
            Combination::StraightFlush(_) => "Straight Flush",
            Combination::FourOfAKind(_, _) => "Four of a Kind",
            Combination::FullHouse(_) => "Full House",
            Combination::Flush(_) => "Flush",
            Combination::Straight(_) => "Straight",
            Combination::ThreeOfAKind(_, _) => "Three of a Kind",
            Combination::TwoPair(_, _) => "Two Pair",
            Combination::OnePair(_, _) => "One Pair",
            Combination::HighCard(_, _) => "High Card",
        }
    }

    pub fn find_high_card_number(cards: &Vec<Card>) -> Card {
        *cards.iter().max_by_key(|card| card.number.value()).unwrap()
    }
    pub fn is_royal_flush(cards: &mut Vec<Card>) -> Option<Combination> {
        sort_in_descending_order(cards);
        for window in cards.windows(5) {
            if Self::is_sequence(window)
                && window.windows(2).all(|pair| pair[0].suit == pair[1].suit)
                && window[0].number.value() == 14 // Ace
                && window[4].number.value() == 10
            // Ten
            {
                return Some(Combination::RoyalFlush([
                    window[0], window[1], window[2], window[3], window[4],
                ]));
            }
        }
        None
    }

    pub fn is_pair(cards: &mut Vec<Card>) -> Option<Combination> {
        let mut pairs: Vec<Card> = Vec::new();
        for card in cards.iter() {
            let card_number_value = card.number.value();
            let cards: Vec<Card> = cards
                .iter()
                .cloned()
                .filter(|card| card.number.value() == card_number_value)
                .collect();
            if cards.len() == 2 {
                pairs = cards
            }
        }
        if pairs.len() == 2 {
            let pair1 = pairs[0];
            let pair2 = pairs[1];
            cards.retain(|x| !pairs.contains(x));
            let kicker1 = cards[0];
            let kicker2 = cards[1];
            let kicker3 = cards[2];
            let kicker4 = cards[3];
            let kicker5 = cards[4];
            Some(Combination::OnePair(
                [pair1, pair2, kicker1, kicker2, kicker3],
                [kicker4, kicker5],
            ))
        } else {
            None
        }
    }

    pub fn is_two_pair(cards: &mut Vec<Card>) -> Option<Combination> {
        let mut pairs: Vec<Card> = Vec::new();
        let mut last_card_number = 0;
        for card in cards.iter() {
            let card_number_value = card.number.value();
            let cards: Vec<Card> = cards
                .iter()
                .cloned()
                .filter(|card| card.number.value() == card_number_value)
                .collect();
            if cards.len() == 2 && last_card_number != card_number_value {
                last_card_number = card_number_value;
                pairs.extend(cards)
            }
        }
        if pairs.len() == 4 {
            let pair1 = pairs[0];
            let pair2 = pairs[1];
            let pair3 = pairs[2];
            let pair4 = pairs[3];
            cards.retain(|x| !pairs.contains(x));
            let kicker1 = cards[0];
            let kicker2 = cards[1];
            let kicker3 = cards[2];
            Some(Combination::TwoPair([pair1, pair2, pair3, pair4, kicker1], [kicker2, kicker3]))
        } else {
            None
        }
    }

    pub fn is_three_of_a_kind(cards: &mut Vec<Card>) -> Option<Combination> {
        let mut counts = HashMap::new();
        for card in cards.iter() {
            let count = counts.entry(card.number.value()).or_insert(0);
            *count += 1;
        }
        let three_of_a_kind_card = counts.iter().find(|&(_, &v)| v == 3);
        if let Some((&number, _)) = three_of_a_kind_card {
            let three: Vec<Card> =
                cards.iter().cloned().filter(|card| card.number.value() == number).collect();
            cards.retain(|x| x.number.value() != number);
            let kicker1 = cards[0];
            let kicker2 = cards[1];
            let kicker3 = cards[2];
            let kicker4 = cards[3];
            return Some(Combination::ThreeOfAKind(
                [three[0], three[1], three[2], kicker1, kicker2],
                [kicker3, kicker4],
            ));
        }
        None
    }

    pub fn is_high_card(cards: &mut Vec<Card>) -> Option<Combination> {
        let high_card = cards[0];
        let kicker1 = cards[1];
        let kicker2 = cards[2];
        let kicker3 = cards[3];
        let kicker4 = cards[4];
        let kicker5 = cards[5];
        let kicker6 = cards[6];
        Some(Combination::HighCard(
            [high_card, kicker1, kicker2, kicker3, kicker4],
            [kicker5, kicker6],
        ))
    }

    pub fn is_four_of_a_kind(cards: &mut Vec<Card>) -> Option<Combination> {
        let mut four: Vec<Card> = Vec::new();
        for card in cards.iter() {
            let card_number_value = card.number.value();
            let cards: Vec<Card> = cards
                .iter()
                .cloned()
                .filter(|card| card.number.value() == card_number_value)
                .collect();
            if cards.len() == 4 {
                four = cards
            }
        }
        if four.len() == 4 {
            cards.retain(|x| !four.contains(x));
            sort_in_descending_order(cards);
            let kicker = cards.pop().unwrap(); // get the card with the lowest value
            let kicker1 = cards[0];
            let kicker2 = cards[1];
            Some(Combination::FourOfAKind(
                [four[0], four[1], four[2], four[3], kicker],
                [kicker1, kicker2],
            ))
        } else {
            None
        }
    }

    pub fn is_straight(cards: &mut Vec<Card>) -> Option<Combination> {
        for window in cards.windows(5) {
            if Self::is_sequence(window) {
                return Some(Combination::Straight([
                    window[0], window[1], window[2], window[3], window[4],
                ]));
            }
        }
        None
    }

    pub fn is_straight_flush(cards: &mut Vec<Card>) -> Option<Combination> {
        for window in cards.windows(5) {
            if Self::is_sequence(window)
                && window.windows(2).all(|pair| pair[0].suit == pair[1].suit)
            {
                return Some(Combination::StraightFlush([
                    window[0], window[1], window[2], window[3], window[4],
                ]));
            }
        }
        None
    }

    fn is_sequence(window: &[Card]) -> bool {
        let mut values: Vec<_> = window.iter().map(|card| card.number.value()).collect();
        if values == vec![14, 5, 4, 3, 2] {
            values = vec![5, 4, 3, 2, 1];
        }
        values.windows(2).all(|pair| pair[0] - 1 == pair[1])
    }

    pub fn is_flush(cards: &mut Vec<Card>) -> Option<Combination> {
        for combo in cards.iter().cloned().combinations(5) {
            if let Some(first_card_suit) = combo.first().map(|card| card.suit) {
                if combo.iter().all(|card| card.suit == first_card_suit) {
                    if cards.len() < 2 {
                        return None;
                    }
                    let max_card = cards[0];
                    let kicker1 = cards[1];
                    let kicker2 = cards[2];
                    let kicker3 = cards[3];
                    let kicker4 = cards[4];
                    return Some(Combination::Flush([
                        max_card, kicker1, kicker2, kicker3, kicker4,
                    ]));
                }
            }
        }
        None
    }
    pub fn is_full_house(cards: &mut Vec<Card>) -> Option<Combination> {
        let mut counts = HashMap::new();
        for card in cards.iter() {
            let count = counts.entry(card.number.value()).or_insert(0);
            *count += 1;
        }
        let mut three = None;
        let mut two = None;
        let mut sorted_counts = counts.iter().collect::<Vec<_>>();
        // crescente
        sorted_counts.sort_by(|a, b| a.0.cmp(b.0));
        for (&number, &count) in sorted_counts.iter().rev() {
            if count >= 3 && three.is_none() {
                three = Some(number);
            } else if count >= 2 && two.is_none() {
                two = Some(number);
            }
        }

        if let (Some(three), Some(two)) = (three, two) {
            let mut three_cards: Vec<Card> =
                cards.iter().cloned().filter(|card| card.number.value() == three).collect();
            let mut two_cards: Vec<Card> =
                cards.iter().cloned().filter(|card| card.number.value() == two).collect();
            cards.retain(|x| !three_cards.contains(x) && !two_cards.contains(x));
            three_cards.sort_by_key(|card| cards.iter().position(|x| *x == *card));
            two_cards.sort_by_key(|card| cards.iter().position(|x| *x == *card));
            let mut combo =
                [three_cards[0], three_cards[1], three_cards[2], two_cards[0], two_cards[1]];
            combo.sort_by(|a, b| b.number.value().cmp(&a.number.value()));
            Some(Combination::FullHouse(combo))
        } else {
            None
        }
    }

    pub fn is(mut cards: Vec<Card>) -> Result<Combination, String> {
        sort_in_descending_order(&mut cards);
        if let Some(combination) = Self::is_royal_flush(&mut cards.clone()) {
            return Ok(combination);
        }
        if let Some(combination) = Self::is_straight_flush(&mut cards.clone()) {
            return Ok(combination);
        }
        if let Some(combination) = Self::is_four_of_a_kind(&mut cards.clone()) {
            return Ok(combination);
        }
        if let Some(combination) = Self::is_full_house(&mut cards.clone()) {
            return Ok(combination);
        }
        if let Some(combination) = Self::is_flush(&mut cards.clone()) {
            return Ok(combination);
        }
        if let Some(combination) = Self::is_straight(&mut cards.clone()) {
            return Ok(combination);
        }
        if let Some(combination) = Self::is_three_of_a_kind(&mut cards.clone()) {
            return Ok(combination);
        }
        if let Some(combination) = Self::is_two_pair(&mut cards.clone()) {
            return Ok(combination);
        }
        if let Some(combination) = Self::is_pair(&mut cards.clone()) {
            return Ok(combination);
        }
        if let Some(combination) = Self::is_high_card(&mut cards.clone()) {
            return Ok(combination);
        }
        Err("is not match combination".to_string())
    }
}
