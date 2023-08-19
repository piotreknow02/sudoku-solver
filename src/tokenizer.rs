use std::{fs::File, io::{BufReader, Write, Cursor}, result::Result, error::Error};
use serde_json::{from_reader, to_string};
use serde::ser::StdError;
use image::{GenericImageView, open, ImageBuffer, ImageFormat, Rgba, EncodableLayout};
use leptess::LepTess;
use crate::model::Sudoku;


impl Sudoku {
    pub fn from_json(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let s = Self{table: from_reader(reader)?};
        Ok(s)
    }

    pub fn save_json(&self, path: &str) -> Result<(), Box<(dyn StdError)>> {
        let mut file = File::create(path)?;
        file.write_all(self.to_json()?.as_bytes())?;
        Ok(())
    }

    fn to_json(&self) -> Result<String, Box<dyn Error>> {
        let json_str = to_string(self)?;
        Ok(json_str)
    }

    pub fn from_ocr(path: &str) -> Result<Self, Box<dyn Error>> {
        let img = open(path)?;
        let (field_width, field_height) = (img.width() / 9, img.height() / 9);
        let mut parts: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> = Vec::new();

        for i in 0..81 {
            let x = (i as u32 / 9) * field_width;
            let y = (i as u32 % 9) * field_height;

            let part = img.view(x, y, field_width, field_height);

            parts.push(part.to_image());
        }

        let mut ocr = LepTess::new(None, "eng")?;
        let mut s = Self{table: [[None; 9]; 9]};

        for (i, p) in parts.iter().enumerate() {
            let mut buffer = Vec::new();
            
            img.write_to(
                &mut Cursor::new(&mut buffer),
                ImageFormat::Png,
            )?;
            ocr.set_image_from_mem(&buffer)?;
            let num = ocr.get_utf8_text()?;
            println!("{}", &num);
            s.table[i/9][i%9] = Some(5);
        }

        Ok(s)
    }
}
