use crate::model::format::ass::{
    parser::{parser::Parser, parser_error::ParseRes},
    stylist::{script_preparer::ScriptPreparer, style_strategy::StyleStrategy},
};

pub struct StyleEditor<'a> {
    parser: &'a Parser,
    prep: &'a ScriptPreparer<'a>,
    strategy: &'a dyn StyleStrategy,
}

impl<'a> StyleEditor<'a> {
    pub fn new(
        parser: &'a Parser,
        prep: &'a ScriptPreparer,
        strategy: &'a dyn StyleStrategy,
    ) -> Self {
        Self {
            parser,
            prep,
            strategy,
        }
    }

    fn inject_styles(&self, out: &mut Vec<String>) {
        match self.prep.post_style_idx() {
            Some(idx) => self.insert_in_existing_section(out, idx),
            None => self.create_style_section(out),
        }
    }

    fn insert_in_existing_section(&self, out: &mut Vec<String>, idx: usize) {
        out.splice(idx + 1..idx + 1, self.strategy.styles());
    }

    fn create_style_section(&self, out: &mut Vec<String>) {
        let i = self.prep.post_info_idx(out);
        out.insert(i, String::new());
        out.insert(i + 1, self.parser.get_styles_tag());
        out.insert(i + 2, self.parser.get_styles_format());
        out.splice(i + 3..i + 3, self.strategy.styles());
    }

    fn proc_dialogue_line(&self, line: &mut String, target: &str) -> ParseRes<()> {
        let current_style = self.parser.get_style(line)?;
        let is_target = current_style == target;
        let is_scene = self.parser.is_scene_line(line)?;
        if is_target || is_scene {
            *line = self.strategy.apply_style(line)?;
        }
        Ok(())
    }

    fn transform_dialogues(&self, out: &mut Vec<String>, target: &str) -> ParseRes<()> {
        for line in out {
            if self.parser.is_dialogue(line) {
                self.proc_dialogue_line(line, target)?;
            }
        }
        Ok(())
    }

    fn apply_style(&self, out: &mut Vec<String>) -> ParseRes<()> {
        if let Some(dominant) = self.prep.dominant_style() {
            return self.transform_dialogues(out, &dominant);
        }
        Ok(())
    }

    pub fn edit(&mut self, out: &mut Vec<String>) -> ParseRes<()> {
        self.inject_styles(out);
        self.apply_style(out)?;
        Ok(())
    }
}
