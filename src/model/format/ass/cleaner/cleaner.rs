use crate::model::format::ass::parser::{parser::Parser, parser_error::ParseRes};

pub struct Cleaner {
    parser: Parser,
}

impl Cleaner {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }

    fn get_key(&self, line: &str) -> ParseRes<String> {
        Ok(self.parser.get_line_key(line)?)
    }

    pub fn should_skip_line(&self, line: &str) -> ParseRes<bool> {
        let is_special = self.parser.is_special_style(line)?;
        let is_empty = self.parser.is_text_empty(line)?;
        let too_many_tags = self.parser.has_excessive_tags(line);
        let is_technical_garbage = self.parser.is_technical_garbage(line)?;

        Ok(is_special || is_empty || too_many_tags || is_technical_garbage)
    }

    fn is_duplicate(&self, key: &String, last_key: &Option<String>) -> bool {
        Some(key) == last_key.as_ref()
    }

    fn update_last_and_keep(&self, last_key: &mut Option<String>, key: String) -> bool {
        *last_key = Some(key);
        true
    }

    fn handle_duplicate(&self, key: String, last_key: &mut Option<String>) -> bool {
        match self.is_duplicate(&key, last_key) {
            true => false,
            false => self.update_last_and_keep(last_key, key),
        }
    }

    fn process_key(&self, line: &str, last_key: &mut Option<String>) -> bool {
        match self.get_key(line) {
            Ok(key) => self.handle_duplicate(key, last_key),
            Err(_) => false,
        }
    }

    fn evaluate_skip(&self, line: &str, last_key: &mut Option<String>) -> bool {
        match self.should_skip_line(line) {
            Ok(true) => false,
            Ok(false) => self.process_key(line, last_key),
            Err(_) => false,
        }
    }

    fn should_keep_line(&self, line: &str, last_key: &mut Option<String>) -> bool {
        match self.parser.is_dialogue(line) {
            false => false,
            true => self.evaluate_skip(line, last_key),
        }
    }

    fn process_lines(&mut self, lines: &mut Vec<String>) {
        let mut last_key: Option<String> = None;
        lines.retain(|line| self.should_keep_line(line, &mut last_key));
    }

    pub fn run(&mut self, lines: &mut Vec<String>) {
        self.process_lines(lines);
    }
}
