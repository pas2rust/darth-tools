pub enum Kind {
    Cash,
    Tournament,
}

type Players = Vec<String>;

pub struct Table {
    pub players: Players,
    pub kind: Kind,
}

impl Table {
    fn new_cash(players: Players) -> Self {
        if players.len() >= 2 && players.len() <= 10 {
            panic!("The number of players must be within the range of 2 to 10 for a valid table.")
        }
        Self { players, kind: Kind::Cash }
    }
}
