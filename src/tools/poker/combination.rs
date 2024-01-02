use std::collections::HashMap;

use itertools::Itertools;

use super::card::{Card, Number};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Combination {
    RoyalFlush,                      // A Coroa Real
    StraightFlush(Card),             // Escada Real
    FourOfAKind(Card, Card, Card),   // Quadra ou Poker
    FullHouse(Card, Card),           // Full ou Casa Cheia
    Flush(Card, Card, Card),         // Flush ou Cor
    Straight(Card),                  // Sequência ou Escada
    ThreeOfAKind(Card, Card, Card),  // Trinca
    TwoPair(Card, Card, Card),       // Dois Pares
    OnePair(Card, Card, Card, Card), // Um Par
    HighCard(Card, Card, Card),      // Carta Alta
}

impl Combination {
    pub fn force(&self) -> usize {
        match *self {
            Combination::RoyalFlush => 10000,
            Combination::StraightFlush(high_card) => 9000 + high_card.number.value(),
            Combination::FourOfAKind(four_of_a_kind_card, kicker1, kicker2) => {
                8000 + four_of_a_kind_card.number.value()
                    + kicker1.number.value()
                    + kicker2.number.value()
            }
            Combination::FullHouse(three_of_a_kind_card, pair) => {
                7000 + three_of_a_kind_card.number.value() + pair.number.value()
            }
            Combination::Flush(high_card, kicker1, kicker2) => {
                6000 + high_card.number.value() + kicker1.number.value() + kicker2.number.value()
            }
            Combination::Straight(high_card) => 5000 + high_card.number.value(),
            Combination::ThreeOfAKind(three_of_a_kind_card, kicker1, kicker2) => {
                4000 + three_of_a_kind_card.number.value()
                    + kicker1.number.value()
                    + kicker2.number.value()
            }
            Combination::TwoPair(high_pair_card, low_pair_card, kicker) => {
                3000 + high_pair_card.number.value()
                    + low_pair_card.number.value()
                    + kicker.number.value()
            }
            Combination::OnePair(pair_card, kicker1, kicker2, kicker3) => {
                2000 + pair_card.number.value()
                    + kicker1.number.value()
                    + kicker2.number.value()
                    + kicker3.number.value()
            }
            Combination::HighCard(high_card, kicker1, kicker2) => {
                1000 + high_card.number.value() + kicker1.number.value() + kicker2.number.value()
            }
        }
    }
    pub fn find_high_card_number(cards: &Vec<Card>) -> Card {
        *cards.iter().max_by_key(|card| card.number.value()).unwrap()
    }
    pub fn is_royal_flush(cards: &mut Vec<Card>) -> Option<Combination> {
        if let Some(Combination::StraightFlush(high_card)) =
            Self::is_straight_flush(&mut cards.clone())
        {
            if high_card.number == Number::Ace {
                return Some(Combination::RoyalFlush);
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
            let high_card_in_combination = pairs[0];
            cards.retain(|x| !pairs.contains(x));
            let kicker1 = cards[0];
            let kicker2 = cards[1];
            let kicker3 = cards[2];
            Some(Combination::OnePair(high_card_in_combination, kicker1, kicker2, kicker3))
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
            let high_card_in_combination = Self::find_high_card_number(&pairs);
            cards.retain(|x| !pairs.contains(x));
            let kicker1 = cards[0];
            let kicker2 = cards[1];
            Some(Combination::TwoPair(high_card_in_combination, kicker1, kicker2))
        } else {
            None
        }
    }

    pub fn is_three_of_a_kind(cards: &mut Vec<Card>) -> Option<Combination> {
        let mut three: Vec<Card> = Vec::new();
        for card in cards.iter() {
            let card_number_value = card.number.value();
            let cards: Vec<Card> = cards
                .iter()
                .cloned()
                .filter(|card| card.number.value() == card_number_value)
                .collect();
            if cards.len() == 3 {
                three = cards
            }
        }
        if three.len() == 3 {
            let high_card_in_combination = three[0];
            cards.retain(|x| !three.contains(x));
            let kicker1 = cards[0];
            let kicker2 = cards[1];
            Some(Combination::ThreeOfAKind(high_card_in_combination, kicker1, kicker2))
        } else {
            None
        }
    }

    pub fn is_high_card(cards: &mut Vec<Card>) -> Option<Combination> {
        let card = cards.remove(0);
        let kicker1 = cards[0];
        let kicker2 = cards[1];
        Some(Combination::HighCard(card, kicker1, kicker2))
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
            let high_card_in_combination = four[0];
            cards.retain(|x| !four.contains(x));
            let kicker1 = cards[0];
            let kicker2 = cards[1];
            Some(Combination::FourOfAKind(high_card_in_combination, kicker1, kicker2))
        } else {
            None
        }
    }
    pub fn is_straight(cards: &mut Vec<Card>) -> Option<Combination> {
        // Ordena as cartas em ordem crescente
        cards.sort_by(|a, b| a.number.value().cmp(&b.number.value()));
        for window in cards.windows(5) {
            if Self::is_sequence(window) {
                return Some(Combination::Straight(window[4].clone()));
            }
        }
        None
    }

    pub fn is_straight_flush(cards: &mut Vec<Card>) -> Option<Combination> {
        // Ordena as cartas em ordem crescente
        cards.sort_by(|a, b| a.number.value().cmp(&b.number.value()));
        for window in cards.windows(5) {
            if Self::is_sequence(window)
                && window.windows(2).all(|pair| pair[0].suit == pair[1].suit)
            {
                return Some(Combination::StraightFlush(window[4].clone()));
            }
        }
        None
    }

    fn is_sequence(window: &[Card]) -> bool {
        let mut values: Vec<_> = window.iter().map(|card| card.number.value()).collect();
        values.sort();
        if values == vec![2, 3, 4, 5, 14] {
            values = vec![1, 2, 3, 4, 5];
        }
        values.windows(2).all(|pair| pair[0] + 1 == pair[1])
    }

    pub fn is_flush(cards: &mut Vec<Card>) -> Option<Combination> {
        for combo in cards.iter().cloned().combinations(5) {
            if let Some(first_card_suit) = combo.get(0).map(|card| card.suit) {
                if combo.iter().all(|card| card.suit == first_card_suit) {
                    let max_card = Self::find_high_card_number(&combo);
                    cards.retain(|card| !combo.contains(card));
                    if cards.len() < 2 {
                        return None;
                    }
                    let kicker1 = cards[0];
                    let kicker2 = cards[1];
                    return Some(Combination::Flush(max_card, kicker1, kicker2));
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
        for (&number, &count) in counts.iter() {
            if count == 3 {
                three = Some(number);
            } else if count == 2 {
                two = Some(number);
            }
        }
        if let (Some(three), Some(two)) = (three, two) {
            let three_cards: Vec<Card> =
                cards.iter().cloned().filter(|card| card.number.value() == three).collect();
            let two_cards: Vec<Card> =
                cards.iter().cloned().filter(|card| card.number.value() == two).collect();
            cards.retain(|x| !three_cards.contains(x) && !two_cards.contains(x));
            Some(Combination::FullHouse(three_cards[0], two_cards[0]))
        } else {
            None
        }
    }

    pub fn is(mut cards: Vec<Card>) -> Result<Combination, String> {
        // ordem decrescente
        cards.sort_by(|a, b| b.number.value().cmp(&a.number.value()));
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
