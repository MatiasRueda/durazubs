use crate::model::format::ass::{
    parser::{parser::Parser, parser_error::ParseRes},
    stylist::style_strategy::StyleStrategy,
};

pub struct MainStrategy {
    parser: Parser,
}

impl MainStrategy {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }
}

impl StyleStrategy for MainStrategy {
    fn apply_style(&self, line: &str) -> ParseRes<String> {
        let style = "Main".to_string();
        Ok(self.parser.replace_style(line, &style)?)
    }

    fn styles(&self) -> Vec<String> {
        vec![
            "Style: Main,Trebuchet MS,24,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1".to_string(),
        ]
    }
}
