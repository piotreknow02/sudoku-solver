use crate::model::Sudoku;

impl Sudoku {
    fn possible(&self, num: u8, position: (usize, usize)) -> bool {
        for y in 0..9 {
            if num == self.table[y][position.1].unwrap_or(0) {
                println!("Number {} not possible {},{}", num, y, position.1);
                return false
            }
        }
        for x in 0..9 {
            if num == self.table[position.0][x].unwrap_or(0) {
                println!("Number {} not possible", num);
                return false
            }
        }

        let y0: usize = (position.1 / 3) * 3;
        let x0: usize = (position.0 / 3) * 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.table[y0+i][x0+j].unwrap_or(0) == num {
                    return false
                }
            }
        }
        println!("Number {} possible", num);
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
                    return;
                }
            }
        }
    }
}