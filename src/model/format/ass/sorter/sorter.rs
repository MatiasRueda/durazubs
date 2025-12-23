use crate::model::format::ass::{
    ass_segmenter::AssSegmenter,
    line_collector::LineCollector,
    parser::{parser::Parser, parser_error::ParseRes},
};

type LinesVec<'a> = Vec<&'a String>;
type TimedDialogues<'a> = Vec<(f64, &'a String)>;

pub struct Sorter {
    parser: Parser,
    segmenter: AssSegmenter,
}

impl Sorter {
    const MILLISECONDS_PER_SECOND: f64 = 1000.0;

    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            segmenter: AssSegmenter::new(),
        }
    }

    fn push_timed<'a>(&self, line: &'a String, to: &mut TimedDialogues<'a>) -> ParseRes<()> {
        let start_time = self.parser.get_start_time(line)?;
        to.push((start_time, line));
        Ok(())
    }

    fn dialogue_sort_key(event: &(f64, &String)) -> u64 {
        (event.0 * Self::MILLISECONDS_PER_SECOND) as u64
    }

    fn sort_dialogues(&self, timed_dialogues: &mut TimedDialogues) {
        timed_dialogues.sort_by_key(Self::dialogue_sort_key);
    }

    fn process<'a>(&self, lines: &LinesVec<'a>, timed: &mut TimedDialogues<'a>) -> ParseRes<()> {
        for line in lines {
            self.push_timed(line, timed)?;
        }
        Ok(())
    }

    pub fn run(&self, lines: &[String]) -> ParseRes<Vec<String>> {
        let mut output_lines = Vec::new();
        let lines_ref: Vec<&String> = lines.iter().collect();
        let mut timed_dialogues: TimedDialogues = Vec::new();
        let header_lines = self.segmenter.extract_header(&lines_ref, &self.parser);
        let mut dialogue_lines = self.segmenter.extract_dialogues(&lines_ref, &self.parser);
        self.process(&mut dialogue_lines, &mut timed_dialogues)?;
        self.sort_dialogues(&mut timed_dialogues);
        let mut collector = LineCollector::new(&mut output_lines);
        collector.collect_all(&header_lines, &timed_dialogues);
        Ok(output_lines)
    }
}
