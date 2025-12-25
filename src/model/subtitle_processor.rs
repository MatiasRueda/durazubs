pub type ProcRes<T, E> = std::result::Result<T, E>;

pub trait SubtitleProcessor {
    type Error: std::fmt::Display + std::fmt::Debug;
    fn process(
        &mut self,
        l_a: &mut Vec<String>,
        l_b: &[String],
    ) -> ProcRes<Vec<String>, Self::Error>;

    fn get_lines_to_translate(&self, lines: &mut Vec<String>) -> ProcRes<Vec<String>, Self::Error>;
}
