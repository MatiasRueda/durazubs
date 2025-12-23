use crate::model::format::ass::parser::{parser::Parser, parser_error::ParseRes};

pub struct SceneApplier {
    parser: Parser,
    idx: usize,
}

impl SceneApplier {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            idx: 0,
        }
    }

    fn translate(&mut self, l: &str, ts: &[String]) -> ParseRes<String> {
        let res = self.parser.replace_text(l, &ts[self.idx])?;
        self.idx += 1;
        Ok(res)
    }

    fn keep(&self, l: &str) -> String {
        l.to_string()
    }

    fn proc_line(&mut self, l: &str, ts: &[String]) -> ParseRes<String> {
        match self.parser.is_scene_line(l)? && self.idx < ts.len() {
            true => self.translate(l, ts),
            false => Ok(self.keep(l)),
        }
    }

    fn proc(&mut self, ls: &[String], ts: &[String]) -> ParseRes<Vec<String>> {
        ls.iter().map(|l| self.proc_line(l, ts)).collect()
    }

    pub fn run(&mut self, ls: &[String], ts: &[String]) -> ParseRes<Vec<String>> {
        self.idx = 0;
        self.proc(ls, ts)
    }
}
