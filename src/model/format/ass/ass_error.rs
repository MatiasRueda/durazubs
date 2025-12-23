use std::fmt::{Display, Formatter, Result};

use crate::model::{format::ass::parser::parser_error::ParserError, io::io_error::IOError};

#[derive(Debug)]
pub enum AssError {
    Io(IOError),
    Parser(ParserError),
}

impl From<IOError> for AssError {
    fn from(err: IOError) -> Self {
        AssError::Io(err)
    }
}

impl From<ParserError> for AssError {
    fn from(err: ParserError) -> Self {
        AssError::Parser(err)
    }
}

impl Display for AssError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AssError::Io(io_err) => write!(f, "{}", io_err),
            AssError::Parser(parser_err) => write!(f, "{}", parser_err),
        }
    }
}

pub type AssRes<T> = std::result::Result<T, AssError>;
