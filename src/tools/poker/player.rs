use super::{
    card::Card, combination::Combination, history::History,
    position::Position,
};

#[derive(Clone, Debug)]
pub enum Decision {
    Call,
    Fold,
    Rise,
    Check,
}
#[derive(Clone, Debug)]
pub struct Player {
    pub id: String,
    pub history: Vec<History>,
    pub hand: (Card, Card),
    pub position: Position,
    pub stack: u64,
    pub bank: u64,
    pub bet: u64,
    pub decisions: Vec<Decision>,
    pub combination: Combination,
}