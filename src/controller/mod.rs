use crate::{
    model::{
        format::ass::{
            ass_error::AssRes, ass_processor::AssProcessor, parser::parser_error::ParserError,
        },
        repository::SubtitleRepository,
        subtitle_persistence::SubtitlePersistence,
        subtitle_processor::SubtitleProcessor,
    },
    view::{AppConfig, AppPaths, AppStatus, View},
};

type Processor = Box<dyn SubtitleProcessor<Error = ParserError>>;
pub struct App<V: View, R: SubtitleRepository> {
    view: V,
    persistence: SubtitlePersistence<R>,
    config: AppConfig,
}

impl<V: View, R: SubtitleRepository> App<V, R> {
    pub fn new(view: V, repository: R) -> Self {
        view.display_status(AppStatus::Welcome);
        let persistence = SubtitlePersistence::new(repository);
        let ext = view.get_format();
        let path_a = Self::get_validated_path_a(&view, &persistence, &ext);
        let path_b = Self::get_validated_path_b(&view, &persistence, &ext);
        let options = view.get_options(&ext);
        Self {
            view,
            persistence,
            config: AppConfig {
                paths: AppPaths { path_a, path_b },
                options,
            },
        }
    }

    fn get_validated_path_a(view: &V, persistence: &SubtitlePersistence<R>, ext: &str) -> String {
        loop {
            let path = view.request_path_a(ext);
            match persistence.check_availability(&path) {
                Ok(_) => return path,
                Err(e) => view.display_error(&e.to_string()),
            }
        }
    }

    fn get_validated_path_b(view: &V, persistence: &SubtitlePersistence<R>, ext: &str) -> String {
        loop {
            let path = view.request_path_b(ext);
            match persistence.check_availability(&path) {
                Ok(_) => return path,
                Err(e) => view.display_error(&e.to_string()),
            }
        }
    }

    fn try_read_translations(&self) -> Option<Vec<String>> {
        match self.persistence.load_translations() {
            Ok(lines) => self.handle_translation_success(lines),
            Err(e) => self.handle_translation_error(e),
        }
    }

    fn handle_translation_success(&self, lines: Vec<String>) -> Option<Vec<String>> {
        self.view.display_status(AppStatus::TranslationFileFound);
        Some(lines)
    }

    fn handle_translation_error(&self, error: impl std::fmt::Display) -> Option<Vec<String>> {
        self.view.display_error(&error.to_string());
        None
    }

    fn read_translations(&self) -> Vec<String> {
        self.view.display_status(AppStatus::AskTranslation);
        loop {
            if !self.view.confirm_translation_ready() {
                return vec![];
            }
            if let Some(lines) = self.try_read_translations() {
                return lines;
            }
        }
    }

    fn step_read_a(&self) -> AssRes<Vec<String>> {
        self.view.display_status(AppStatus::ReadingA);
        Ok(self.persistence.load_subs(&self.config.paths.path_a)?)
    }

    fn step_read_b(&self) -> AssRes<Vec<String>> {
        self.view.display_status(AppStatus::ReadingB);
        Ok(self.persistence.load_subs(&self.config.paths.path_b)?)
    }

    fn step_preprocessing(&self, p: &mut Processor, lines: &mut Vec<String>) -> AssRes<()> {
        self.view.display_status(AppStatus::Preprocessing);
        *lines = p.preprocessing(lines)?;
        Ok(())
    }

    fn step_synchronize(
        &self,
        processor: &mut Processor,
        l_a: &[String],
        l_b: &[String],
    ) -> AssRes<Vec<String>> {
        self.view.display_status(AppStatus::Processing);
        Ok(processor.synchronize(l_a, l_b)?)
    }

    fn step_translate(&self, p: &mut Processor, lines: &mut Vec<String>) -> AssRes<()> {
        let opt = &self.config.options;
        if !opt.translation_enabled {
            return Ok(());
        }
        if !p.has_additional_scene(lines)? {
            self.view.display_status(AppStatus::NoLinesToTranslate);
            return Ok(());
        }
        self.view.display_status(AppStatus::Translating);
        match &opt.ai_type {
            Some(c) if c == "1" => self.internal_translation_flow(p, lines),
            Some(_) => self.external_translation_flow(p, lines),
            _ => Ok(()),
        }
    }
    fn internal_translation_flow(&self, p: &mut Processor, lines: &mut Vec<String>) -> AssRes<()> {
        *lines = p.translate_internal(lines)?;
        Ok(())
    }

    fn external_translation_flow(&self, p: &mut Processor, lines: &mut Vec<String>) -> AssRes<()> {
        let to_tr = p.get_lines_to_translate(lines)?;
        self.persistence.save_translation_to_translate(&to_tr)?;
        let translations = self.read_translations();
        *lines = p.apply_translation(lines, &translations)?;
        Ok(())
    }

    fn step_style(&self, p: &mut Processor, lines: &mut Vec<String>) -> AssRes<()> {
        match &self.config.options.style {
            Some(_) => self.apply_styling_flow(p, lines),
            None => Ok(()),
        }
    }

    fn apply_styling_flow(&self, p: &mut Processor, lines: &mut Vec<String>) -> AssRes<()> {
        self.view.display_status(AppStatus::Styling);
        *lines = p.apply_style(lines)?;
        Ok(())
    }

    fn execute_workflow(&mut self) -> AssRes<()> {
        let output_path = &self.config.options.output_path;
        self.view.display_status(AppStatus::Reading);
        let mut lines_a = self.step_read_a()?;
        let lines_b = self.step_read_b()?;
        let style_name = self.config.options.style.clone();
        let mut processor: Processor = Box::new(AssProcessor::new().with_style(style_name));
        self.step_preprocessing(&mut processor, &mut lines_a)?;
        let mut current_lines = self.step_synchronize(&mut processor, &lines_a, &lines_b)?;
        self.step_translate(&mut processor, &mut current_lines)?;
        self.step_style(&mut processor, &mut current_lines)?;
        self.view.display_status(AppStatus::Writing);
        self.persistence.save_subs(output_path, &current_lines)?;
        Ok(())
    }

    pub fn run(&mut self) {
        match self.execute_workflow() {
            Ok(()) => self.view.display_status(AppStatus::Success),
            Err(e) => self.view.display_error(&e.to_string()),
        }
    }
}
