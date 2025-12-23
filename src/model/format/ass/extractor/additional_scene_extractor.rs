use crate::model::format::ass::parser::{parser::Parser, parser_error::ParseRes};

pub struct AdditionalSceneExtractor {
    parser: Parser,
}

impl AdditionalSceneExtractor {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }

    fn process_line(&mut self, line: &String, output: &mut Vec<String>) -> ParseRes<()> {
        if self.parser.is_scene_line(line)? {
            let text = self.parser.get_text(line)?;
            output.push(text.clone())
        }
        Ok(())
    }

    fn process_lines(&mut self, lines: &[String], output: &mut Vec<String>) -> ParseRes<()> {
        for line in lines {
            self.process_line(line, output)?;
        }
        Ok(())
    }

    pub fn run(&mut self, lines: &[String], output: &mut Vec<String>) -> ParseRes<()> {
        self.process_lines(lines, output)?;
        Ok(())
    }
}
