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

    fn handle_duplicate(&self, key: String, last_key: &mut Option<String>) -> bool {
        if Some(&key) == last_key.as_ref() {
            false
        } else {
            *last_key = Some(key);
            true
        }
    }

    fn should_keep_line(&self, line: &str, last_key: &mut Option<String>) -> ParseRes<bool> {
        if !self.parser.is_dialogue(line) {
            return Ok(false);
        }
        if self.should_skip_line(line)? {
            return Ok(false);
        }
        let key = self.get_key(line)?;
        Ok(self.handle_duplicate(key, last_key))
    }

    pub fn run(&mut self, lines: &mut Vec<String>) -> ParseRes<()> {
        let mut last_key: Option<String> = None;
        let mut i = 0;
        while i < lines.len() {
            if self.should_keep_line(&lines[i], &mut last_key)? {
                i += 1;
            } else {
                lines.remove(i);
            }
        }
        Ok(())
    }
}
