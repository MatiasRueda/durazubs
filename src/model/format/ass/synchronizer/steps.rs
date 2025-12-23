pub struct Steps<'a> {
    current_line: &'a String,
    next_line: Option<&'a String>,
}

impl<'a> Steps<'a> {
    pub fn new(lines: &'a [&'a String], index: usize) -> Self {
        Self {
            current_line: lines[index],
            next_line: lines.get(index + 1).copied(),
        }
    }

    pub fn current_line(&self) -> &'a String {
        self.current_line
    }

    pub fn next_line(&self) -> Option<&'a String> {
        self.next_line
    }
}
