use std::process::Command;

pub struct Translator;

impl Translator {
    pub fn new() -> Self {
        Self
    }

    fn should_translate(&self, text: &str) -> bool {
        match self.heuristic(text) {
            Some(is_english) => is_english,
            None => self.is_english_llama(text),
        }
    }

    fn heuristic(&self, text: &str) -> Option<bool> {
        let t = text.to_lowercase();
        let english_words = [" the ", " and ", " you ", " is ", " are ", " i "];
        let spanish_chars = ['á', 'é', 'í', 'ó', 'ú', 'ñ', '¿', '¡'];

        if spanish_chars.iter().any(|c| t.contains(*c)) {
            return Some(false);
        }
        if english_words.iter().any(|w| t.contains(w)) {
            return Some(true);
        }

        None
    }

    fn is_english_llama(&self, text: &str) -> bool {
        let prompt = format!(
            "Detect if the following text is in English.\n\
             Respond only with true or false.\n\
             Text:\n\"{}\"",
            text
        );

        let output = Command::new("ollama")
            .args(["run", "llama3:8b", &prompt])
            .output()
            .expect("Error executing llama");

        let response = String::from_utf8_lossy(&output.stdout).to_lowercase();
        match response.trim() {
            "true" => true,
            "false" => false,
            r if r.contains("false") => false,
            r if r.contains("true") => true,
            _ => false,
        }
    }

    fn translate(&self, text: &str) -> String {
        let prompt = format!(
            "Translate this anime subtitle from English to Spanish.\n\
             Return only the translation, no quotes, no explanations, no extra content.\n\
             Text:\n\"{}\"",
            text
        );

        let output = Command::new("ollama")
            .args(["run", "llama3:8b", &prompt])
            .output()
            .expect("Error executing llama");

        let mut translation = String::from_utf8_lossy(&output.stdout).trim().to_string();

        translation = translation
            .trim_start_matches('"')
            .trim_end_matches('"')
            .to_string();

        translation
    }

    pub fn run(&self, lines: &Vec<String>) -> Vec<String> {
        let mut results = Vec::new();
        for line in lines {
            match self.should_translate(line) {
                true => results.push(self.translate(line)),
                false => results.push(line.clone()),
            }
        }
        results
    }
}
