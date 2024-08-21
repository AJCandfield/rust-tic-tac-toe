use std::{fmt, io};

struct Coordinate {
    x: u32,
    y: u32,
}
pub enum SquareState {
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

pub struct Square {
    coordinate: Coordinate,
    state: SquareState,
}

impl Square {
    pub fn update_state(&mut self, new_state: SquareState) {
        self.state = new_state;
    }
}

pub struct Table {
    pub size: u32,
    pub grid: Vec<Vec<Square>>,
}

impl Table {
    pub fn draw(&self) {
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

    pub fn make(&mut self) {
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

                row.insert(y as usize, square);
            }

            columns.insert(x as usize, row);
        }

        // Assign the created grid to the object attribute
        self.grid = columns;
    }
}
