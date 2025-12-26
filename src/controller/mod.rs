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

pub struct App<V: View, R: SubtitleRepository> {
    view: V,
    persistence: SubtitlePersistence<R>,
    config: AppConfig,
}

impl<V: View, R: SubtitleRepository> App<V, R> {
    pub fn new(view: V, repository: R) -> Self {
        view.display_status(AppStatus::Welcome);
        let persistence = SubtitlePersistence::new(repository);
        let extension = view.get_format();
        let path_a = loop {
            let p = view.request_path_a(&extension);
            if let Err(e) = persistence.check_availability(&p) {
                view.display_error(&e.to_string());
            } else {
                break p;
            }
        };
        let path_b = loop {
            let p = view.request_path_b(&extension);
            if let Err(e) = persistence.check_availability(&p) {
                view.display_error(&e.to_string());
            } else {
                break p;
            }
        };
        let options = view.get_options(&extension);
        let config = AppConfig {
            paths: AppPaths { path_a, path_b },
            options,
        };
        Self {
            view,
            persistence,
            config,
        }
    }

    fn try_read_translations(&self) -> Option<Vec<String>> {
        match self.persistence.load_translations() {
            Ok(lines) => {
                self.view.display_status(AppStatus::TranslationFileFound);
                Some(lines)
            }
            Err(e) => {
                self.view.display_error(&e.to_string());
                None
            }
        }
    }

    fn read_translations(&self) -> Vec<String> {
        self.view
            .display_status(AppStatus::InstructionsForTranslation);
        loop {
            match self.view.confirm_translation_ready() {
                true => {
                    if let Some(lines) = self.try_read_translations() {
                        return lines;
                    }
                }
                false => return vec![],
            }
        }
    }

    fn step_read_files(&self) -> AssRes<(Vec<String>, Vec<String>)> {
        self.view.display_status(AppStatus::Reading);
        let lines_a = self.persistence.load_subtitles(&self.config.paths.path_a)?;
        let lines_b = self.persistence.load_subtitles(&self.config.paths.path_b)?;
        Ok((lines_a, lines_b))
    }

    fn step_synchronize(
        &self,
        processor: &mut Box<dyn SubtitleProcessor<Error = ParserError>>,
        l_a: &mut Vec<String>,
        l_b: &[String],
    ) -> AssRes<Vec<String>> {
        self.view.display_status(AppStatus::Processing);
        Ok(processor.synchronize(l_a, l_b)?)
    }

    fn step_translate(
        &self,
        processor: &mut Box<dyn SubtitleProcessor<Error = ParserError>>,
        lines: &mut Vec<String>,
    ) -> AssRes<Vec<String>> {
        if self.config.options.translation_enabled {
            self.view.display_status(AppStatus::Translating);
            match &self.config.options.ai_type {
                Some(choice) if choice == "1" => Ok(processor.translate_internal(lines)?),
                Some(_) => {
                    let to_translate = processor.get_lines_to_translate(lines)?;
                    self.persistence
                        .save_translation_to_translate(&to_translate)?;
                    let translations = self.read_translations();
                    Ok(processor.apply_translation(lines, translations)?)
                }
                None => Ok(lines.clone()),
            }
        } else {
            Ok(lines.clone())
        }
    }

    fn step_style(
        &self,
        processor: &mut Box<dyn SubtitleProcessor<Error = ParserError>>,
        lines: &mut Vec<String>,
    ) -> AssRes<Vec<String>> {
        match &self.config.options.style {
            Some(_) => {
                self.view.display_status(AppStatus::Styling);
                Ok(processor.apply_style(lines)?)
            }
            None => Ok(lines.clone()),
        }
    }

    fn execute_workflow(&mut self) -> AssRes<()> {
        let (mut lines_a, lines_b) = self.step_read_files()?;
        let mut processor: Box<dyn SubtitleProcessor<Error = ParserError>> =
            Box::new(AssProcessor::new().with_style(self.config.options.style.clone()));
        let mut current_lines = self.step_synchronize(&mut processor, &mut lines_a, &lines_b)?;
        current_lines = self.step_translate(&mut processor, &mut current_lines)?;
        current_lines = self.step_style(&mut processor, &mut current_lines)?;
        self.view.display_status(AppStatus::Writing);
        self.persistence
            .save_subtitles(&self.config.options.output_path, &current_lines)?;

        Ok(())
    }
    pub fn run(&mut self) {
        match self.execute_workflow() {
            Ok(()) => self.view.display_status(AppStatus::Success),
            Err(e) => self.view.display_error(&e.to_string()),
        }
    }
}
