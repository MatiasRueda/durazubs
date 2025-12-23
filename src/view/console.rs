use std::io::{self, Write};

pub struct Console;

impl Console {
    pub fn new() -> Self {
        Self
    }

    fn read_input(&self) -> String {
        io::stdout().flush().unwrap();
        let mut input_string = String::new();
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read from keyboard");
        input_string.trim().to_string()
    }

    pub fn file_name_error(&self) {
        println!("Cannot choose 'translations' as a filename");
    }

    pub fn show_translation_load_success(&self) {
        self.show_step("Translation file detected and loaded");
    }

    fn select_option(&self, title: &str, options: &[&str]) -> String {
        loop {
            println!("\n--- {} ---", title);
            for (index, option) in options.iter().enumerate() {
                println!("{}. {}", index + 1, option);
            }
            print!("Select an option: ");
            let input = self.read_input();
            if let Ok(value) = input.parse::<usize>() {
                if value > 0 && value <= options.len() {
                    return input;
                }
            }
            println!("Option '{}' is invalid. Please try again.", input);
        }
    }

    pub fn request_style_type(&self) -> String {
        self.select_option("Select style to apply", &["Main", "Second"])
    }

    pub fn request_translation_type(&self) -> String {
        self.select_option("Select translation engine", &["Local AI", "External AI"])
    }

    pub fn request_format(&self) -> String {
        self.select_option("Format Selection", &["ASS", "SRT"])
    }

    pub fn request_confirmation(&self, message: &str) -> bool {
        print!("{} (y/n): ", message);
        self.read_input().to_lowercase() == "y"
    }

    pub fn request_file_name(&self) -> String {
        loop {
            print!("Enter the filename: ");
            let name = self.read_input();
            if !name.is_empty() {
                return format!("{}.txt", name);
            }
            println!("Filename cannot be empty.");
        }
    }

    pub fn show_welcome(&self) {
        println!("--- WELCOME TO SUBTITLE PROCESSOR v1.0 ---");
    }

    fn show_step(&self, message: &str) {
        println!("[PROCESS] {}...", message);
    }

    pub fn reading_step(&self) {
        self.show_step("Reading files");
    }

    pub fn processing_step(&self) {
        self.show_step("Processing subtitles");
    }

    pub fn writing_step(&self) {
        self.show_step("Writing results");
    }

    pub fn show_translation_instructions(&self) {
        println!("\n[INFO] Instructions:");
        println!(
            "1. Open the generated file.\n2. Copy the content into the AI.\n3. Save the result as 'translations.txt'.\n"
        );
    }

    pub fn wait_for_input(&self) {
        println!("Press Enter once you have created the file...");
        self.read_input();
    }

    pub fn request_coloring(&self) -> bool {
        self.request_confirmation("Do you want to modify the style?")
    }

    pub fn request_scene_translation(&self) -> bool {
        self.request_confirmation("Do you want to translate additional scenes?")
    }

    pub fn show_error(&self, error: &str) {
        eprintln!("CRITICAL ERROR: {}", error);
    }

    pub fn show_translation_read_error(&self, path: &str) {
        self.show_error(&format!("Could not read '{}'", path));
    }

    pub fn show_success(&self, duration_secs: f64) {
        println!("--------------------------------------");
        println!("Finished successfully!");
        let total_secs = duration_secs.round() as u64;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        match (mins, secs) {
            (0, s) => println!("Total time: {} seconds", s),
            (m, 0) => println!("Total time: {} minutes", m),
            (m, s) => println!("Total time: {} min and {} sec", m, s),
        }
    }
}
