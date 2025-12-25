use super::*;
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
            .expect("Error reading input");
        input_string.trim().to_string()
    }

    fn select_option(&self, title: &str, options: &[&str]) -> String {
        loop {
            println!("\n❯ {}", title);
            for (i, opt) in options.iter().enumerate() {
                println!("  {}. {}", i + 1, opt);
            }
            print!("  Select choice [1-{}]: ", options.len());
            let input = self.read_input();
            if let Ok(val) = input.parse::<usize>() {
                if val > 0 && val <= options.len() {
                    return input;
                }
            }
            println!("  '{}' is not valid.", input);
        }
    }

    fn request_path(&self, label: &str) -> String {
        loop {
            print!("❯ Enter path for {} (e.g. 'input'): ", label);
            let name = self.read_input();
            if !name.is_empty() {
                return format!("{}.ass", name);
            }
            println!("  Required field.");
        }
    }
}

impl View for Console {
    fn display_status(&self, status: AppStatus) {
        match status {
            AppStatus::Welcome => {
                println!("\n┌──────────────────────────────────────────────────┐");
                println!("│         DURAZUBS SUBTITLE PROCESSOR v1.0         │");
                println!("└──────────────────────────────────────────────────┘");
            }
            AppStatus::Reading => println!("[ START  ] Initializing file streams..."),
            AppStatus::Processing => println!("[  WORK  ] Applying transformations..."),
            AppStatus::Writing => println!("[ EXPORT ] Saving output to disk..."),
            AppStatus::InstructionsForTranslation => {
                println!("\n[ AI EXPORT ] Process via AI and save as 'translations.txt'.");
            }
        }
    }

    fn get_config(&self) -> AppConfig {
        let path_a = self.request_path("file A");
        let path_b = self.request_path("file B");
        let output_path = self.request_path("RESULT file");
        let format_type = self.select_option("Format", &["ASS", "SRT"]);

        let mut style = None;
        print!("❯ Apply custom styling? (y/n): ");
        if self.read_input().to_lowercase() == "y" {
            style = Some(self.select_option("Style Profile", &["Main", "Second"]));
        }

        print!("❯ Enable translation engine? (y/n): ");
        let translation_enabled = self.read_input().to_lowercase() == "y";

        AppConfig {
            path_a,
            path_b,
            output_path,
            format_type,
            style,
            translation_enabled,
        }
    }

    fn confirm_translation_ready(&self) -> bool {
        print!("\n  Press Enter to sync with 'translations.txt' (or 'n' to cancel)...");
        self.read_input().to_lowercase() != "n"
    }

    fn display_error(&self, message: &str) {
        eprintln!("\n[!] ERROR: {}\n", message);
    }

    fn display_success(&self, seconds: f64) {
        println!("\n[ SUCCESS ] Completed in {:.2}s\n", seconds);
    }
}
