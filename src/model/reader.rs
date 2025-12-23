use crate::model::io::io_error::IOError;

pub trait Reader {
    fn read_lines(&self) -> Result<Vec<String>, IOError>;
}
