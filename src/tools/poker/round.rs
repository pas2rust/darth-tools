use super::{card::Card, player::Player};

#[derive(Clone, Debug)]
pub struct Round {
    pub id: String,
    pub participants: Vec<Player>,
    pub flop: (Card, Card, Card),
    pub river: Card,
    pub showdown: Card,
    pub pot: u64,
    pub winners: Vec<Player>,
}
