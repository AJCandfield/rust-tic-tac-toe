use crate::resources::Square;
use crate::resources::SquareState;
use crate::resources::Table;
use std::io;
use std::io::Write;

pub mod resources;

fn main() {
    // "   A|B|C ")
    // "1 |_|_|_ ")
    // "2 |_|_|_ ")
    // "3 |_|_|_ ")

    let size: u32 = 5;

    // TODO: Move grid init inside module
    let grid: Vec<Vec<Square>> = Vec::new();

    let mut table: Table = Table { size, grid };

    table.make();
    table.draw();

    // Prompt user
    print!("(Player 1) Enter position: ");
    let _ = io::stdout().flush();

    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let input: &str = input.trim();

    // Check input was two characters
    assert_eq!(input.len(), 2);

    let input: Vec<char> = input.chars().collect();

    let x = input[0]
        .to_digit(10)
        .expect("Error parsing 'x' coordinate from user input.");

    let y = input[1]
        .to_digit(10)
        .expect("Error parsing 'x' coordinate from user input.");

    // TODO: Better handling of index access
    assert!(x < size);
    assert!(x > 0);

    assert!(y < size);
    assert!(y > 0);

    println!("x: {}, y: {}", x, y);

    // TODO: Create function that accepts the coordinates to update the state
    table.grid[x as usize - 1][y as usize - 1].update_state(SquareState::Player1);

    // TODO: Implement loop and turns
    table.draw();
}
