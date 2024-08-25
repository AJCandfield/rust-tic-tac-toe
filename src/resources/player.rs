use std::fmt;

pub struct Player {
    pub name: String,
    pub symbol: String,
    pub score: u32,
}

impl Player {
    pub fn new(name: String, symbol: String, score: u32) -> Player {
        Player {
            name: name.to_string(),
            symbol,
            score,
        }
    }
    pub fn new_dummy() -> Player {
        Player {
            name: String::from("Dummy"),
            symbol: String::from("_"),
            score: 0,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

// Implementing PartialEq to compare based on symbol
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

// Implementing Eq because Eq requires PartialEq to be implemented
impl Eq for Player {}
