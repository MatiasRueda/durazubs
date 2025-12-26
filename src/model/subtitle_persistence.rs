use std::path::Path;

use crate::model::repository::SubtitleRepository;
use crate::model::repository::repository_error::{RepoRes, RepositoryError};

pub struct SubtitlePersistence<R: SubtitleRepository> {
    repository: R,
}

impl<R: SubtitleRepository> SubtitlePersistence<R> {
    const TRANSLATIONS_PATH: &'static str = "translations.txt";
    const TO_TRANSLATE_PATH: &'static str = "to_translate.txt";

    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn check_availability(&self, path: &str) -> RepoRes<()> {
        let p = Path::new(path);
        if !p.exists() {
            return Err(RepositoryError::SourceNotFound {
                context: path.to_string(),
            });
        }
        if !p.is_file() {
            return Err(RepositoryError::ReadError {
                context: format!("{} is not a file", path),
            });
        }
        Ok(())
    }

    pub fn load_translations(&self) -> RepoRes<Vec<String>> {
        self.repository.get_all(Self::TRANSLATIONS_PATH)
    }

    pub fn save_translation_to_translate(&self, lines: &[String]) -> RepoRes<()> {
        self.repository.save(Self::TO_TRANSLATE_PATH, lines)
    }

    pub fn load_subs(&self, path: &str) -> RepoRes<Vec<String>> {
        self.repository.get_all(path)
    }

    pub fn save_subs(&self, path: &str, lines: &[String]) -> RepoRes<()> {
        self.repository.save(path, lines)
    }
}
