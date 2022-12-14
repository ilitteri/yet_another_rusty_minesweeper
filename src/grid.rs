use crate::cell::Cell;
use std::{
    convert::TryFrom,
    fmt::{Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
    slice::Iter,
};

pub struct Grid<C> {
    grid: Vec<Vec<C>>,
}

impl<C: Copy> Grid<C> {
    pub fn iter(&self) -> Iter<Vec<C>> {
        self.grid.iter()
    }

    pub fn neighbors(&self, i: usize, j: usize) -> Vec<C> {
        let mut neighbors = Vec::new();
        for x in i.saturating_sub(1)..=i + 1 {
            for y in j.saturating_sub(1)..=j + 1 {
                if x < self.grid.len() && y < self.grid[x].len() {
                    neighbors.push(self.grid[x][y]);
                }
            }
        }
        neighbors
    }
}

// TODO: Errors should be handled in a better way.
impl TryFrom<PathBuf> for Grid<Cell<char>> {
    type Error = io::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut grid: Vec<Vec<Cell<char>>> = vec![];

        for line in reader.lines() {
            let mut row: Vec<Cell<char>> = Vec::new();
            for char in line?.as_bytes() {
                row.push(
                    Cell::try_from(*char)
                        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?,
                );
            }
            grid.push(row);
        }
        Ok(Self::from(grid))
    }
}

// TODO: this should be a try_from because the input grid could be a non rectangular matrix.
impl<C> From<Vec<Vec<C>>> for Grid<C> {
    fn from(grid: Vec<Vec<C>>) -> Self {
        Grid { grid }
    }
}

impl<V: Display> Display for Grid<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
