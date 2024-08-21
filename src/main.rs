use std::io::stdout;
use std::io::Write;
use std::{fmt, io};

fn main() {
    // "   A|B|C ")
    // "1 |_|_|_ ")
    // "2 |_|_|_ ")
    // "3 |_|_|_ ")
    let mut table: Table = Table {
        size: 5,
        grid: Vec::new(),
    };

    table.make();

    // Prompt user
    print!("Enter position: ");
    let _ = stdout().flush();

    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let input: &str = input.trim();

    // Check input was two characters
    assert_eq!(input.len(), 2);
    let x = &input.chars().next().unwrap();
    let y = &input.chars().next().unwrap();

    print!("x: {}, y: {}", x, y);

    table.draw();
}

struct Coordinate {
    x: u8,
    y: u8,
}
enum SquareState {
    Empty,
    Player1,
    Player2,
}

impl fmt::Display for SquareState {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SquareState::Empty => write!(formatter, "_"),
            SquareState::Player1 => write!(formatter, "x"),
            SquareState::Player2 => write!(formatter, "o"),
        }
    }
}

struct Square {
    coordinate: Coordinate,
    state: SquareState,
}

struct Table {
    size: u8,
    grid: Vec<Vec<Square>>,
}

impl Table {
    fn draw(&self) {
        let edge: &str = "|";

        for x in 0..self.size {
            println!("");

            for y in 0..self.size {
                print!("{edge}");
                print!("{}", self.grid[x as usize][y as usize].state);

                if y == self.size - 1 {
                    // Draw the outer edge of the grid
                    print!("{edge}");
                }
            }
        }
        println!("");
    }

    fn make(&mut self) {
        // Create a 2-dimensional matrix
        // where a column is an array of rows
        // and a row is an array of Coordinates
        let mut columns: Vec<Vec<Square>> = Vec::new();

        // The "y" coordinate measures the rows
        // The "x" coordinate measures the columns

        // Iterate through each row
        for x in 0..self.size {
            let mut row: Vec<Square> = Vec::new();

            // Iterate through each column
            for y in 0..self.size {
                let coordinate = Coordinate { x: x + 1, y: y + 1 };
                let square = Square {
                    coordinate,
                    state: SquareState::Empty,
                };

                row.insert(y.into(), square);
            }

            columns.insert(x.into(), row);
        }

        // Assign the created grid to the object attribute
        self.grid = columns;
    }
}
