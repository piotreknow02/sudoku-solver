use crate::model::Sudoku;

impl Sudoku {
    fn possible(&self, num: u8, position: (usize, usize)) -> bool {
        for y in 0..9 {
            if num == self.table[y][position.1].unwrap_or(0) {
                return false;
            }
        }
        for x in 0..9 {
            if num == self.table[position.0][x].unwrap_or(0) {
                return false;
            }
        }

        let y0: usize = position.0 - (position.0 % 3);
        let x0: usize = position.1 - (position.1 % 3);
        for i in 0..3 {
            for j in 0..3 {
                if self.table[y0+i][x0+j].unwrap_or(0) == num {
                    return false;
                }
            }
        }
        true
    }

    pub fn solve(&mut self) -> bool {
        for y in 0..9 {
            for x in 0..9 {
                if self.table[y][x].is_none() {
                    for n in 1..=9 {
                        if self.possible(n, (y, x)) {
                            self.table[y][x] = Some(n);
                            if self.solve() {
                                return true;
                            }
                            self.table[y][x] = None;
                        }
                    }
                    return false;
                }
            }
        }
        true
    }
}