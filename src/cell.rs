use std::{
    convert::TryFrom,
    fmt::{Display, Formatter},
};

#[derive(Clone, Copy)]
pub enum Cell<V> {
    Bomb,
    Empty(V),
}

impl TryFrom<u8> for Cell<char> {
    type Error = &'static str;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            b'.' => Ok(Cell::Empty(b'.' as char)),
            b'*' => Ok(Cell::Bomb),
            _ => Err("Invalid cell"),
        }
    }
}

impl<V: Display> Display for Cell<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Cell::Bomb => "*".to_string(),
            Cell::Empty(value) => value.to_string(),
        };
        write!(f, "{}", symbol)
    }
}
