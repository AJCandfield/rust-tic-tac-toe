pub mod resources;
pub mod utils;

use crate::resources::duel::Duel;
use crate::resources::game::Game;
use crate::resources::player::Player;
use crate::resources::table::Table;

use crate::utils::cli::MessageBuffer;
use crate::utils::cli::Prompt;

fn main() {
    // "   A|B|C "
    // "1 |_|_|_ "
    // "2 |_|_|_ "
    // "3 |_|_|_ "

    // Size of the grid
    const TABLE_SIZE: u32 = 5;

    let mut error_buffer = MessageBuffer::new();
    let mut info_buffer = MessageBuffer::new();

    let mut game = Game::new();

    let player_1: Player = Player::new(String::from("Alex"), String::from("x"), 0);
    game.turn_queue.push_back(player_1);

    let player_2: Player = Player::new(String::from("Gaby"), String::from("o"), 0);
    game.turn_queue.push_back(player_2);

    // Start game loop
    'game: while game.is_on {
        let mut table: Table = Table::new(TABLE_SIZE);
        table.init();

        // Start a new match
        let mut duel = Duel::new();

        // Start a new turn
        'duel: while duel.is_on && game.is_on {
            // Clean screen and draw game
            table.clear();
            table.draw();

            // Print any errors and messages in the buffer
            error_buffer.flush();
            info_buffer.flush();

            // Get reference to first player in the turn queue
            let current_player = game
                .turn_queue
                .front_mut()
                .expect("Failed to get first element of the queue.");

            let input: String = Prompt::coordinates_or_command(current_player.name.as_str());

            match input.as_str() {
                "q" | "quit" | "exit" => {
                    info_buffer.push(format!("Game over!"));
                    game.is_on = false;
                    continue 'game;
                }

                "r" | "reset" => {
                    table.init();
                    info_buffer.push(format!("Game reset!"));
                    continue 'duel;
                }

                "s" | "stats" => {
                    info_buffer.push(format!("Match count: {}", game.duel_count));
                    info_buffer.push(format!("Total turn count: {}", game.turn_count));
                    info_buffer.push(format!("Turn count in current match: {}", duel.turn_count));
                    continue 'duel;
                }

                "w" | "win" => {
                    duel.turn_count += 1;
                    game.turn_count += 1;
                    duel.is_on = false;
                    break 'duel;
                }

                _ => {
                    println!("nothing selected");
                }
            };

            let input: Vec<char> = input.chars().collect();
            if input.len() != 2 {
                println!();
                error_buffer.push(format!(
                    "❌ Please, enter two digits between 1 to {TABLE_SIZE}!"
                ));
            }

            // Next turn
            if !error_buffer.is_empty() {
                continue 'duel;
            }

            let x: u32 = input[0]
                .to_digit(10)
                .expect("Error parsing 'x' coordinate from user input.");

            let y: u32 = input[1]
                .to_digit(10)
                .expect("Error parsing 'x' coordinate from user input.");

            if x > TABLE_SIZE || x < 1 {
                error_buffer.push(format!(
                    "❌ Coordinate 'x' must be between 1 and {}!",
                    TABLE_SIZE
                ));
            }

            if y > TABLE_SIZE || y < 1 {
                error_buffer.push(format!(
                    "❌ Coordinate 'y' must be between 1 and {}!",
                    TABLE_SIZE
                ));
            }

            // Next turn
            if !error_buffer.is_empty() {
                continue 'duel;
            }

            table.update_symbol(&current_player.symbol, x, y);

            info_buffer.push(format!(
                "Player '{}' played x:{} and y:{}",
                current_player.name, x, y,
            ));

            let victory = table.check_win(&current_player.symbol);

            if victory {
                table.clear();
                table.draw();
                info_buffer.push(format!("{} won the game!", current_player.name));
                current_player.score += 1;
                info_buffer.flush();
                duel.is_on = false;
            }

            // The player that just played a turn now becomes the last in the queue
            game.turn_queue.rotate_left(1);

            duel.turn_count += 1;
            game.turn_count += 1;
        }

        game.duel_count += 1;
        game.is_on = Prompt::next_duel();
    }
}
