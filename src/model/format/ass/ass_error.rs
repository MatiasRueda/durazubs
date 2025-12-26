use std::fmt::{Display, Formatter, Result};

use crate::model::format::ass::parser::parser_error::ParserError;
use crate::model::repository::repository_error::RepositoryError;

#[derive(Debug)]
pub enum AssError {
    Repository(RepositoryError),
    Parser(ParserError),
}

impl From<RepositoryError> for AssError {
    fn from(err: RepositoryError) -> Self {
        AssError::Repository(err)
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
            AssError::Repository(repo_err) => write!(f, "{}", repo_err),
            AssError::Parser(parser_err) => write!(f, "{}", parser_err),
        }
    }
}

pub type AssRes<T> = std::result::Result<T, AssError>;
