use crate::model::line::Line;

#[repr(usize)]
pub enum AssField {
    Layer = 0,
    Start = 1,
    End = 2,
    Style = 3,
    Name = 4,
    Effect = 8,
    Text = 9,
}

pub const ASS_FIELDS_COUNT: usize = 10;

impl AssField {
    pub fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone)]
pub struct AssLine {
    pub layer: String,
    pub start: f64,
    pub end: f64,
    pub style: String,
    pub name: String,
    pub effect: String,
    pub text: String,
}

impl Line for AssLine {
    fn start(&self) -> f64 {
        self.start
    }
    fn end(&self) -> f64 {
        self.end
    }
    fn text(&self) -> &str {
        &self.text
    }
}
