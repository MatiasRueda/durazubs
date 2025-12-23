pub struct LineCollector<'a> {
    collected: &'a mut Vec<String>,
}

impl<'a> LineCollector<'a> {
    pub fn new(collected: &'a mut Vec<String>) -> Self {
        Self { collected }
    }

    fn add_header_lines(&mut self, header: &[&String]) {
        for line in header {
            self.collected.push((*line).clone());
        }
    }

    fn add_string_lines(&mut self, lines: &[String]) {
        for line in lines {
            self.collected.push(line.clone());
        }
    }

    fn add_timed_lines(&mut self, timed_lines: &[(f64, &String)]) {
        for (_, line) in timed_lines {
            self.collected.push((*line).clone());
        }
    }

    pub fn collect_all(&mut self, header: &[&String], dialogue: &[(f64, &String)]) {
        self.collected.reserve(header.len() + dialogue.len());
        self.add_header_lines(header);
        self.add_timed_lines(dialogue);
    }

    pub fn collect_header_and_lines(&mut self, header: &[&String], lines: &[String]) {
        self.collected.reserve(header.len() + lines.len());
        self.add_header_lines(header);
        self.add_string_lines(lines);
    }
}
