use crate::{DarthTools, UuidTrait};

use super::{round::Round, player::Players};

pub enum Kind {
    Cash,
    Tournament,
}
pub type History = Option<Round>;

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
        Self { id, players, kind: Kind::Cash, history: None }
    }
}