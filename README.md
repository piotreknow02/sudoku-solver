# sudoku-solver

Simple sudoku solver in rust with focus on performance

---

## About

This program currently accepts only JSON arrays as puzzles and outputs solution to json documents or pretty-prints it to stdout.

## Running

Normal

```bash
cargo run -- testdata/1.json
```

With json output

```bash
cargo run -- testdata/1.json --out-file solution.json
```

## Building

```bash
cargo build --release
```

## Installing

```bash
mv target/release/sudoku-solver /usr/local/bin/
```
