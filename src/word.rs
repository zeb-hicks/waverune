use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub struct Word(u16);
impl Word {
    pub fn new(value: u16) -> Self {
        Word(value)
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl From<u16> for Word {
    fn from(value: u16) -> Self {
        Word::new(value)
    }
}

impl From<i32> for Word {
    fn from(value: i32) -> Self {
        Word::new(value as u16)
    }
}

impl From<isize> for Word {
    fn from(value: isize) -> Self {
        Word::new(value as u16)
    }
}

impl Into<u16> for Word {
    fn into(self) -> u16 {
        self.0
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04x}", self.0)
    }
}