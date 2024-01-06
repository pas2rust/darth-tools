use crate::card::Card;

pub enum Action {
    Fold,
    Call(u128),
    Rise(u128),
    AllIn(u128)
}

pub struct Player {
    pub id: String,
    pub hand: (Card, Card),
    pub action: Action,
    pub bankroll: u128,
    pub stack: u128
}

pub type Players = Vec<Player>;