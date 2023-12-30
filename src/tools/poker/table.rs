use super::{player::Player, round::Round};

#[derive(Clone, Debug)]
pub enum Kind {
    CashGame,
}

#[derive(Clone, Debug)]
pub struct Table {
    pub id: String,
    pub rounds: Vec<Round>,
    pub players: Vec<Player>,
    pub kind: Kind,
}
