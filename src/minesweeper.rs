use crate::{cell::Cell, grid::Grid};
use std::{fmt::Display, io, path::PathBuf};

pub struct Minesweeper {
    board: Grid<char>,
}

impl Minesweeper {
    pub fn load(path: PathBuf) -> Result<Self, io::Error> {
        let board = Grid::try_from(path)?;
        Ok(Minesweeper { board })
    }

    pub fn revealed(&self) -> Grid<usize> {
        let revealed_cells: Vec<Vec<Cell<usize>>> = self
            .board
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, cell)| match cell {
                        Cell::Bomb => Cell::Bomb,
                        Cell::Empty(_) => Cell::Empty(self.close_bombs(i, j)),
                    })
                    .collect()
            })
            .collect();
        Grid::from(revealed_cells)
    }

    fn close_bombs(&self, i: usize, j: usize) -> usize {
        self.board
            .neighbors(i, j)
            .iter()
            .filter(|cell| matches!(cell, Cell::Bomb))
            .count()
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
}
