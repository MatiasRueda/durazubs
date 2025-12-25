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
        let msg = format!("Source '{}' not found.", Self::TRANSLATIONS_PATH);
        self.view.display_error(&msg);
        None
    }

    fn try_read_translations(&self) -> Option<Vec<String>> {
        match FileReader::new(Self::TRANSLATIONS_PATH).read_lines() {
            Ok(lines) => Some(lines),
            Err(_) => self.handle_translation_error(),
        }
    }

    fn attempt_translation_read(&self) -> Option<Vec<String>> {
        match self.try_read_translations() {
            Some(lines) => Some(lines),
            None => None,
        }
    }

    fn read_translations(&self) -> Vec<String> {
        self.view
            .display_status(AppStatus::InstructionsForTranslation);

        loop {
            match self.view.confirm_translation_ready() {
                true => {
                    if let Some(lines) = self.attempt_translation_read() {
                        return lines;
                    }
                }
                false => return vec![],
            }
        }
    }

    fn process_translation(
        &self,
        processor: &mut AssProcessor,
        lines: &mut Vec<String>,
    ) -> AssRes<()> {
        if !self.config.translation_enabled {
            return Ok(());
        }
        let ai_lines = processor.get_lines_to_translate(lines)?;
        let mut temp_writer = FileWriter::new("to_translate.txt");
        temp_writer.write_lines(&ai_lines)?;
        let translations = self.read_translations();
        processor.translated_subtitles(translations);

        Ok(())
    }

    pub fn prepare_processor(
        &self,
        lines: &mut Vec<String>,
    ) -> AssRes<Box<dyn SubtitleProcessor<Error = ParserError>>> {
        let mut processor = AssProcessor::new().with_style(self.config.style.clone());
        processor.with_translation(self.config.translation_enabled);

        if self.config.translation_enabled {
            self.process_translation(&mut processor, lines)?;
        }

        Ok(Box::new(processor))
    }

    fn read_source_files(&self) -> AssRes<(Vec<String>, Vec<String>)> {
        let lines_a = FileReader::new(&self.config.path_a).read_lines()?;
        let lines_b = FileReader::new(&self.config.path_b).read_lines()?;
        Ok((lines_a, lines_b))
    }

    fn execute_processing(
        &self,
        lines_a: &mut Vec<String>,
        lines_b: &[String],
    ) -> AssRes<Vec<String>> {
        let mut processor = self.prepare_processor(lines_a)?;
        self.view.display_status(AppStatus::Processing);
        Ok(processor.process(lines_a, lines_b)?)
    }

    fn save_output(&self, final_result: &[String]) -> AssRes<()> {
        self.view.display_status(AppStatus::Writing);
        FileWriter::new(&self.config.output_path).write_lines(final_result)?;
        Ok(())
    }

    fn execute_workflow(&mut self) -> AssRes<f64> {
        self.view.display_status(AppStatus::Reading);
        let (mut lines_a, lines_b) = self.read_source_files()?;
        let start_time = Instant::now();
        let final_result = self.execute_processing(&mut lines_a, &lines_b)?;
        self.save_output(&final_result)?;
        Ok(start_time.elapsed().as_secs_f64())
    }

    pub fn run(&mut self) {
        match self.execute_workflow() {
            Ok(elapsed) => self.view.display_success(elapsed),
            Err(e) => self.view.display_error(&e.to_string()),
        }
    }
}
