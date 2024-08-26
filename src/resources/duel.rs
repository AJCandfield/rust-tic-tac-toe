pub struct Duel {
    // Whether the duel is currently on
    pub is_on: bool,
    // Number of turns taken since the start of the duel
    pub turn_count: u32,
}

impl Duel {
    pub fn new() -> Duel {
        Duel {
            is_on: true,
            turn_count: 0,
        }
    }
}
