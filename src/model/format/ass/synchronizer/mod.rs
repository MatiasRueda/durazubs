use crate::model::format::ass::{
    ass_segmenter::AssSegmenter,
    line_collector::LineCollector,
    parser::{parser::Parser, parser_error::ParseRes},
    synchronizer::line_processor::LineProcessor,
};

mod block;
mod block_state;
mod blocks;
mod line_processor;
mod steps;

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

        let mut output_a_lines = Vec::new();
        let mut processor = LineProcessor::new(&mut output_a_lines, &self.parser);

        processor.run(&dialogues_a, &dialogues_b)?;
        Ok(output_a_lines)
    }

    fn collect(&self, lines_a: &[&String], processed_a: &[String], output: &mut Vec<String>) {
        let header_a = self
            .segmenter
            .extract_header(&lines_a.to_vec(), &self.parser);
        let mut collector = LineCollector::new(output);

        collector.collect_header_and_lines(&header_a, processed_a);
    }

    pub fn run(&mut self, source_a: &[String], source_b: &[String]) -> ParseRes<Vec<String>> {
        let mut final_output = Vec::new();

        let refs_a: Vec<&String> = source_a.iter().collect();
        let refs_b: Vec<&String> = source_b.iter().collect();

        let processed_a_lines = self.process_dialogues(&refs_a, &refs_b)?;
        self.collect(&refs_a, &processed_a_lines, &mut final_output);

        Ok(final_output)
    }
}

#[cfg(test)]
mod tests;
