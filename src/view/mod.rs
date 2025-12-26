pub mod console;

pub enum AppStatus {
    Welcome,
    Reading,
    Processing,
    Translating,
    Styling,
    Writing,
    InstructionsForTranslation,
    TranslationFileFound,
    Success,
}

pub struct AppConfig {
    pub path_a: String,
    pub path_b: String,
    pub output_path: String,
    pub format_type: String,
    pub style: Option<String>,
    pub translation_enabled: bool,
    pub ai_type: Option<String>,
}
pub trait View {
    fn display_status(&self, status: AppStatus);
    fn display_error(&self, message: &str);
    fn get_config(&self) -> AppConfig;
    fn confirm_translation_ready(&self) -> bool;
}
