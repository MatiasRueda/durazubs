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
    view::console::Console,
};

pub struct App {
    reader_a: FileReader,
    reader_b: FileReader,
    writer: FileWriter,
    view: Console,
}

impl App {
    const TRANSLATIONS_PATH: &str = "translations.txt";

    pub fn new(path_a: &str, path_b: &str, output_path: &str) -> Self {
        Self {
            reader_a: FileReader::new(path_a),
            reader_b: FileReader::new(path_b),
            writer: FileWriter::new(output_path),
            view: Console::new(),
        }
    }

    fn request_extracted_file_name(&self) -> String {
        loop {
            let name = self.view.request_file_name();
            match name.to_lowercase().as_str() {
                Self::TRANSLATIONS_PATH => self.view.file_name_error(),
                _ => return name,
            }
        }
    }

    fn successful_read(&self, lines: Vec<String>) -> Option<Vec<String>> {
        self.view.show_translation_load_success();
        Some(lines)
    }

    fn failed_read(&self) -> Option<Vec<String>> {
        self.view
            .show_translation_read_error(Self::TRANSLATIONS_PATH);
        None
    }

    fn try_read_translations(&self) -> Option<Vec<String>> {
        match FileReader::new(Self::TRANSLATIONS_PATH).read_lines() {
            Ok(lines) => self.successful_read(lines),
            Err(_) => self.failed_read(),
        }
    }

    fn read_translations(&self) -> Vec<String> {
        self.view.show_translation_instructions();
        loop {
            self.view.wait_for_input();
            if let Some(lines) = self.try_read_translations() {
                return lines;
            }
        }
    }

    fn external_translation(
        &self,
        processor: &mut AssProcessor,
        lines: &mut Vec<String>,
    ) -> AssRes<()> {
        let name = self.request_extracted_file_name();
        let ai_lines = processor.get_lines_to_translate(lines)?;
        let mut temp_writer = FileWriter::new(&name);
        temp_writer.write_lines(&ai_lines)?;

        let translations = self.read_translations();
        processor.translated_subtitles(translations);
        Ok(())
    }

    fn translation_method(
        &self,
        processor: &mut AssProcessor,
        lines: &mut Vec<String>,
    ) -> AssRes<()> {
        match self.view.request_translation_type().as_str() {
            "1" => Ok(()),
            _ => self.external_translation(processor, lines),
        }
    }

    fn process_request_styler(&self) -> Option<String> {
        if self.view.request_coloring() {
            Some(self.view.request_style_type())
        } else {
            None
        }
    }

    fn process_request_translation(
        &self,
        processor: &mut AssProcessor,
        lines: &mut Vec<String>,
    ) -> AssRes<()> {
        let should_translate = self.view.request_scene_translation();
        processor.with_translation(should_translate);

        if should_translate {
            self.translation_method(processor, lines)?;
        }
        Ok(())
    }

    pub fn setup_ass_file(
        &self,
        lines: &mut Vec<String>,
    ) -> AssRes<Box<dyn SubtitleProcessor<Error = ParserError>>> {
        let style = self.process_request_styler();
        let mut processor = AssProcessor::new().with_style(style);
        self.process_request_translation(&mut processor, lines)?;
        Ok(Box::new(processor))
    }

    pub fn prepare_processor(
        &self,
        lines: &mut Vec<String>,
    ) -> AssRes<Box<dyn SubtitleProcessor<Error = ParserError>>> {
        match self.view.request_format().as_str() {
            "1" => self.setup_ass_file(lines),
            _ => Ok(Box::new(AssProcessor::new())),
        }
    }

    pub fn run(&mut self) {
        self.view.show_welcome();

        let result: AssRes<()> = (|| {
            self.view.reading_step();
            let mut lines_a = self.reader_a.read_lines()?;
            let lines_b = self.reader_b.read_lines()?;
            let mut processor = self.prepare_processor(&mut lines_a)?;
            let start_time = Instant::now();
            self.view.processing_step();
            let final_result = processor.process(&mut lines_a, &lines_b)?;
            self.view.writing_step();
            self.writer.write_lines(&final_result)?;
            self.view.show_success(start_time.elapsed().as_secs_f64());
            Ok(())
        })();

        if let Err(e) = result {
            self.view.show_error(&e.to_string());
        }
    }
}
