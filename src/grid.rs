use crate::cell::Cell;
use std::{
    convert::TryFrom,
    fmt::{Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader}, path::PathBuf, slice::Iter,
};

pub struct Grid {
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn iter(&self) -> Iter<Vec<Cell>> {
       self.grid.iter() 
    }

    pub fn neighbors(&self, i: usize, j: usize) -> Vec<Cell> {
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

impl TryFrom<PathBuf> for Grid {
    type Error = io::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut grid: Vec<Vec<Cell>> = vec![];

        for line in reader.lines() {
            let mut row: Vec<Cell> = Vec::new();
            for char in line?.as_bytes() {
                match char {
                    b'.' => row.push(Cell::Empty(0)),
                    b'*' => row.push(Cell::Bomb),
                    _ => return Err(Self::Error::new(io::ErrorKind::InvalidData, "Invalid cell")),
                }
            }
            grid.push(row);
        }
        Ok(Self::from(grid))
    }
}

// TODO: this should be a try_from because the input grid could not be a square matrix.
impl From<Vec<Vec<Cell>>> for Grid {
    fn from(grid: Vec<Vec<Cell>>) -> Self {
        Grid { grid }
    }
}

impl Display for Grid {
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
