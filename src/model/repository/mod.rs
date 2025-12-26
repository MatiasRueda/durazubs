use crate::model::repository::repository_error::RepoRes;

pub mod file;
pub mod repository_error;

pub trait SubtitleRepository {
    fn get_all(&self, id: &str) -> RepoRes<Vec<String>>;
    fn save(&self, id: &str, lines: &[String]) -> RepoRes<()>;
}
