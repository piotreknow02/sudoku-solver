use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Deserialize, Serialize)]
pub struct Sudoku {
    pub table: [[Option<u8>; 9]; 9]
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for y in 0..9 {
            if y % 3 == 0 {
                writeln!(f, "+---+---+---+")?;
            } 
            for x in 0..9 {
                if x % 3 == 0 {
                    write!(f, "|")?;
                }
                write!(f, "{:?}", self.table[x][y].unwrap_or(0))?;
            }
            write!(f, "|\n")?;
        }
        writeln!(f, "+---+---+---+\n")?;
        Ok(())
    }
}

