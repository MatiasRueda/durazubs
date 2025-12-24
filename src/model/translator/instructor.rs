pub struct Instructor {}

impl Instructor {
    const LAST_ELEMENT_OFFSET: usize = 1;
    const CHUNK_SIZE: usize = 40;
    const EXTRA_ELEMENTS_PER_CHUNK: usize = 2;
    const INSTRUCTION: &str = "Act as an expert anime translator. You will process a block of subtitles from ENGLISH to NEUTRAL LATIN AMERICAN SPANISH.
    Strict rules:
    1. MAINTAIN FORMAT: The total number of output lines must be exactly equal to the input.
    2. DO NOT TRANSLATE OTHER LANGUAGES: If a line is in Japanese (romaji/kanji) or any other language, leave it identical.
    3. TONE: Natural and colloquial Latin American anime style.
    4. INTEGRITY: Do not omit any lines, even if they are empty or not in English.
    5. OUTPUT: Return the FULL RESULT (translated lines and preserved original lines) ONLY within a Markdown code block, without additional comments.\n";

    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, lines: &Vec<String>) -> Vec<String> {
        let chunk_count = (lines.len() as f64 / Self::CHUNK_SIZE as f64).ceil() as usize;
        let total_capacity = chunk_count * (Self::EXTRA_ELEMENTS_PER_CHUNK + Self::CHUNK_SIZE);
        let mut result = Vec::with_capacity(total_capacity);
        let chunks = lines.chunks(Self::CHUNK_SIZE);
        let total_chunks = chunks.len();
        for (i, chunk) in chunks.enumerate() {
            result.push(Self::INSTRUCTION.to_string());
            result.push("---".to_string());
            result.extend_from_slice(chunk);
            if i < total_chunks - Self::LAST_ELEMENT_OFFSET {
                result.push("\n".to_string());
            }
        }
        result
    }
}
