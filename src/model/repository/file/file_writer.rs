use std::fs::OpenOptions;
use std::io::Write;

use crate::model::repository::repository_error::{RepoRes, RepositoryError};

pub struct FileWriter {
    path: String,
}

impl FileWriter {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn write_lines<'a, I>(&self, lines: I) -> RepoRes<()>
    where
        I: IntoIterator<Item = &'a String>,
    {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.path)
            .map_err(|_| RepositoryError::SaveFailed {
                context: self.path.clone(),
            })?;

        for l in lines {
            writeln!(file, "{}", l).map_err(|_| RepositoryError::SaveFailed {
                context: self.path.clone(),
            })?;
        }
        Ok(())
    }
}
