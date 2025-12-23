pub trait Writer {
    fn write_line(&mut self, line: &str);
    fn write_lines<'a, I>(&mut self, lines: I)
    where
        I: IntoIterator<Item = &'a String>,
    {
        for l in lines {
            self.write_line(l);
        }
    }
}
