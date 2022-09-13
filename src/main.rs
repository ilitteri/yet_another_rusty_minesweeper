use minesweeper::minesweeper::Minesweeper;
use std::{env, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let minesweeper = Minesweeper::load(PathBuf::from(&args[1])).unwrap();
    println!("Hidden\n{}", minesweeper);
    println!("Revealed\n{}", minesweeper.revealed());
}
