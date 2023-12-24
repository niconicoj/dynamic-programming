use std::{
    fmt::{Display, Formatter},
    io::BufRead,
};

pub struct Sudoku {
    grid: Vec<Vec<u8>>,
}

impl Sudoku {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let mut grid = Vec::new();
        for line in reader.lines() {
            let line = line?;
            assert_eq!(
                line.len(),
                9,
                "The sudoku grid must be 9x9 integers ranging from 0 to 9 (0 being an empty cell)"
            );
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as u8);
            }
            grid.push(row);
        }
        Ok(Self { grid })
    }

    pub fn solve(&mut self) {
        if !self._solve(0) {
            panic!("no solution found");
        }
    }

    fn _solve(&mut self, index: usize) -> bool {
        let (row, col) = (index / 9, index % 9);
        if let Some(value) = self.grid.get(row).and_then(|r| r.get(col)) {
            if value != &0 {
                self._solve(index + 1)
            } else {
                let possible_values = self.possible_values(index);

                for v in possible_values {
                    self.grid[row][col] = v;
                    if !self._solve(index + 1) {
                        self.grid[row][col] = 0;
                    } else {
                        return true;
                    }
                }
                false
            }
        } else {
            // we reached the end of the grid by recursively placing valid values
            // in each cell, so we're done
            return true;
        }
    }

    fn possible_values(&self, index: usize) -> Vec<u8> {
        let (row, col) = (index / 9, index % 9);
        (1..=9).filter(|v| self.is_valid(row, col, *v)).collect()
    }

    fn is_valid(&self, row: usize, col: usize, v: u8) -> bool {
        self.is_valid_row(row, v) && self.is_valid_col(col, v) && self.is_valid_block(row, col, v)
    }

    fn is_valid_row(&self, row: usize, v: u8) -> bool {
        self.grid[row].iter().find(|&c| c == &v).is_none()
    }

    fn is_valid_col(&self, col: usize, v: u8) -> bool {
        self.grid.iter().find(|r| r[col] == v).is_none()
    }

    fn is_valid_block(&self, row: usize, col: usize, v: u8) -> bool {
        // interger division returns integers
        let (block_row, block_col) = (row / 3, col / 3);
        for i in 0..3 {
            for j in 0..3 {
                let (r, c) = (block_row * 3 + i, block_col * 3 + j);
                if self.grid[r][c] == v {
                    return false;
                }
            }
        }
        true
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┏━━━┯━━━┯━━━┳━━━┯━━━┯━━━┳━━━┯━━━┯━━━┓")?;
        for (i, row) in self.grid.iter().enumerate() {
            let is_last = i == 8;
            let is_third = i % 3 == 2;
            let s = format!(
                "┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃ {} │ {} │ {} ┃",
                row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[8]
            );
            writeln!(f, "{}", hide_zeros(&s))?;
            match (is_last, is_third) {
                (true, _) => writeln!(f, "┗━━━┷━━━┷━━━┻━━━┷━━━┷━━━┻━━━┷━━━┷━━━┛")?,
                (false, false) => writeln!(f, "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨")?,
                (false, true) => writeln!(f, "┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫")?,
            }
        }
        Ok(())
    }
}

fn hide_zeros(s: &str) -> String {
    s.replace("0", " ")
}
