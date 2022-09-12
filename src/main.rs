use minesweeper::minesweeper::Minesweeper;
use std::path::PathBuf;

fn main() {
    let minesweeper = Minesweeper::load(PathBuf::from("./example.txt")).unwrap();
    println!("{}", minesweeper);
    println!("{}", minesweeper.revealed());
}
