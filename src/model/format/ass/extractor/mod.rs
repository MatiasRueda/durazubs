use crate::model::format::ass::{
    extractor::additional_scene_extractor::AdditionalSceneExtractor, parser::parser_error::ParseRes,
};

mod additional_scene_extractor;

pub struct SceneExtractor {
    extractor: AdditionalSceneExtractor,
}

impl SceneExtractor {
    pub fn new() -> Self {
        Self {
            extractor: AdditionalSceneExtractor::new(),
        }
    }

    pub fn run(&mut self, lines: &[String]) -> ParseRes<Vec<String>> {
        let mut output = Vec::new();
        self.extractor.run(lines, &mut output)?;
        Ok(output)
    }
}

#[cfg(test)]
mod tests;
