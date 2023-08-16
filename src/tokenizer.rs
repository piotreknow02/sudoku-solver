use crate::model::Sudoku;

use serde_json::{from_reader, to_string};
use std::{fs::File, io::BufReader, result::Result, error::Error};


impl Sudoku {
    pub fn from_json(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let table: [[Option<u8>; 9]; 9] = from_reader(reader)?;
        let s = Self{table: table};
        Ok(s)
    }

    #[allow(dead_code)]
    pub fn to_json(&self) -> Result<String, Box<dyn Error>> {
        let json_str = to_string(self)?;
        Ok(json_str)
    }

    // fn read_ocr(&mut self, path: &str) -> Result<()> {
        
    // }
}