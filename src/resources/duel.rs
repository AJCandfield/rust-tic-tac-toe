pub struct Duel {
    // Whether the duel is currently on
    pub is_on: bool,
    // Number of turns taken since the start of the duel
    pub turn_count: u32,
}

impl Default for Duel {
    fn default() -> Self {
        Self::new()
    }
}

impl Duel {
    pub fn new() -> Self {
        Self {
            is_on: true,
            turn_count: 0,
        }
    }
}
