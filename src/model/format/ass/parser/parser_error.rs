use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ParserError {
    MissingFields { found: usize },
    DialoguePrefix,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ParserError::MissingFields { found } => {
                write!(f, "Missing Fields: Expected 10, found {}", found)
            }
            ParserError::DialoguePrefix => {
                write!(f, "Prefix Error: Line does not start with 'Dialogue:'")
            }
        }
    }
}

pub type ParseRes<T> = std::result::Result<T, ParserError>;
