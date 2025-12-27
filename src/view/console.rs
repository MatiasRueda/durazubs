use super::*;
use std::io::{self, Write};

pub struct Console;

impl Console {
    pub fn new() -> Self {
        Self
    }

    fn show_app_description(&self) {
        println!("\n┌──────────────────────────────────────────────────┐");
        println!("│                PROCESSOR WORKFLOW                │");
        println!("├──────────────────────────────────────────────────┤");
        println!("│ 1. SYNCHRONIZATION:                              │");
        println!("│    Merges TEXTS (File A) with TIMESTAMPS         │");
        println!("│    (File B) to create a perfectly synced file.   │");
        println!("│                                                  │");
        println!("│ 2. TRANSLATION ENGINE:                           │");
        println!("│    Ideal for Blu-Ray extended scenes. AI will    │");
        println!("│    translate lines that exist in 'A' but are     │");
        println!("│    missing in 'B' (Local or External AI).        │");
        println!("│                                                  │");
        println!("│ 3. STYLE PROFILES:                               │");
        println!("│    Apply custom visual formats, fonts, and       │");
        println!("│    styles to the final output subtitles.         │");
        println!("│                                                  │");
        println!("│ 4. SUBTITLE CONSOLIDATION:                       │");
        println!("│    Integrates all layers and transformations     │");
        println!("│    into a single production-ready output file.   │");
        println!("└──────────────────────────────────────────────────┘")
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
}

impl View for Console {
    fn get_format(&self) -> String {
        match self.select_option("Format", &["ASS", "SRT"]).as_str() {
            "1" => "ass".to_string(),
            _ => "srt".to_string(),
        }
    }

    fn request_path_a(&self, ext: &str) -> String {
        loop {
            print!("\n❯ Enter path for file A (Texts): ");
            let name = self.read_input();
            if !name.is_empty() {
                return format!("{}.{}", name, ext);
            }
            println!("  [!] Required field. Please enter the name for file A.");
        }
    }

    fn request_path_b(&self, ext: &str) -> String {
        loop {
            print!("❯ Enter path for file B (Timestamps): ");
            let name = self.read_input();
            if !name.is_empty() {
                return format!("{}.{}", name, ext);
            }
            println!("  [!] Required field. Please enter the name for file B.");
        }
    }

    fn request_path_result(&self, ext: &str) -> String {
        loop {
            print!("❯ Enter path for result file: ");
            let name = self.read_input();
            if !name.is_empty() {
                return format!("{}.{}", name, ext);
            }
            println!("  [!] Required field. Please enter a name for the output.");
        }
    }

    fn display_status(&self, status: AppStatus) {
        match status {
            AppStatus::Welcome => {
                println!("\n┌──────────────────────────────────────────────────┐");
                println!("│         DURAZUBS SUBTITLE PROCESSOR v1.0         │");
                println!("└──────────────────────────────────────────────────┘");
                self.show_app_description();
            }
            AppStatus::Reading => println!("\n[   START    ] Initializing file streams..."),
            AppStatus::ReadingA => {
                println!("[    READ    ] Extracting DIALOGUE TEXTS from Source (A)...")
            }
            AppStatus::ReadingB => {
                println!("[    READ    ] Extracting TIMESTAMPS from Source (B)...")
            }
            AppStatus::Preprocessing => {
                println!("[    CLEAN   ] Sorting and normalizing Source (B) timestamps...")
            }
            AppStatus::Processing => println!("[    WORK    ] Synchronizing subtitle layers..."),
            AppStatus::Translating => println!("[ TRANSLATE  ] Running translation engine..."),
            AppStatus::NoLinesToTranslate => {
                println!("[    INFO    ] No missing lines detected; skipping translation step.")
            }
            AppStatus::Styling => println!("[   STYLE    ] Applying visual profiles..."),
            AppStatus::Writing => println!("[   EXPORT   ] Saving output to disk..."),
            AppStatus::AskTranslation => {
                println!("[  AI-TASK   ] Process via AI and save as 'translations.txt'.\n")
            }
            AppStatus::TranslationFileFound => {
                println!("\n[     OK     ] 'translations.txt' found and loaded successfully.")
            }
            AppStatus::Success => println!("[  SUCCESS   ] Process completed successfully!\n"),
        }
    }

    fn get_sync_enabled(&self) -> bool {
        print!("\n❯ Enable synchronization (Merge File A text with File B timestamps)? (y/n): ");
        self.read_input().to_lowercase() == "y"
    }

    fn get_options(&self, output_path: &str, ext: &str, sync_enabled: bool) -> AppOptions {
        let mut translation_enabled = false;
        let mut ai_type = None;
        print!("\n❯ Enable translation engine? (y/n): ");
        if self.read_input().to_lowercase() == "y" {
            translation_enabled = true;
            ai_type =
                Some(self.select_option("Translation Engine Type", &["Local AI", "External AI"]));
        }
        let mut style = None;
        print!("\n❯ Apply custom styling? (y/n): ");
        if self.read_input().to_lowercase() == "y" {
            style = Some(self.select_option("Style Profile", &["Main", "Second"]));
        }
        AppOptions {
            output_path: output_path.to_string(),
            format_type: ext.to_string(),
            sync_enabled,
            style,
            translation_enabled,
            ai_type,
        }
    }

    fn confirm_translation_ready(&self) -> bool {
        print!("  Press Enter to sync with 'translations.txt' (or 'n' to cancel)...");
        self.read_input().to_lowercase() != "n"
    }

    fn display_error(&self, message: &str) {
        eprintln!("\n[   ERROR    ] {}\n", message);
    }
}
