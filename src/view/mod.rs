pub mod console;

pub enum AppStatus {
    Welcome,
    Reading,
    ReadingA,
    ReadingB,
    Preprocessing,
    Processing,
    Translating,
    NoLinesToTranslate,
    Styling,
    Writing,
    AskTranslation,
    TranslationFileFound,
    Success,
}

pub struct AppPaths {
    pub path_a: String,
    pub path_b: Option<String>,
}

pub struct AppOptions {
    pub output_path: String,
    pub format_type: String,
    pub sync_enabled: bool,
    pub style: Option<String>,
    pub translation_enabled: bool,
    pub ai_type: Option<String>,
}

pub struct AppConfig {
    pub paths: AppPaths,
    pub options: AppOptions,
}

pub trait View {
    fn get_format(&self) -> String;
    fn request_path_a(&self, ext: &str) -> String;
    fn request_path_b(&self, ext: &str) -> String;
    fn request_path_result(&self, ext: &str) -> String;
    fn display_status(&self, status: AppStatus);
    fn display_error(&self, message: &str);
    fn get_sync_enabled(&self) -> bool;
    fn get_options(&self, output_path: &str, ext: &str, sync_enabled: bool) -> AppOptions;
    fn confirm_translation_ready(&self) -> bool;
}
