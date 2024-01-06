use crate::card::Card;

use super::player::Players;



pub enum Phase {
    Flop(Card, Card, Card),
    River(Card),
    Turn(Card)
}

pub struct Round {
    pub id: String,
    pub participants: Players,
    pub winners: Players,
    pub phase: Phase
}