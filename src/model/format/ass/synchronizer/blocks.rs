use std::collections::VecDeque;

use crate::model::format::ass::{
    parser::{parser::Parser, parser_error::ParseRes},
    synchronizer::{block::Block, block_state::BlockState},
};

pub struct Blocks<'a> {
    queue: VecDeque<Block<'a>>,
}

impl<'a> Blocks<'a> {
    pub fn new(lines: &Vec<&'a String>, parser: &'a Parser) -> ParseRes<Self> {
        let queue = Self::analyze_blocks(parser, lines)?;
        Ok(Self { queue })
    }

    fn process_lines(
        lines: &Vec<&String>,
        state: &mut BlockState,
        parser: &Parser,
    ) -> ParseRes<()> {
        for (i, line) in lines.iter().enumerate() {
            match parser.is_scene_line(line)? {
                true => state.update_start(i),
                false => state.end_block(i),
            }
        }
        Ok(())
    }

    fn get_block_indices(lines: &Vec<&String>, parser: &Parser) -> ParseRes<Vec<(usize, usize)>> {
        let mut state = BlockState::new();
        Self::process_lines(lines, &mut state, parser)?;
        state.close_final(lines.len());
        Ok(state.into_ranges())
    }

    fn analyze_blocks(parser: &Parser, lines: &Vec<&'a String>) -> ParseRes<VecDeque<Block<'a>>> {
        let mut queue = VecDeque::new();
        let indices = Self::get_block_indices(lines, parser)?;
        for (start, end) in indices {
            queue.push_back(Self::create_block(parser, lines, start, end)?);
        }
        Ok(queue)
    }

    fn create_block<'b>(
        parser: &Parser,
        lines: &Vec<&'b String>,
        start: usize,
        end: usize,
    ) -> ParseRes<Block<'b>> {
        let additional = Self::get_lines(lines, start, end);
        let previous_normal = Self::find_previous_normal(lines, start, parser)?;
        let next_normal = Self::find_next_normal(lines, end, parser)?;
        Ok(Block::new(previous_normal, next_normal, additional))
    }

    fn get_lines<'b>(lines: &Vec<&'b String>, start: usize, end: usize) -> Vec<&'b String> {
        lines[start..=end].to_vec()
    }

    fn find_previous_normal<'b>(
        lines: &Vec<&'b String>,
        index: usize,
        parser: &Parser,
    ) -> ParseRes<Option<&'b String>> {
        for line in lines[..index].iter().rev() {
            if !parser.is_scene_line(line)? {
                return Ok(Some(line));
            }
        }
        Ok(None)
    }

    fn find_next_normal<'b>(
        lines: &Vec<&'b String>,
        end: usize,
        parser: &Parser,
    ) -> ParseRes<Option<&'b String>> {
        for line in lines.iter().skip(end + 1) {
            if !parser.is_scene_line(line)? {
                return Ok(Some(line));
            }
        }
        Ok(None)
    }

    pub fn peek_first_block(&'_ self) -> Block<'_> {
        self.queue.front().unwrap().clone()
    }

    pub fn pop_first_block(&mut self) -> Option<Block<'a>> {
        self.queue.pop_front()
    }

    pub fn reinsert_first_block(&mut self, block: Block<'a>) {
        self.queue.push_front(block);
    }

    pub fn has_blocks(&self) -> bool {
        !self.queue.is_empty()
    }

    pub fn consume_first_block(&mut self) {
        self.queue.pop_front();
    }
}
