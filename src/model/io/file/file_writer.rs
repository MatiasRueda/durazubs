use crate::model::io::io_error::IOError;
use crate::model::writer::Writer;
use std::fs::OpenOptions;
use std::io::Write;

pub struct FileWriter {
    path: String,
}

impl FileWriter {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl Writer for FileWriter {
    fn write_line(&mut self, line: &str) -> Result<(), IOError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|_| IOError::WriteError {
                context: self.path.clone(),
            })?;

        writeln!(file, "{}", line).map_err(|_| IOError::WriteError {
            context: self.path.clone(),
        })
    }
}
