use super::{card::Card, player::Player};

#[derive(Clone, Debug)]
pub struct Round {
    pub id: String,
    pub participants: Vec<Player>,
    pub pot: u64,
    pub winners: Vec<Player>,
}
