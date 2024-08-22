pub struct Game {
    // Number of matches played so far
    pub match_count: u32,
    // Number of turns taken since the start of the game
    pub game_turn_count: u32,
    // Number of turns taken since the start of the match
    pub match_turn_count: u32,
    // List of errors to be displayed to the user at the start of a turn
    pub error_buffer: Vec<String>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            match_count: 0,
            game_turn_count: 0,
            match_turn_count: 0,
            error_buffer: Vec::new(),
        }
    }
}

pub struct Player {
    pub name: String,
    pub symbol: char,
    pub is_active: bool,
    pub score: u32,
}

impl Player {
    pub fn new(name: String, symbol: char, is_active: bool, score: u32) -> Player {
        Player {
            name: name.to_string(),
            symbol,
            is_active,
            score,
        }
    }
}

pub struct Table {
    pub size: u32,

    // Create a 2-dimensional matrix where:
    //   a row is an array of symbols
    //   a column is an array of rows
    grid: Vec<Vec<char>>,
}

impl Table {
    pub fn new(size: u32) -> Table {
        Table {
            size,
            grid: Vec::new(),
        }
    }

    pub fn draw(&self) {
        // Escape sequence to delete terminal buffer
        print!("\x1B[2J\x1B[H");

        let edge: &str = "|";

        for y in 0..self.size {
            println!("");

            for x in 0..self.size {
                print!("{edge}");
                print!("{}", self.grid[x as usize][y as usize]);

                if x == self.size - 1 {
                    // Draw the outer edge of the grid
                    print!("{edge}");
                }
            }
        }
        println!("");
    }

    pub fn init(&mut self) {
        // The "x" coordinate measures the rows
        // The "y" coordinate measures the columns

        // Create a column in each row
        for x in 0..self.size {
            let mut column: Vec<char> = Vec::new();

            // Iterate through each column
            for y in 0..self.size {
                let symbol = '_';

                column.insert(y as usize, symbol);
            }

            self.grid.insert(x as usize, column);
        }
    }

    pub fn update_symbol(&mut self, symbol: char, x: u32, y: u32) {
        self.grid[x as usize - 1][y as usize - 1] = symbol;
    }

    // pub fn check_win(&self) -> Option<Player> {
    // A win happens when:
    //   the same symbol appears in an entire row
    //   the same symbol appears in an entire column
    //   the same symbol appears in an entire diagonal

    // Check two diagonals
    //  first diagonal starts at position (x=0, y=0)
    //  second diagonal starts at position (x=grid.len -1, y=grid.len -1)

    // let mut player: &Player = self.grid[0 as usize][0 as usize];
    // for x in 1..self.size {
    //     if self.grid[x as usize][x as usize] != symbol {
    //         break;
    //     }
    // }

    // return Some(player);
    // }
}
