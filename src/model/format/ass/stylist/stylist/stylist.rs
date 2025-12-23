use crate::model::format::ass::{
    parser::{parser::Parser, parser_error::ParseRes},
    stylist::{
        style_strategy::StyleStrategy,
        stylist::{
            script_preparer::ScriptPreparer, strategy_factory::StrategyFactory,
            style_editor::StyleEditor, style_type::StyleType,
        },
    },
};

pub struct Stylist {
    parser: Parser,
    strategy: Box<dyn StyleStrategy>,
}

impl Stylist {
    pub fn new(style_type: &StyleType) -> Self {
        Self {
            parser: Parser::new(),
            strategy: StrategyFactory::create(style_type),
        }
    }

    pub fn run(&self, lines: &[String]) -> ParseRes<Vec<String>> {
        let mut output_lines = Vec::new();
        let mut script_preparer = ScriptPreparer::new(&self.parser);
        script_preparer.analyze(lines, &mut output_lines)?;
        let mut style_editor = StyleEditor::new(&self.parser, &script_preparer, &*self.strategy);
        style_editor.edit(&mut output_lines)?;
        Ok(output_lines)
    }
}
