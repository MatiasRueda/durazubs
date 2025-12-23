use crate::model::format::ass::{
    ass_segmenter::AssSegmenter,
    line_collector::LineCollector,
    parser::{parser::Parser, parser_error::ParseRes},
    synchronizer::line_processor::LineProcessor,
};

pub struct Synchronizer {
    segmenter: AssSegmenter,
    parser: Parser,
}

impl Synchronizer {
    pub fn new() -> Self {
        Self {
            segmenter: AssSegmenter::new(),
            parser: Parser::new(),
        }
    }

    fn process_dialogues(&self, lines_a: &[&String], lines_b: &[&String]) -> ParseRes<Vec<String>> {
        let dialogues_a = self
            .segmenter
            .extract_dialogues(&lines_a.to_vec(), &self.parser);
        let dialogues_b = self
            .segmenter
            .extract_dialogues(&lines_b.to_vec(), &self.parser);

        let mut output_b_lines = Vec::new();
        let mut processor = LineProcessor::new(&mut output_b_lines, &self.parser);

        processor.run(&dialogues_a, &dialogues_b)?;
        Ok(output_b_lines)
    }

    fn collect(&self, lines_b: &[&String], processed_b: &[String], output: &mut Vec<String>) {
        let header_b = self
            .segmenter
            .extract_header(&lines_b.to_vec(), &self.parser);
        let mut collector = LineCollector::new(output);

        collector.collect_header_and_lines(&header_b, processed_b);
    }

    pub fn run(&mut self, source_a: &[String], source_b: &[String]) -> ParseRes<Vec<String>> {
        let mut final_output = Vec::new();

        let refs_a: Vec<&String> = source_a.iter().collect();
        let refs_b: Vec<&String> = source_b.iter().collect();

        let processed_b_lines = self.process_dialogues(&refs_a, &refs_b)?;
        self.collect(&refs_b, &processed_b_lines, &mut final_output);

        Ok(final_output)
    }
}
