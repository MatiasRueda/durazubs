use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum IOError {
    ConnectionFailed { context: String },
    ReadError { context: String },
    WriteError { context: String },
    InvalidData { details: String },
}

impl IOError {
    fn read_msg(ctx: &str) -> String {
        format!("Failed to read from: {}", ctx)
    }

    fn write_msg(ctx: &str) -> String {
        format!("Failed to write to: {}", ctx)
    }

    fn conn_msg(ctx: &str) -> String {
        format!("Connection error: {}", ctx)
    }
}

impl Display for IOError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            IOError::ReadError { context } => write!(f, "{}", Self::read_msg(context)),
            IOError::WriteError { context } => write!(f, "{}", Self::write_msg(context)),
            IOError::ConnectionFailed { context } => write!(f, "{}", Self::conn_msg(context)),
            IOError::InvalidData { details } => write!(f, "Invalid data: {}", details),
        }
    }
}
