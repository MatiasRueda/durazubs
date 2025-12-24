use crate::model::format::ass::{
    parser::{parser::Parser, parser_error::ParseRes},
    stylist::style_strategy::StyleStrategy,
};

pub struct SecondStrategy {
    parser: Parser,
}

impl SecondStrategy {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }
}

impl StyleStrategy for SecondStrategy {
    fn apply_style(&self, line: &str) -> ParseRes<String> {
        let style = "Second".to_string();
        Ok(self.parser.replace_style(line, &style)?)
    }

    fn styles(&self) -> Vec<String> {
        vec![
            "Style: Second,Roboto,22,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1.5,2,10,10,10,1".to_string(),
        ]
    }
}
