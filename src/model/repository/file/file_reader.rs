use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::model::repository::repository_error::{RepoRes, RepositoryError};

pub struct FileReader {
    path: String,
}

impl FileReader {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn read_lines(&self) -> RepoRes<Vec<String>> {
        let file = File::open(&self.path).map_err(|_| RepositoryError::SourceNotFound {
            context: self.path.clone(),
        })?;

        let reader = BufReader::new(file);
        reader
            .lines()
            .collect::<std::io::Result<Vec<String>>>()
            .map_err(|_| RepositoryError::ReadError {
                context: self.path.clone(),
            })
    }
}
