use crate::model::{
    format::ass::{
        applier::scene_applier::SceneApplier,
        cleaner::cleaner::Cleaner,
        extractor::scene_extractor::SceneExtractor,
        parser::parser_error::{ParseRes, ParserError},
        sorter::sorter::Sorter,
        stylist::stylist::{style_type::StyleType, stylist::Stylist},
        synchronizer::synchronizer::Synchronizer,
    },
    subtitle_processor::{ProcRes, SubtitleProcessor},
    translator::{instructor::Instructor, translator::Translator},
};

pub struct AssProcessor {
    style_type: Option<StyleType>,
    translate: bool,
    external_translation: bool,
    external_subs: Vec<String>,
}

impl AssProcessor {
    pub fn new() -> Self {
        Self {
            style_type: None,
            translate: false,
            external_translation: false,
            external_subs: Vec::new(),
        }
    }

    fn identify_style(&self, s: &str) -> StyleType {
        match s {
            "1" => StyleType::Main,
            _ => StyleType::Second,
        }
    }

    pub fn with_style(mut self, style_name: Option<String>) -> Self {
        self.style_type = style_name.map(|s| self.identify_style(&s));
        self
    }

    pub fn with_translation(&mut self, value: bool) {
        self.translate = value;
    }

    pub fn translated_subtitles(&mut self, translations: Vec<String>) {
        self.external_translation = true;
        self.external_subs = translations;
    }

    fn apply_coloring(&mut self, lines: &Vec<String>) -> ParseRes<Vec<String>> {
        let style = self.style_type.as_ref().unwrap_or(&StyleType::Main);
        let stylist = Stylist::new(style);
        stylist.run(lines)
    }

    fn apply_translation(&mut self, lines: &Vec<String>) -> ParseRes<Vec<String>> {
        let mut extractor = SceneExtractor::new();
        let translator = Translator::new();
        let mut applier = SceneApplier::new();

        let additional_scenes = extractor.run(lines)?;
        let translated_lines = translator.run(&additional_scenes);

        applier.run(lines, &translated_lines)
    }

    fn execute(
        &mut self,
        lines_a: &mut Vec<String>,
        lines_b: &Vec<String>,
    ) -> ParseRes<Vec<String>> {
        let mut cleaner = Cleaner::new();
        let sorter = Sorter::new();
        let mut synchronizer = Synchronizer::new();
        cleaner.run(lines_a);
        let sortered_lines_a = sorter.run(lines_a)?;
        let mut result = synchronizer.run(&sortered_lines_a, lines_b)?;
        if self.translate && self.external_translation {
            let mut applier = SceneApplier::new();
            result = applier.run(&result, &self.external_subs)?;
        }
        if self.translate && !self.external_translation {
            result = self.apply_translation(&result)?;
        }
        if self.style_type.is_some() {
            result = self.apply_coloring(&result)?;
        }
        Ok(result)
    }

    fn extract_additional_scenes(&self, lines: &mut Vec<String>) -> ParseRes<Vec<String>> {
        let mut cleaner = Cleaner::new();
        let sorter = Sorter::new();
        let mut extractor = SceneExtractor::new();
        cleaner.run(lines);
        let ordered_lines = sorter.run(lines)?;
        let scenes = extractor.run(&ordered_lines)?;
        let instructor = Instructor::new();
        Ok(instructor.run(&scenes))
    }
}
impl SubtitleProcessor for AssProcessor {
    type Error = ParserError;

    fn process(
        &mut self,
        lines_a: &mut Vec<String>,
        lines_b: &Vec<String>,
    ) -> ProcRes<Vec<String>, Self::Error> {
        self.execute(lines_a, lines_b)
    }

    fn get_lines_to_translate(&self, lines: &mut Vec<String>) -> ProcRes<Vec<String>, Self::Error> {
        self.extract_additional_scenes(lines)
    }
}
