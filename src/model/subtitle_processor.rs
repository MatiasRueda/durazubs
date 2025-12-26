pub type ProcRes<T, E> = std::result::Result<T, E>;

pub trait SubtitleProcessor {
    type Error: std::fmt::Display + std::fmt::Debug;

    fn synchronize(&self, l_a: &[String], l_b: &[String]) -> ProcRes<Vec<String>, Self::Error>;

    fn get_lines_to_translate(&self, lines: &mut Vec<String>) -> ProcRes<Vec<String>, Self::Error>;

    fn apply_translation(
        &self,
        lines: &mut Vec<String>,
        translations: &[String],
    ) -> ProcRes<Vec<String>, Self::Error>;

    fn translate_internal(&self, lines: &mut Vec<String>) -> ProcRes<Vec<String>, Self::Error>;

    fn preprocessing(&self, lines: &mut Vec<String>) -> ProcRes<Vec<String>, Self::Error>;

    fn has_additional_scene(&self, lines: &Vec<String>) -> ProcRes<bool, Self::Error>;

    fn apply_style(&self, lines: &Vec<String>) -> ProcRes<Vec<String>, Self::Error>;
}
