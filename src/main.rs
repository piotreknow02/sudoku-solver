use model::Sudoku;

mod model;
mod solver;
mod tokenizer;

fn main() {
    let mut my_sudoku = Sudoku::from_json("testdata/1.json").unwrap();
    // my_sudoku.solve();
    println!("{}", &my_sudoku);

}
