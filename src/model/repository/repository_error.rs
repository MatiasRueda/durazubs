use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum RepositoryError {
    SourceNotFound { context: String },
    ReadError { context: String },
    AccessDenied { context: String },
    SaveFailed { context: String },
    InvalidFormat { details: String },
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            RepositoryError::SourceNotFound { context } => {
                write!(f, "Source not found: {}", context)
            }
            RepositoryError::ReadError { context } => {
                write!(f, "Failed to read data from: {}", context)
            }
            RepositoryError::AccessDenied { context } => {
                write!(f, "Access denied: {}", context)
            }
            RepositoryError::SaveFailed { context } => {
                write!(f, "Failed to save data to: {}", context)
            }
            RepositoryError::InvalidFormat { details } => {
                write!(f, "Format error: {}", details)
            }
        }
    }
}

pub type RepoRes<T> = Result<T, RepositoryError>;
