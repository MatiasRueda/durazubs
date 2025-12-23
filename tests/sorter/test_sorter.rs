#[cfg(test)]
mod tests {
    use durazubs::model::format::ass::{parser::parser_error::ParseRes, sorter::sorter::Sorter};

    struct TestCase {
        name: &'static str,
        input: &'static [&'static str],
        expected: &'static [&'static str],
    }

    static BASIC_ORDER_CASE: TestCase = TestCase {
        name: "basic line ordering",
        input: &[
            "[Script Info]",
            "Title: Basic Test",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:05.00,0:00:06.00,Default,,0,0,0,,Line 3",
            "Dialogue: 0,0:00:01.00,0:00:02.00,Default,,0,0,0,,Line 1",
            "Dialogue: 0,0:00:03.00,0:00:04.00,Default,,0,0,0,,Line 2",
        ],
        expected: &[
            "[Script Info]",
            "Title: Basic Test",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:01.00,0:00:02.00,Default,,0,0,0,,Line 1",
            "Dialogue: 0,0:00:03.00,0:00:04.00,Default,,0,0,0,,Line 2",
            "Dialogue: 0,0:00:05.00,0:00:06.00,Default,,0,0,0,,Line 3",
        ],
    };

    static WITH_METADATA_CASE: TestCase = TestCase {
        name: "ordering with lines outside Events and metadata",
        input: &[
            "[Script Info]",
            "Title: With lines outside Events",
            "SomeMetadata: true",
            "",
            "[V4+ Styles]",
            "Format: Name,Fontname,Fontsize,PrimaryColour",
            "Style: Default,Arial,20,&H00FFFFFF",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:04.00,0:00:05.00,Default,,0,0,0,,C",
            "Dialogue: 0,0:00:02.00,0:00:03.00,Default,,0,0,0,,B",
            "Dialogue: 0,0:00:01.00,0:00:02.00,Default,,0,0,0,,A",
        ],
        expected: &[
            "[Script Info]",
            "Title: With lines outside Events",
            "SomeMetadata: true",
            "",
            "[V4+ Styles]",
            "Format: Name,Fontname,Fontsize,PrimaryColour",
            "Style: Default,Arial,20,&H00FFFFFF",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:01.00,0:00:02.00,Default,,0,0,0,,A",
            "Dialogue: 0,0:00:02.00,0:00:03.00,Default,,0,0,0,,B",
            "Dialogue: 0,0:00:04.00,0:00:05.00,Default,,0,0,0,,C",
        ],
    };

    static CLOSE_TIMES_CASE: TestCase = TestCase {
        name: "ordering with close timestamps",
        input: &[
            "[Script Info]",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:01.50,0:00:02.00,Default,,0,0,0,,B",
            "Dialogue: 0,0:00:01.40,0:00:02.00,Default,,0,0,0,,A",
            "Dialogue: 0,0:00:01.90,0:00:02.00,Default,,0,0,0,,C",
        ],
        expected: &[
            "[Script Info]",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:01.40,0:00:02.00,Default,,0,0,0,,A",
            "Dialogue: 0,0:00:01.50,0:00:02.00,Default,,0,0,0,,B",
            "Dialogue: 0,0:00:01.90,0:00:02.00,Default,,0,0,0,,C",
        ],
    };

    static CORRUPT_TIME_CASE: TestCase = TestCase {
        name: "fails on corrupt timestamp",
        input: &["Dialogue: 0,99:XX:99.00,0:00:02.00,Default,,0,0,0,,Error"],
        expected: &[],
    };

    fn run_test_case(test_case: &TestCase) -> ParseRes<()> {
        let input: Vec<String> = test_case.input.iter().map(|s| s.to_string()).collect();
        let expected: Vec<String> = test_case.expected.iter().map(|s| s.to_string()).collect();
        let sorter = Sorter::new();
        let result = sorter.run(&input)?;
        assert_eq!(result, expected, "Failed at case: {}", test_case.name);
        Ok(())
    }

    #[test]
    fn test_basic_ordering() -> ParseRes<()> {
        run_test_case(&BASIC_ORDER_CASE)
    }

    #[test]
    fn test_ordering_with_metadata() -> ParseRes<()> {
        run_test_case(&WITH_METADATA_CASE)
    }

    #[test]
    fn test_ordering_close_times() -> ParseRes<()> {
        run_test_case(&CLOSE_TIMES_CASE)
    }

    #[test]
    #[should_panic]
    fn test_fails_on_corrupt_time() {
        run_test_case(&CORRUPT_TIME_CASE).unwrap();
    }
}
