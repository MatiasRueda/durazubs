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
            .expect("Fatal error reading input");
        input_string.trim().to_string()
    }

    pub fn show_welcome(&self) {
        println!("\n┌──────────────────────────────────────────────────┐");
        println!("│         DURAZUBS SUBTITLE PROCESSOR v1.0         │");
        println!("└──────────────────────────────────────────────────┘");
    }

    pub fn show_app_description(&self) {
        println!("\n┌──────────────────────────────────────────────────┐");
        println!("│                WORKFLOW & LOGIC                  │");
        println!("├──────────────────────────────────────────────────┤");
        println!("│ 1. INPUT A:    File with correct TIMESTAMPS      │");
        println!("│ 2. INPUT B:    File with the desired TEXTS       │");
        println!("│ 3. OUTPUT:     Name for the merged result        │");
        println!("│                                                  │");
        println!("│ 4. STYLING (Optional):                           │");
        println!("│    Apply custom visual profiles and font         │");
        println!("│    styles to the final output file.              │");
        println!("│                                                  │");
        println!("│ 5. AI ENGINE (Optional):                         │");
        println!("│    Ideal for Blu-Ray extended scenes. If a line  │");
        println!("│    exists in 'A' but is missing in 'B', AI will  │");
        println!("│    translate it using Llama3 or Cloud Mode.      │");
        println!("│                                                  │");
        println!("│ 6. MERGE:      Final sync and file export        │");
        println!("└──────────────────────────────────────────────────┘");
    }

    fn show_step(&self, label: &str, message: &str) {
        println!("[{:^8}] {}", label, message);
    }

    pub fn reading_step(&self) {
        self.show_step("START", "Initializing file streams...");
    }

    pub fn processing_step(&self) {
        self.show_step("WORK", "Applying transformations...");
    }

    pub fn writing_step(&self) {
        self.show_step("EXPORT", "Saving output to disk...");
    }

    pub fn show_translation_load_success(&self) {
        self.show_step("SUCCESS", "Translations synchronized.");
    }

    pub fn show_error(&self, error: &str) {
        eprintln!("\n[!] ERROR: {}\n", error);
    }

    pub fn show_translation_read_error(&self, path: &str) {
        self.show_error(&format!("Source '{}' not found or inaccessible.", path));
    }

    pub fn file_name_error(&self) {
        println!("Invalid name: reserved for translation buffer.");
    }

    fn select_option(&self, title: &str, options: &[&str]) -> String {
        loop {
            println!("\n❯ {}", title);
            for (index, option) in options.iter().enumerate() {
                println!("  {}. {}", index + 1, option);
            }
            print!("  Select choice [1-{}]: ", options.len());
            let input = self.read_input();
            if let Ok(value) = input.parse::<usize>() {
                if value > 0 && value <= options.len() {
                    self.line_break();
                    return input;
                }
            }
            println!("  '{}' is not a valid option.", input);
        }
    }

    pub fn line_break(&self) {
        print!("\n");
    }

    pub fn request_format(&self) -> String {
        self.select_option("Subtitle Format", &["ASS (Advanced SSA)", "SRT (SubRip)"])
    }

    pub fn request_coloring(&self) -> bool {
        self.request_confirmation("Apply custom styling?")
    }

    pub fn request_style_type(&self) -> String {
        self.select_option("Style Profile", &["Main", "Second"])
    }

    pub fn request_scene_translation(&self) -> bool {
        self.request_confirmation("Enable scene translation engine?")
    }

    pub fn request_translation_type(&self) -> String {
        self.select_option(
            "Translation Engine",
            &["Local AI (Direct)", "Cloud AI (Export Mode)"],
        )
    }

    pub fn request_file_name(&self) -> String {
        loop {
            print!("❯ Enter export filename (no extension): ");
            let name = self.read_input();
            if !name.is_empty() {
                return format!("{}.txt", name);
            }
            println!("  Filename is required.");
        }
    }

    pub fn show_translation_instructions(&self) {
        println!("\n┌──────────────────────────────────────────────────┐");
        println!("│             AI EXPORT INSTRUCTIONS               │");
        println!("├──────────────────────────────────────────────────┤");
        println!("│ 1. Copy the generated export file content.       │");
        println!("│ 2. Process via AI and save as 'translations.txt'.│");
        println!("│ 3. Place 'translations.txt' in the root folder.  │");
        println!("└──────────────────────────────────────────────────┘");
    }

    pub fn wait_for_input(&self) {
        print!("\n  Press Enter to synchronize with 'translations.txt'...");
        self.read_input();
        self.line_break();
    }

    pub fn request_confirmation(&self, message: &str) -> bool {
        print!("❯ {} (y/n): ", message);
        let result = self.read_input().to_lowercase() == "y";
        result
    }

    pub fn show_success(&self, duration_secs: f64) {
        let total_secs = duration_secs.round() as u64;
        let mins = total_secs / 60;
        let secs = total_secs % 60;

        println!("\n┌──────────────────────────────────────────────────┐");
        println!("│           STATUS: Deployment Complete            │");
        print!("│           TIME: ");
        match (mins, secs) {
            (0, s) => print!("{:>2} seconds", s),
            (m, 0) => print!("{:>2} minutes", m),
            (m, s) => print!("{}m {}s", m, s),
        }
        println!("                       │");
        println!("└──────────────────────────────────────────────────┘\n");
    }

    pub fn request_path_a(&self) -> String {
        loop {
            print!("❯ Enter path for file A (e.g. 'input'): ");
            let name = self.read_input();
            match name.is_empty() {
                false => return format!("{}.ass", name),
                true => println!("  Path is required."),
            }
        }
    }

    pub fn request_path_b(&self) -> String {
        loop {
            print!("❯ Enter path for file B (e.g. 'input'): ");
            let name = self.read_input();
            match name.is_empty() {
                false => return format!("{}.ass", name),
                true => println!("  Path is required."),
            }
        }
    }

    pub fn request_output_path(&self) -> String {
        loop {
            print!("❯ Enter name for the RESULT file: ");
            let name = self.read_input();
            if !name.is_empty() {
                return format!("{}.ass", name);
            }
            println!("  Output name is required.");
        }
    }
}
