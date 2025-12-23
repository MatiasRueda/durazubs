use crate::model::format::ass::parser::parser_error::ParseRes;

pub trait StyleStrategy {
    fn apply_style(&self, line: &str) -> ParseRes<String>;
    fn styles(&self) -> Vec<String>;
}
