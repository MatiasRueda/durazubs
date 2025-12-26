pub type ProcRes<T, E> = std::result::Result<T, E>;

pub trait SubtitleProcessor {
    type Error: std::fmt::Display + std::fmt::Debug;

    fn synchronize(
        &mut self,
        l_a: &mut Vec<String>,
        l_b: &[String],
    ) -> ProcRes<Vec<String>, Self::Error>;

    fn get_lines_to_translate(&self, lines: &mut Vec<String>) -> ProcRes<Vec<String>, Self::Error>;

    fn apply_translation(
        &mut self,
        lines: &mut Vec<String>,
        translations: Vec<String>,
    ) -> ProcRes<Vec<String>, Self::Error>;

    fn translate_internal(&mut self, lines: &mut Vec<String>) -> ProcRes<Vec<String>, Self::Error>;

    fn apply_style(&mut self, lines: &mut Vec<String>) -> ProcRes<Vec<String>, Self::Error>;
}
