use std::{
    convert::TryFrom,
    fmt::{Display, Formatter},
};

#[derive(Clone, Copy)]
pub enum Cell {
    Bomb,
    Empty(usize),
}

impl TryFrom<u8> for Cell {
    type Error = &'static str;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            b'.' => Ok(Cell::Empty(0)),
            b'*' => Ok(Cell::Bomb),
            _ => Err("Invalid cell"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Cell::Bomb => "*".to_string(),
            Cell::Empty(value) => value.to_string(),
        };
        write!(f, "{}", symbol)
    }
}
