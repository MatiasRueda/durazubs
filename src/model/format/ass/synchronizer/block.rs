#[derive(Debug, Clone)]
pub struct Block<'a> {
    previous_normal: Option<&'a String>,
    next_normal: Option<&'a String>,
    additional_lines: Vec<&'a String>,
}

impl<'a> Block<'a> {
    pub fn new(
        previous_normal: Option<&'a String>,
        next_normal: Option<&'a String>,
        additional_lines: Vec<&'a String>,
    ) -> Self {
        Self {
            previous_normal,
            next_normal,
            additional_lines,
        }
    }

    pub fn previous_line(&self) -> Option<&'a String> {
        self.previous_normal
    }

    pub fn next_line(&self) -> Option<&'a String> {
        self.next_normal
    }

    pub fn additional_lines(&self) -> &Vec<&'a String> {
        &self.additional_lines
    }
}
