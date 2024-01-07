use crate::{DarthTools, UuidTrait};
use darth_rust::DarthRust;
use serde::{Serialize, Deserialize};
use super::{round::Round, player::Players};

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum Kind {
    #[default]
    Cash,
    Tournament,
}
pub type History = Vec<Round>;

#[derive(DarthRust, Debug, Serialize, Deserialize, Default)]
pub struct Table {
    pub id: String,
    pub players: Players,
    pub kind: Kind,
    pub history: History
}

impl Table {
    fn new_cash(players: Players) -> Self {
        if players.len() >= 2 && players.len() <= 10 {
            panic!("The number of players must be within the range of 2 to 10 for a valid table.")
        }
        let id = DarthTools::new_uuid();
        Self { id, players, kind: Kind::Cash, history: vec![] }
    }
}