use std::collections::HashMap;

use super::card::Card;

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Combination {
    HighCard(u8),      // 1
    OnePair(u8),       // 2
    TwoPair(u8),       // 3
    ThreeOfAKind(u8),  // 4
    Straight(u8),      // 5
    Flush(u8),         // 6
    FullHouse(u8),     // 7
    FourOfAKind(u8),   // 8
    StraightFlush(u8), // 9
    RoyalFlush,        // 10
}

impl Combination {
    pub fn weight(&self) -> u8 {
        match *self {
            Combination::HighCard(_) => 1,
            Combination::OnePair(_) => 2,
            Combination::TwoPair(_) => 3,
            Combination::ThreeOfAKind(_) => 4,
            Combination::Straight(_) => 5,
            Combination::Flush(_) => 6,
            Combination::FullHouse(_) => 7,
            Combination::FourOfAKind(_) => 8,
            Combination::StraightFlush(_) => 9,
            Combination::RoyalFlush => 10,
        }
    }
    pub fn kicker(&self) -> Option<u8> {
        match *self {
            Combination::HighCard(kicker) => Some(kicker),
            Combination::OnePair(kicker) => Some(kicker),
            Combination::TwoPair(kicker) => Some(kicker),
            Combination::ThreeOfAKind(kicker) => Some(kicker),
            Combination::Straight(kicker) => Some(kicker),
            Combination::Flush(kicker) => Some(kicker),
            Combination::FullHouse(kicker) => Some(kicker),
            Combination::FourOfAKind(kicker) => Some(kicker),
            Combination::StraightFlush(kicker) => Some(kicker),
            Combination::RoyalFlush => None,
        }
    }
    pub fn is_high_card(cards: &[Card; 7]) -> Option<Combination> {
        let max_weight = cards.iter().map(|card| card.number.weight()).max().unwrap();
        Some(Combination::HighCard(max_weight))
    }

    pub fn is_one_pair(cards: &[Card; 7]) -> Option<Combination> {
        let mut weights = [0; 15];
        for card in cards.iter() {
            weights[card.number.weight() as usize] += 1;
        }
        for i in (2..=14).rev() {
            if weights[i] >= 2 {
                return Some(Combination::OnePair(i as u8));
            }
        }
        None
    }
    pub fn is_straight(cards: &[Card; 7]) -> Option<Combination> {
        let mut weights = [false; 15];
        for card in cards.iter() {
            weights[card.number.weight() as usize] = true;
        }

        for i in (2..=10).rev() {
            if weights[i] && weights[i - 1] && weights[i - 2] && weights[i - 3] && weights[i - 4] {
                return Some(Combination::Straight(i as u8));
            }
        }

        // Check for A-5 straight
        if weights[14] && weights[2] && weights[3] && weights[4] && weights[5] {
            return Some(Combination::Straight(5)); // Ace-low straight
        }

        // Check for 10-A straight
        if weights[14] && weights[10] && weights[11] && weights[12] && weights[13] {
            return Some(Combination::Straight(10));
        }

        None
    }

    pub fn is_flush(cards: &[Card; 7]) -> Option<Combination> {
        let mut suits = [0; 4];
        let mut suit_cards = [[false; 15]; 4];
        for card in cards.iter() {
            suits[card.suit as usize] += 1;
            suit_cards[card.suit as usize][card.number.weight() as usize] = true;
        }
        for i in 0..4 {
            if suits[i] >= 5 {
                for j in (2..=14).rev() {
                    if suit_cards[i][j] {
                        return Some(Combination::Flush(j as u8));
                    }
                }
            }
        }
        None
    }

    pub fn is_full_house(cards: &[Card; 7]) -> Option<Combination> {
        let mut weights = [0; 15];
        let mut three = 0;
        let mut two = 0;
        for card in cards.iter() {
            weights[card.number.weight() as usize] += 1;
        }
        for i in (2..=14).rev() {
            if weights[i] >= 3 {
                three = i as u8;
                break;
            }
        }
        for i in (2..=14).rev() {
            if weights[i] >= 2 && i as u8 != three {
                two = i as u8;
                break;
            }
        }
        if three > 0 && two > 0 {
            return Some(Combination::FullHouse(three));
        }
        None
    }

    pub fn is_four_of_a_kind(cards: &[Card; 7]) -> Option<Combination> {
        let mut weights = [0; 15];
        for card in cards.iter() {
            weights[card.number.weight() as usize] += 1;
        }
        for i in (2..=14).rev() {
            if weights[i] >= 4 {
                return Some(Combination::FourOfAKind(i as u8));
            }
        }
        None
    }
    pub fn is_straight_flush(cards: &[Card; 7]) -> Option<Combination> {
        let mut suits = [[false; 15]; 4];
        for card in cards.iter() {
            suits[card.suit as usize][card.number.weight() as usize] = true;
        }
        for i in 0..4 {
            for j in (2..=10).rev() {
                if suits[i][j..j + 5].iter().all(|&x| x) {
                    return Some(Combination::StraightFlush(j as u8 + 4));
                }
            }
        }
        None
    }

    pub fn is_royal_flush(cards: &[Card; 7]) -> Option<Combination> {
        let mut suits = [[false; 15]; 4];
        for card in cards.iter() {
            suits[card.suit as usize][card.number.weight() as usize] = true;
        }
        for i in 0..4 {
            if suits[i][10..15].iter().all(|&x| x) {
                return Some(Combination::RoyalFlush);
            }
        }
        None
    }
    pub fn is_three_of_a_kind(cards: &[Card; 7]) -> Option<Combination> {
        let mut weights = [0; 15];
        for card in cards.iter() {
            weights[card.number.weight() as usize] += 1;
        }
        for i in (2..=14).rev() {
            if weights[i] >= 3 {
                return Some(Combination::ThreeOfAKind(i as u8));
            }
        }
        None
    }
    pub fn is_two_pair(cards: &[Card; 7]) -> Option<Combination> {
        let mut weights = [0; 15];
        let mut pairs = 0;
        let mut max_pair_weight = 0;
        for card in cards.iter() {
            weights[card.number.weight() as usize] += 1;
        }
        for i in (2..=14).rev() {
            if weights[i] >= 2 {
                pairs += 1;
                if pairs == 2 {
                    return Some(Combination::TwoPair(max_pair_weight));
                }
                max_pair_weight = i as u8;
            }
        }
        None
    }
    pub fn best_possible_combination_texas_holdem(cards: &[Card; 7]) -> Option<Combination> {
        let combinations = [
            Self::is_royal_flush(cards),
            Self::is_straight_flush(cards),
            Self::is_four_of_a_kind(cards),
            Self::is_full_house(cards),
            Self::is_flush(cards),
            Self::is_straight(cards),
            Self::is_three_of_a_kind(cards),
            Self::is_two_pair(cards),
            Self::is_one_pair(cards),
            Self::is_high_card(cards),
        ];

        for combination in &combinations {
            if let Some(c) = combination {
                return Some(c.clone());
            }
        }

        None
    }
}
