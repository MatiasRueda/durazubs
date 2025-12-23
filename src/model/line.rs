pub trait Line {
    fn start(&self) -> f64;
    fn end(&self) -> f64;
    fn text(&self) -> &str;
}
