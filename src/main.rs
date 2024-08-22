use crate::resources::Game;
use crate::resources::Player;
use crate::resources::Table;

use std::collections::VecDeque;
use std::io;
use std::io::Write;

pub mod resources;

fn main() {
    // "   A|B|C "
    // "1 |_|_|_ "
    // "2 |_|_|_ "
    // "3 |_|_|_ "
    let size: u32 = 3;

    let mut game = Game::new();

    // Manage turns by using a queue
    let mut turn_queue: VecDeque<&mut Player> = VecDeque::new();

    let mut player_1: Player = Player::new(String::from("Alex"), 'x', false, 0);
    turn_queue.push_back(&mut player_1);

    let mut player_2: Player = Player::new(String::from("Gaby"), 'o', false, 0);
    turn_queue.push_back(&mut player_2);

    let mut table: Table = Table::new(size);
    table.init();

    loop {
        table.draw();

        let current_player = turn_queue
            .pop_front()
            .expect("Failed to get first element of the queue.");

        current_player.is_active = true;

        // Prompt user
        print!("({}) Enter position: ", current_player.name);
        let _ = io::stdout().flush();

        // Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");
        let input: &str = input.trim();

        // Exit if input is in the array
        if ["q", "quit", "exit"].contains(&input) {
            return;
        }

        if ["r", "reset"].contains(&input) {
            table.init();
            turn_queue.push_front(current_player);
            continue;
        }

        // Check input was two characters
        if input.len() != 2 {
            println!("❌ Please, enter two digits between 1 to {}!", size);
            turn_queue.push_front(current_player);
            continue;
        }

        let input: Vec<char> = input.chars().collect();

        let x = input[0]
            .to_digit(10)
            .expect("Error parsing 'x' coordinate from user input.");

        let y = input[1]
            .to_digit(10)
            .expect("Error parsing 'x' coordinate from user input.");

        if x > size || x < 1 {
            println!("❌ Coordinate 'x' must be between 1 and {}!", size);
            turn_queue.push_front(current_player);
            continue;
        }

        if y > size || y < 1 {
            println!("❌ Coordinate 'y' must be between 1 and {}!", size);
            turn_queue.push_front(current_player);
            continue;
        }

        table.update_symbol(current_player.symbol, x, y);

        println!(
            "Player '{}' played x:{} and y:{}",
            current_player.name, x, y
        );

        turn_queue.push_back(current_player);
    }
}
