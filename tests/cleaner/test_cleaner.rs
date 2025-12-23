#[cfg(test)]
mod tests {
    use durazubs::model::format::ass::cleaner::cleaner::Cleaner;

    struct TestCase {
        name: &'static str,
        input: &'static [&'static str],
        expected: &'static [&'static str],
    }

    static DUPLICATE_CASE: TestCase = TestCase {
        name: "removes basic duplicate lines",
        input: &[
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:01.00,0:00:04.00,Default,,0000,0000,0000,,Line A",
            "Dialogue: 0,0:00:01.00,0:00:04.00,Default,,0000,0000,0000,,Line A",
            "Dialogue: 0,0:00:02.00,0:00:05.00,Default,,0000,0000,0000,,Line B",
        ],
        expected: &[
            "Dialogue: 0,0:00:01.00,0:00:04.00,Default,,0000,0000,0000,,Line A",
            "Dialogue: 0,0:00:02.00,0:00:05.00,Default,,0000,0000,0000,,Line B",
        ],
    };

    static FILTER_EFFECTS_CASE: TestCase = TestCase {
        name: "removes lines with allowed effects",
        input: &[
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:01.00,0:00:03.00,Default,,0,0,0,,Normal line 1",
            "Dialogue: 0,0:00:03.00,0:00:05.00,Default,,0,0,0,,Normal line 2",
            "Dialogue: 0,0:00:05.00,0:00:07.00,Default,,0,0,0,fade,This line should be removed",
            "Dialogue: 0,0:00:07.00,0:00:09.00,Default,,0,0,0,,Normal line 3",
            "Dialogue: 0,0:00:09.00,0:00:11.00,Default,,0,0,0,blur,Another line to be removed",
        ],
        expected: &[
            "Dialogue: 0,0:00:01.00,0:00:03.00,Default,,0,0,0,,Normal line 1",
            "Dialogue: 0,0:00:03.00,0:00:05.00,Default,,0,0,0,,Normal line 2",
            "Dialogue: 0,0:00:05.00,0:00:07.00,Default,,0,0,0,fade,This line should be removed",
            "Dialogue: 0,0:00:07.00,0:00:09.00,Default,,0,0,0,,Normal line 3",
            "Dialogue: 0,0:00:09.00,0:00:11.00,Default,,0,0,0,blur,Another line to be removed",
        ],
    };

    static REMOVE_OP_STYLE_CASE: TestCase = TestCase {
        name: "removes lines with forbidden styles",
        input: &[
            "[Script Info]",
            "Title: Test 2",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:01.00,0:00:03.00,Default,,0000,0000,0000,,Hello world",
            "Dialogue: 0,0:00:03.00,0:00:05.00,Default,,0000,0000,0000,,Another line",
            "Dialogue: 0,0:00:05.00,0:00:07.00,OP,,0000,0000,0000,,This should be removed",
        ],
        expected: &[
            "Dialogue: 0,0:00:01.00,0:00:03.00,Default,,0000,0000,0000,,Hello world",
            "Dialogue: 0,0:00:03.00,0:00:05.00,Default,,0000,0000,0000,,Another line",
        ],
    };

    static WITH_FORMAT_CASE: TestCase = TestCase {
        name: "keeps lines with format but removes empty ones",
        input: &[
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:02.00,0:00:05.00,Default,,0000,0000,0000,,{\\i1}Formatted text",
            "Dialogue: 0,0:00:05.00,0:00:08.00,Default,,0000,0000,0000,,{\\b1}{\\i1}",
        ],
        expected: &[
            "Dialogue: 0,0:00:02.00,0:00:05.00,Default,,0000,0000,0000,,{\\i1}Formatted text",
        ],
    };

    static JUNK_IGNORED_CASE: TestCase = TestCase {
        name: "ignores trash header lines",
        input: &[
            "[Script Info]",
            "; trash",
            "; more trash",
            "",
            "[V4 Styles]",
            "; ignore all",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:03.00,0:00:05.00,Default,,0000,0000,0000,,Hello",
        ],
        expected: &["Dialogue: 0,0:00:03.00,0:00:05.00,Default,,0000,0000,0000,,Hello"],
    };

    static LONG_TAGS_CASE: TestCase = TestCase {
        name: "removes lines with excessively long tags (animations)",
        input: &[
            "Dialogue: 0,0:00:01.00,0:00:05.00,Default,,0,0,0,,{\\an4\\pos(100,100)\\t(0,500,\\fscx120\\fscy120\\blur5\\1c&H0000FF&\\t(500,1000,\\fscx100\\fscy100\\blur0\\1c&HFFFFFF&))}Text with heavy animation",
        ],
        expected: &[],
    };

    fn run_test_case(test_case: &TestCase) {
        let mut input: Vec<String> = test_case.input.iter().map(|s| s.to_string()).collect();
        let expected: Vec<String> = test_case.expected.iter().map(|s| s.to_string()).collect();
        let mut cleaner = Cleaner::new();
        cleaner.run(&mut input);
        assert_eq!(input, expected, "Failed at case: {}", test_case.name);
    }

    #[test]
    fn test_clean_basic_duplicates() {
        run_test_case(&DUPLICATE_CASE);
    }

    #[test]
    fn test_clean_filter_effects() {
        run_test_case(&FILTER_EFFECTS_CASE);
    }

    #[test]
    fn test_clean_remove_op_style() {
        run_test_case(&REMOVE_OP_STYLE_CASE);
    }

    #[test]
    fn test_clean_with_format() {
        run_test_case(&WITH_FORMAT_CASE);
    }

    #[test]
    fn test_clean_ignored_junk() {
        run_test_case(&JUNK_IGNORED_CASE);
    }

    #[test]
    fn test_clean_excessive_tags() {
        run_test_case(&LONG_TAGS_CASE);
    }
}
