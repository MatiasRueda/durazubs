use crate::model::format::ass::{
    parser::{parser::Parser, parser_error::ParseRes},
    synchronizer::{block::Block, blocks::Blocks, steps::Steps},
};

pub struct LineProcessor<'a> {
    delta: f64,
    output_lines: &'a mut Vec<String>,
    parser: &'a Parser,
}

impl<'a> LineProcessor<'a> {
    const INITIAL_OFFSET: usize = 0;
    const MAX_DIFFERENCE: f64 = 1.0;

    pub fn new(output_lines: &'a mut Vec<String>, parser: &'a Parser) -> Self {
        Self {
            delta: 0.0,
            output_lines,
            parser,
        }
    }

    fn add_line(&mut self, line: &str) {
        self.output_lines.push(line.to_string());
    }

    fn process_line(
        &mut self,
        blocks: &mut Blocks,
        lines_a: &Vec<&String>,
        idx_a: usize,
    ) -> ParseRes<()> {
        let step = Steps::new(lines_a, idx_a);
        if !self.try_insert_block(blocks, &step)? {
            self.add_corrected_line(step.current_line())?;
        }
        Ok(())
    }

    fn try_insert_block(&mut self, blocks: &mut Blocks, step: &Steps) -> ParseRes<bool> {
        match blocks.has_blocks() {
            true => Ok(self.process_block(blocks, step)?),
            false => Ok(false),
        }
    }

    fn update_start_time(&self, line: &str) -> ParseRes<String> {
        let mut start_time = self.parser.get_start_time(line)?;
        start_time += self.delta;
        Ok(self.parser.set_start_time(line, start_time)?)
    }

    fn update_end_time(&self, line: &str) -> ParseRes<String> {
        let mut end_time = self.parser.get_end_time(line)?;
        end_time += self.delta;
        Ok(self.parser.set_end_time(line, end_time)?)
    }

    fn add_corrected_line(&mut self, line: &String) -> ParseRes<()> {
        let line_with_new_start = self.update_start_time(line)?;
        let fully_corrected_line = self.update_end_time(&line_with_new_start)?;
        self.add_line(&fully_corrected_line);
        Ok(())
    }

    fn is_difference_valid(&self, current_time: f64, previous_time: f64) -> bool {
        let calculated_diff = (current_time - previous_time).abs();
        calculated_diff < Self::MAX_DIFFERENCE
    }

    fn check_start_vs_previous(
        &self,
        step: &Steps,
        offset: f64,
        previous: &String,
    ) -> ParseRes<bool> {
        let current_line = step.current_line();
        let current_time = self.parser.get_start_time(current_line)? + offset;
        let previous_time = self.parser.get_start_time(previous)?;
        Ok(self.is_difference_valid(current_time, previous_time))
    }

    fn is_block_valid(&self, block: &Block, step: &Steps, offset: f64) -> ParseRes<bool> {
        match block.previous_line() {
            Some(previous) => Ok(self.check_start_vs_previous(step, offset, previous)?),
            None => Ok(false),
        }
    }

    fn add_additional_lines(&mut self, lines: &[&String]) {
        for line in lines {
            self.add_line(line);
        }
    }

    fn add_corrected_block(&mut self, block: &Block, step: &Steps) -> ParseRes<()> {
        self.add_corrected_line(step.current_line())?;
        self.add_additional_lines(block.additional_lines());
        Ok(())
    }

    fn update_delta(&mut self, next_line: &str, next_block_line: &str) -> ParseRes<()> {
        self.delta = self.parser.get_new_offset(next_line, next_block_line)?;
        Ok(())
    }

    fn execute_consumed_block(&mut self, block: Block, step: &Steps) -> ParseRes<()> {
        self.add_corrected_block(&block, step)?;
        if let Some(next) = step.next_line() {
            if let Some(next_block_line) = block.next_line() {
                self.update_delta(next, next_block_line)?;
            }
        }
        Ok(())
    }

    fn process_block(&mut self, blocks: &mut Blocks, step: &Steps) -> ParseRes<bool> {
        let block = match blocks.pop_first_block() {
            Some(b) => b,
            None => return Ok(false),
        };

        match self.is_block_valid(&block, step, self.delta)? {
            true => {
                self.execute_consumed_block(block, step)?;
                Ok(true)
            }
            false => {
                blocks.reinsert_first_block(block);
                Ok(false)
            }
        }
    }

    fn calculate_initial_delta(
        &self,
        dialogues_a: &Vec<&String>,
        dialogues_b: &Vec<&String>,
    ) -> ParseRes<f64> {
        let line_a = &dialogues_a[Self::INITIAL_OFFSET];
        let line_b = &dialogues_b[Self::INITIAL_OFFSET];
        let time_a = self.parser.get_start_time(line_a)?;
        let time_b = self.parser.get_start_time(line_b)?;
        Ok(time_b - time_a)
    }

    pub fn run(&mut self, dialogues_a: &Vec<&String>, dialogues_b: &Vec<&String>) -> ParseRes<()> {
        self.delta = self.calculate_initial_delta(dialogues_a, dialogues_b)?;
        let mut blocks = Blocks::new(dialogues_b, &self.parser)?;
        for idx_a in 0..dialogues_a.len() {
            self.process_line(&mut blocks, dialogues_a, idx_a)?;
        }
        Ok(())
    }
}
