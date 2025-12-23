use crate::model::format::ass::parser::parser::Parser;

type LinesVec<'a> = Vec<&'a String>;

pub struct AssSegmenter;

impl AssSegmenter {
    pub fn new() -> Self {
        Self
    }

    pub fn extract_header<'a>(&self, lines: &LinesVec<'a>, p: &Parser) -> Vec<&'a String> {
        let mut header = Vec::new();
        for line in lines.iter() {
            if p.is_dialogue(line) {
                break;
            }
            header.push(*line);
        }
        header
    }

    pub fn extract_dialogues<'a>(&self, lines: &LinesVec<'a>, p: &Parser) -> Vec<&'a String> {
        match lines.iter().position(|line| p.is_dialogue(line)) {
            Some(idx) => lines[idx..].to_vec(),
            None => Vec::new(),
        }
    }
}
