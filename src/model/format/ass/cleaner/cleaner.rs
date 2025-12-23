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
        self.parser.get_line_key(line)
    }

    pub fn should_skip_line(&self, line: &str) -> ParseRes<bool> {
        let is_special = self.parser.is_special_style(line)?;
        let is_empty = self.parser.is_text_empty(line)?;
        let too_many_tags = self.parser.has_excessive_tags(line);
        let is_technical_garbage = self.parser.is_technical_garbage(line)?;
        Ok(is_special || is_empty || too_many_tags || is_technical_garbage)
    }

    fn keep_increment(&self, i: &mut usize) -> bool {
        *i += 1;
        true
    }

    fn discard_remove(&self, lines: &mut Vec<String>, i: usize) -> bool {
        lines.remove(i);
        false
    }

    fn update_last_key(&self, last_key: &mut Option<String>, key: String) -> bool {
        *last_key = Some(key);
        true
    }

    fn discard_line(&self) -> ParseRes<bool> {
        Ok(false)
    }

    fn process_duplicate(&self, line: &str, last_key: &mut Option<String>) -> ParseRes<bool> {
        let key = self.get_key(line)?;
        Ok(self.handle_duplicate(key, last_key))
    }

    fn evaluate_skip_decision(&self, line: &str, last_key: &mut Option<String>) -> ParseRes<bool> {
        match self.should_skip_line(line)? {
            true => self.discard_line(),
            false => self.process_duplicate(line, last_key),
        }
    }

    fn handle_duplicate(&self, key: String, last_key: &mut Option<String>) -> bool {
        match Some(&key) == last_key.as_ref() {
            true => false,
            false => self.update_last_key(last_key, key),
        }
    }

    fn should_keep_line(&self, line: &str, last_key: &mut Option<String>) -> ParseRes<bool> {
        match self.parser.is_dialogue(line) {
            false => self.discard_line(),
            true => self.evaluate_skip_decision(line, last_key),
        }
    }

    fn handle_line(
        &self,
        lines: &mut Vec<String>,
        i: &mut usize,
        last_key: &mut Option<String>,
    ) -> ParseRes<()> {
        match self.should_keep_line(&lines[*i], last_key)? {
            true => self.keep_increment(i),
            false => self.discard_remove(lines, *i),
        };
        Ok(())
    }

    pub fn run(&mut self, lines: &mut Vec<String>) -> ParseRes<()> {
        let mut last_key: Option<String> = None;
        let mut i = 0;
        while i < lines.len() {
            self.handle_line(lines, &mut i, &mut last_key)?;
        }
        Ok(())
    }
}
