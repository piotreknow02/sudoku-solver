use crate::model::Sudoku;

impl Sudoku {
    fn possible(&self, num: u8, position: (usize, usize)) -> bool {
        for y in 0..9 {
            if num == self.table[y][position.1].unwrap_or(0) {
                return false
            }
        }
        for x in 0..9 {
            if num == self.table[position.0][x].unwrap_or(0) {
                return false
            }
        }
        true
    }

    pub fn solve(&mut self) {
        for y in 0..9 {
            for x in 0..9 {
                if self.table[y][x] == None {
                    for n in 1..=9 {
                        if self.possible(n, (y, x)) {
                            self.table[y][x] = Some(n);
                            self.solve();
                            self.table[y][x] = None;
                        }
                    }
                }
            }
        }
    }
}