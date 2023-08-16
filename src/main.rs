use model::Sudoku;

mod model;
mod solver;
mod tokenizer;

fn main() {
    let mut my_sudoku = Sudoku::from_json("testdata/1.json").unwrap();
    let res = my_sudoku.solve();
    if res {
        println!("{}", &my_sudoku);
    } else {
        eprintln!("This sudoku cannot be solved\nGoodbye");
    }

}
