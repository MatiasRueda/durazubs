use crate::model::format::ass::parser::{parser::Parser, parser_error::ParseRes};

pub struct SceneExtractor {
    parser: Parser,
}

impl SceneExtractor {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }

    fn process_line(&mut self, line: &String, output: &mut Vec<String>) -> ParseRes<()> {
        match self.parser.is_scene_line(line)? {
            true => self.extract_and_push(line, output),
            false => Ok(()),
        }
    }

    fn extract_and_push(&mut self, line: &String, output: &mut Vec<String>) -> ParseRes<()> {
        let text = self.parser.get_text(line)?;
        output.push(text.clone());
        Ok(())
    }

    pub fn run(&mut self, lines: &[String]) -> ParseRes<Vec<String>> {
        let mut output = Vec::new();
        for line in lines {
            self.process_line(line, &mut output)?;
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests;
