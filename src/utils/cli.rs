use std::io;
use std::io::Write;

pub struct MessageBuffer {
    // List of messages to be displayed
    messages: Vec<String>,
}

impl MessageBuffer {
    pub fn new() -> MessageBuffer {
        MessageBuffer {
            messages: Vec::new(),
        }
    }
}

impl MessageBuffer {
    pub fn flush(&mut self) {
        for msg in self.messages.drain(..) {
            println!("{}", msg);
        }
    }

    pub fn push(&mut self, msg: String) {
        self.messages.push(msg);
    }

    pub fn is_empty(&self) -> bool {
        return self.messages.is_empty();
    }
}

pub struct Prompt {}

impl Prompt {
    pub fn coordinates_or_command(player_name: &str) -> String {
        let msg: String = format!(
            "({}) Enter coordinates or 'help' for command list: ",
            player_name
        );

        let input: String = get_input(&msg);

        return input;
    }

    pub fn next_duel() -> bool {
        loop {
            let msg = format!("Another duel? [y/n]: ");
            let input = get_input(&msg);

            match input.as_str() {
                "y" => return true,
                "n" => return false,
                _ => {
                    println!("Please, choose either 'y' or 'n'.");
                }
            }
        }
    }
}

fn get_input(msg: &str) -> String {
    print!("{msg}");
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input.");

    return input.trim().to_string();
}
