use std::collections::VecDeque;

use super::player::Player;

pub struct Game {
    // Whether the game is currently on
    pub is_on: bool,
    // Number of duels played so far
    pub duel_count: u32,
    // Number of turns taken since the start of the game
    pub turn_count: u32,
    // Manage turns by using a queue
    pub turn_queue: VecDeque<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            is_on: true,
            duel_count: 0,
            turn_count: 0,
            turn_queue: VecDeque::new(),
        }
    }
}
