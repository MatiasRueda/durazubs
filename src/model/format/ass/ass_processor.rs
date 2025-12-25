use crate::model::{
    format::ass::{
        applier::SceneApplier,
        cleaner::Cleaner,
        extractor::SceneExtractor,
        parser::parser_error::ParserError,
        sorter::Sorter,
        stylist::{Stylist, style_type::StyleType},
        synchronizer::Synchronizer,
    },
    subtitle_processor::{ProcRes, SubtitleProcessor},
    translator::instructor::Instructor,
};

pub struct AssProcessor {
    style_type: Option<StyleType>,
}

impl AssProcessor {
    pub fn new() -> Self {
        Self { style_type: None }
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
}

impl SubtitleProcessor for AssProcessor {
    type Error = ParserError;

    fn synchronize(
        &mut self,
        l_a: &mut Vec<String>,
        l_b: &[String],
    ) -> ProcRes<Vec<String>, Self::Error> {
        Cleaner::new().run(l_a)?;
        let sorted = Sorter::new().run(l_a)?;
        Synchronizer::new().run(&sorted, l_b)
    }

    fn get_lines_to_translate(&self, lines: &mut Vec<String>) -> ProcRes<Vec<String>, Self::Error> {
        Cleaner::new().run(lines)?;
        let ordered = Sorter::new().run(lines)?;
        let scenes = SceneExtractor::new().run(&ordered)?;
        Ok(Instructor::new().run(&scenes))
    }

    fn apply_translation(
        &mut self,
        lines: &mut Vec<String>,
        translations: Vec<String>,
    ) -> ProcRes<Vec<String>, Self::Error> {
        SceneApplier::new().run(lines, &translations)
    }

    fn apply_style(&mut self, lines: &mut Vec<String>) -> ProcRes<Vec<String>, Self::Error> {
        let style = self.style_type.as_ref().unwrap_or(&StyleType::Main);
        Stylist::new(style).run(lines)
    }
}
