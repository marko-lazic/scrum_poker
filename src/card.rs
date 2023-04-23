use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Card {
    pub value: Option<u8>,
}

impl Card {
    pub fn new (value: u8) -> Self {
        Card { value: Some(value) }
    }
    pub fn none() -> Self {
        Card { value: None }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.value {
            Some(value) => write!(f, "{}", value),
            None => write!(f, "N/a"),
        }
    }
}