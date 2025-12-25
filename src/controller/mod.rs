use std::time::Instant;

use crate::{
    model::{
        format::ass::{
            ass_error::AssRes, ass_processor::AssProcessor, parser::parser_error::ParserError,
        },
        io::file::{file_reader::FileReader, file_writer::FileWriter},
        reader::Reader,
        subtitle_processor::SubtitleProcessor,
        writer::Writer,
    },
    view::{AppConfig, AppStatus, View},
};

pub struct App<V: View> {
    view: V,
    config: AppConfig,
}

impl<V: View> App<V> {
    const TRANSLATIONS_PATH: &str = "translations.txt";

    pub fn new(view: V) -> Self {
        view.display_status(AppStatus::Welcome);
        let config = view.get_config();
        Self { view, config }
    }

    fn handle_translation_error(&self) -> Option<Vec<String>> {
        self.view
            .display_error(&format!("Source '{}' not found.", Self::TRANSLATIONS_PATH));
        None
    }

    fn try_read_translations(&self) -> Option<Vec<String>> {
        match FileReader::new(Self::TRANSLATIONS_PATH).read_lines() {
            Ok(lines) => Some(lines),
            Err(_) => self.handle_translation_error(),
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
        let lines_a = FileReader::new(&self.config.path_a).read_lines()?;
        let lines_b = FileReader::new(&self.config.path_b).read_lines()?;
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
        match self.config.translation_enabled {
            true => {
                let to_translate = processor.get_lines_to_translate(lines)?;
                FileWriter::new("to_translate.txt").write_lines(&to_translate)?;

                let translations = self.read_translations();
                Ok(processor.apply_translation(lines, translations)?)
            }
            false => Ok(lines.clone()),
        }
    }

    fn step_style(
        &self,
        processor: &mut Box<dyn SubtitleProcessor<Error = ParserError>>,
        lines: &mut Vec<String>,
    ) -> AssRes<Vec<String>> {
        match self.config.style {
            Some(_) => Ok(processor.apply_style(lines)?),
            None => Ok(lines.clone()),
        }
    }

    fn execute_workflow(&mut self) -> AssRes<f64> {
        let (mut lines_a, lines_b) = self.step_read_files()?;
        let mut processor: Box<dyn SubtitleProcessor<Error = ParserError>> =
            Box::new(AssProcessor::new().with_style(self.config.style.clone()));
        let start_time = Instant::now();
        let mut current_lines = self.step_synchronize(&mut processor, &mut lines_a, &lines_b)?;
        current_lines = self.step_translate(&mut processor, &mut current_lines)?;
        current_lines = self.step_style(&mut processor, &mut current_lines)?;

        self.view.display_status(AppStatus::Writing);
        FileWriter::new(&self.config.output_path).write_lines(&current_lines)?;

        Ok(start_time.elapsed().as_secs_f64())
    }

    pub fn run(&mut self) {
        match self.execute_workflow() {
            Ok(elapsed) => self.view.display_success(elapsed),
            Err(e) => self.view.display_error(&e.to_string()),
        }
    }
}
