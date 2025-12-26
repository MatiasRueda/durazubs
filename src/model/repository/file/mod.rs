pub mod file_reader;
pub mod file_writer;

use super::SubtitleRepository;
use crate::model::repository::{
    file::{file_reader::FileReader, file_writer::FileWriter},
    repository_error::RepoRes,
};

pub struct FileRepository;

impl FileRepository {
    pub fn new() -> Self {
        Self
    }
}

impl SubtitleRepository for FileRepository {
    fn get_all(&self, path: &str) -> RepoRes<Vec<String>> {
        FileReader::new(path).read_lines()
    }

    fn save(&self, path: &str, lines: &[String]) -> RepoRes<()> {
        FileWriter::new(path).write_lines(lines)
    }
}
