use crate::model::Sudoku;

use serde_json::{from_reader, to_string};
use serde::ser::StdError;
use std::{fs::File, io::{BufReader, Write}, result::Result, error::Error};


impl Sudoku {
    pub fn from_json(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // let table: [[Option<u8>; 9]; 9] = from_reader(reader)?;
        let s = Self{table: from_reader(reader)?};
        Ok(s)
    }

    pub fn save_json(&self, path: &str) -> Result<(), Box<(dyn StdError)>> {
        let mut file = File::create(path)?;
        let _ = file.write_all(self.to_json()?.as_bytes());
        Ok(())
    }

    fn to_json(&self) -> Result<String, Box<dyn Error>> {
        let json_str = to_string(&self.table)?;
        Ok(json_str)
    }
}