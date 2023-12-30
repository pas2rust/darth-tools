use super::{player::Player, table::Table};

#[derive(Clone, Debug)]
pub struct History {
    pub id: String,
    pub player: Player,
    pub table: Vec<Table>,
}
