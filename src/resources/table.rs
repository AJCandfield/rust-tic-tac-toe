pub struct Table {
    pub size: u32,

    // Create a 2-dimensional matrix where:
    //   a row is an array of symbols
    //   a column is an array of rows
    grid: Vec<Vec<String>>,
}

impl Table {
    pub fn new(size: u32) -> Table {
        Table {
            size,
            grid: Vec::new(),
        }
    }

    pub fn clear(&self) {
        // Escape sequence to delete terminal buffer
        print!("\x1B[2J\x1B[H");
    }

    pub fn draw(&self) {
        let edge: &str = "|";

        for y in 0..self.size {
            println!();

            for x in 0..self.size {
                print!("{edge}");
                print!("{}", self.grid[x as usize][y as usize]);

                if x == self.size - 1 {
                    // Draw the outer edge of the grid
                    print!("{edge}");
                }
            }
        }
        println!();
    }

    pub fn init(&mut self) {
        // The "x" coordinate measures the rows
        // The "y" coordinate measures the columns

        // Create a column in each row
        for x in 0..self.size {
            let mut column: Vec<String> = Vec::new();

            // Iterate through each column
            for y in 0..self.size {
                column.insert(y as usize, String::from("_"));
            }

            self.grid.insert(x as usize, column);
        }
    }

    pub fn update_symbol(&mut self, symbol: &String, x: u32, y: u32) {
        self.grid[x as usize - 1][y as usize - 1] = symbol.clone();
    }

    pub fn check_win(&self, symbol: &String) -> bool {
        // A win happens when:
        //   the same symbol appears in an entire row
        //   the same symbol appears in an entire column
        //   the same symbol appears in an entire diagonal

        // Check two diagonals
        //  first diagonal starts at position (x=0, y=0)
        //  second diagonal starts at position (x=grid.len -1, y=grid.len -1)

        for x in 0..self.size {
            if self.grid[x as usize][x as usize] != *symbol.to_string() {
                return false;
            }
        }

        true
    }
}
