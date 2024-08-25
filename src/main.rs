pub mod resources;
pub mod utils;

use crate::resources::duel::Duel;
use crate::resources::game::Game;
use crate::resources::player::Player;
use crate::resources::table::Table;
use crate::utils::Utils;

fn main() {
    // "   A|B|C "
    // "1 |_|_|_ "
    // "2 |_|_|_ "
    // "3 |_|_|_ "

    let size: u32 = 3;

    let mut game = Game::new();

    let player_1: Player = Player::new(String::from("Alex"), String::from("x"), 0);
    game.turn_queue.push_back(player_1);

    let player_2: Player = Player::new(String::from("Gaby"), String::from("o"), 0);
    game.turn_queue.push_back(player_2);

    'game: while game.is_on {
        let mut table: Table = Table::new(size);
        table.init();

        // Start a new match
        let mut duel = Duel::new();

        'duel: while duel.is_on {
            // Start of turn

            // Clean screen and draw game
            table.clear();
            table.draw();

            // Print any error and messages in the buffer
            duel.flush_msg();
            duel.flush_err();

            // Clone the first player in the turn queue
            let current_player = game
                .turn_queue
                .front_mut()
                .expect("Failed to get first element of the queue.");

            let input: String = Utils::prompt_input(current_player.name.as_str());

            // Exit if input is in the array
            if ["q", "quit", "exit"].contains(&input.as_str()) {
                duel.msg_buffer.push(format!("Game over!"));
                duel.is_on = false;
                game.is_on = false;
                continue 'game;
            }

            if ["r", "reset"].contains(&input.as_str()) {
                table.init();
                duel.msg_buffer.push(format!("Game reset!"));
                continue 'duel;
            }

            if ["s", "stats"].contains(&input.as_str()) {
                duel.msg_buffer
                    .push(format!("Match count: {}", game.duel_count));
                duel.msg_buffer
                    .push(format!("Total turn count: {}", game.turn_count));
                duel.msg_buffer
                    .push(format!("Turn count in current match: {}", duel.turn_count));
                continue 'duel;
            }

            // Temp condition to simulate a win
            if ["w", "win"].contains(&input.as_str()) {
                duel.turn_count += 1;
                game.turn_count += 1;
                duel.is_on = false;
                break 'duel;
            }

            let input: Vec<char> = input.chars().collect();
            if input.len() != 2 {
                println!();
                duel.err_buffer
                    .push(format!("❌ Please, enter two digits between 1 to {size}!"));
            }

            // Next turn
            if !duel.err_buffer.is_empty() {
                continue 'duel;
            }

            let x: u32 = input[0]
                .to_digit(10)
                .expect("Error parsing 'x' coordinate from user input.");

            let y: u32 = input[1]
                .to_digit(10)
                .expect("Error parsing 'x' coordinate from user input.");

            if x > size || x < 1 {
                duel.err_buffer
                    .push(format!("❌ Coordinate 'x' must be between 1 and {}!", size));
            }

            if y > size || y < 1 {
                duel.err_buffer
                    .push(format!("❌ Coordinate 'y' must be between 1 and {}!", size));
            }

            // Next turn
            if !duel.err_buffer.is_empty() {
                continue 'duel;
            }

            table.update_symbol(&current_player.symbol, x, y);

            duel.msg_buffer.push(format!(
                "Player '{}' played x:{} and y:{}",
                current_player.name, x, y,
            ));

            let victory = table.check_win(&current_player.symbol);

            if victory {
                table.clear();
                table.draw();
                duel.msg_buffer
                    .push(format!("{} won the game!", current_player.name));
                current_player.score += 1;
                duel.flush_msg();
                duel.is_on = false;
            }

            // The player that just played a turn now becomes the last in the queue
            game.turn_queue.rotate_left(1);

            duel.turn_count += 1;
            game.turn_count += 1;
        }

        game.duel_count += 1;
        game.is_on = Utils::prompt_next_duel();
    }
}
