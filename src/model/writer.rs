use crate::model::io::io_error::IOError;

pub trait Writer {
    fn write_line(&mut self, line: &str) -> Result<(), IOError>;
    fn write_lines<'a, I>(&mut self, lines: I) -> Result<(), IOError>
    where
        I: IntoIterator<Item = &'a String>,
    {
        for l in lines {
            self.write_line(l)?;
        }
        Ok(())
    }
}
