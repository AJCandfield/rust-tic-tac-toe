use std::io;
use std::io::Write;

pub struct Utils {}
impl Utils {
    pub fn prompt_input(player_name: &str) -> String {
        print!("({}) Enter coordinates: ", player_name);
        let _ = io::stdout().flush();

        // Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");
        return input.trim().to_string();
    }

    pub fn prompt_next_duel() -> bool {
        loop {
            print!("Another game? [y/n]: ");
            let _ = io::stdout().flush();

            // Read input
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error");
            let input = input.trim();

            match input {
                "y" => return true,
                "n" => return false,
                _ => {
                    println!("Please, choose either 'y' or 'n'.");
                }
            }
        }
    }
}
