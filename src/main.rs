use clap::Parser;
use model::Sudoku;
use std::{path::Path, process::exit};

mod model;
mod solver;
mod tokenizer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_file: Box<Path>,

    #[clap(long, short)]
    out_file: Option<Box<Path>>,
}

fn main() {
    let args = Args::parse();
    let mut my_sudoku = match args.input_file.extension().and_then(|s| s.to_str()) {
        Some("json") => Sudoku::from_json(args.input_file.to_str().unwrap()).unwrap(),
        Some("png") => Sudoku::from_ocr(args.input_file),
        Some("jpg") => Sudoku::from_ocr(args.input_file),
        None => exit(1),
    };
    let res = my_sudoku.solve();

    if res {
        match args.out_file {
            Some(p) => my_sudoku.save_json(p.to_str()).unwrap(),
            None => println!("{}", my_sudoku),
        }
    } else {
        eprintln!("This sudoku cannot be solved\nGoodbye");
    }
}
