mod minesweeper {
    pub struct Grid {
        grid: Vec<Vec<char>>,
        rows: usize,
        cols: usize,
    }

    impl Grid {
        pub fn new(grid: String) -> Self {
            let grid: Vec<Vec<char>> = grid
                .lines()
                .map(|line| line.chars().collect())
                .collect();
            let rows = grid.len();
            let cols = grid[0].len();
            Self { grid, rows, cols}
        }

        //méthode qui parcours la grid et resout le démineur
        pub fn solve_field(&mut self) {
            for i in 0..self.rows {
                for j in 0..self.cols {
                    if !self.is_mine(i, j) {
                        let adjacent_mines = self.adjacent_mines(i, j);
                        self.grid[i][j] = char::from_digit(adjacent_mines, 10).unwrap();
                    }
                }
            }
        }

        fn is_mine(&self, i: usize, j: usize) -> bool {
            if self.grid[i][j] == '*' {
                return true;
            }
            return false;
        }

        //méthode qui check les mines autours de la case
        fn adjacent_mines(&self, i: usize, j: usize) -> u32 {
            let mut adjacent_mine_count = 0;
            for x in -1..=1 {
                for y in -1..=1 {
                    let ni = i as i32 + x;
                    let nj = j as i32 + y;
                    if self.is_in_field(ni, nj) {
                        if self.is_mine(ni as usize, nj as usize) {
                            adjacent_mine_count += 1;
                        }

                    }
                }
            }
            adjacent_mine_count
        }

        pub fn is_in_field(&self, i: i32, j: i32) -> bool {
            i >= 0 && i < self.rows as i32 && j >= 0 && j < self.cols as i32
        }

        pub fn solved_field_to_string(&self) -> String {
            let resolved_rows: Vec<String> = self.grid.iter().map(|row| row.iter().collect::<String>()).collect();
            let resolved_grid: String = resolved_rows.join("\n");
            resolved_grid
        }
    }
}


fn main() {
    let mine_grid: String = String::from("*...\n....\n.*..\n....");
    let mut minesweeper = minesweeper::Grid::new(mine_grid);
    minesweeper.solve_field();
    println!("{}", minesweeper.solved_field_to_string());
}

