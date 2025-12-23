use crate::model::io::io_error::IOError;
use crate::model::reader::Reader;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct FileReader {
    path: String,
}

impl FileReader {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl Reader for FileReader {
    fn read_lines(&self) -> std::result::Result<Vec<String>, IOError> {
        let file = File::open(&self.path).map_err(|_| IOError::ReadError {
            context: self.path.clone(),
        })?;

        let reader = BufReader::new(file);
        reader
            .lines()
            .collect::<std::io::Result<Vec<String>>>()
            .map_err(|_| IOError::ReadError {
                context: self.path.clone(),
            })
    }
}
