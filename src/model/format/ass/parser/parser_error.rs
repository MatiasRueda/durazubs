use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ParserError {
    MissingFields { found: usize },
    DialoguePrefix,
}

impl ParserError {
    fn missing_fields_msg() -> &'static str {
        "The dialogue line does not have enough fields. Expected 10 fields but found"
    }
    fn dialogue_prefix_msg() -> &'static str {
        "The line does not start with the 'Dialogue:' prefix"
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ParserError::MissingFields { found } => {
                let msg = ParserError::missing_fields_msg();
                write!(f, "{} {}", msg, found)
            }
            ParserError::DialoguePrefix => {
                let msg = ParserError::dialogue_prefix_msg();
                write!(f, "{}", msg)
            }
        }
    }
}

pub type ParseRes<T> = std::result::Result<T, ParserError>;
