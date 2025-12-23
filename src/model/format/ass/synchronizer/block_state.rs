pub struct BlockState {
    start: Option<usize>,
    ranges: Vec<(usize, usize)>,
}

impl BlockState {
    pub fn new() -> Self {
        Self {
            start: None,
            ranges: Vec::new(),
        }
    }

    pub fn update_start(&mut self, index: usize) {
        self.start.get_or_insert(index);
    }

    pub fn end_block(&mut self, index: usize) {
        if let Some(start) = self.start.take() {
            self.ranges.push((start, index - 1));
        }
    }

    pub fn close_final(&mut self, total: usize) {
        if let Some(start) = self.start.take() {
            self.ranges.push((start, total - 1));
        }
    }

    pub fn into_ranges(self) -> Vec<(usize, usize)> {
        self.ranges
    }
}
