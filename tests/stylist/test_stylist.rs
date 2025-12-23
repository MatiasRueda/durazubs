#[cfg(test)]
mod tests {
    use durazubs::model::format::ass::{
        parser::parser_error::ParseRes,
        stylist::stylist::{style_type::StyleType, stylist::Stylist},
    };

    struct TestCase {
        name: &'static str,
        input: &'static [&'static str],
        expected: &'static [&'static str],
    }

    static MAIN_CASE: TestCase = TestCase {
        name: "adds styles and changes dialogue style to Main",
        input: &[
            "[Script Info]",
            "Title: Basic Test",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 10,0:00:01.00,0:00:02.00,Default,,0,0,0,,Line 1",
            "Dialogue: 10,0:00:02.00,0:00:03.00,Default,,0,0,0,,Line 2",
        ],
        expected: &[
            "[Script Info]",
            "Title: Basic Test",
            "PlayResX: 640",
            "PlayResY: 360",
            "ScaledBorderAndShadow: yes",
            "",
            "[V4+ Styles]",
            "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding",
            "Style: Main,Trebuchet MS,24,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 10,0:00:01.00,0:00:02.00,Main,,0,0,0,,Line 1",
            "Dialogue: 10,0:00:02.00,0:00:03.00,Main,,0,0,0,,Line 2",
        ],
    };

    static SECOND_CASE: TestCase = TestCase {
        name: "changes style to Second",
        input: &[
            "[Script Info]",
            "Title: Second Test",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 10,0:00:01.00,0:00:02.00,Default,,0,0,0,,Hello",
            "Dialogue: 10,0:00:02.00,0:00:03.00,Default,,0,0,0,,World",
        ],
        expected: &[
            "[Script Info]",
            "Title: Second Test",
            "PlayResX: 640",
            "PlayResY: 360",
            "ScaledBorderAndShadow: yes",
            "",
            "[V4+ Styles]",
            "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding",
            "Style: Second,Roboto,22,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1.5,2,10,10,10,1",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 10,0:00:01.00,0:00:02.00,Second,,0,0,0,,Hello",
            "Dialogue: 10,0:00:02.00,0:00:03.00,Second,,0,0,0,,World",
        ],
    };

    static EXISTING_STYLE_CASE: TestCase = TestCase {
        name: "adds new style to already existing styles",
        input: &[
            "[Script Info]",
            "Title: Existing Styles Test",
            "",
            "[V4+ Styles]",
            "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding",
            "Style: Default,Tahoma,18,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 10,0:00:01.00,0:00:02.00,Default,,0,0,0,,Hello",
            "Dialogue: 10,0:00:02.00,0:00:03.00,Default,,0,0,0,,World",
        ],
        expected: &[
            "[Script Info]",
            "Title: Existing Styles Test",
            "PlayResX: 640",
            "PlayResY: 360",
            "ScaledBorderAndShadow: yes",
            "",
            "[V4+ Styles]",
            "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding",
            "Style: Default,Tahoma,18,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1",
            "Style: Main,Trebuchet MS,24,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 10,0:00:01.00,0:00:02.00,Main,,0,0,0,,Hello",
            "Dialogue: 10,0:00:02.00,0:00:03.00,Main,,0,0,0,,World",
        ],
    };

    static MULTIPLE_STYLES_CASE: TestCase = TestCase {
        name: "adds new style and replaces only the dominant one",
        input: &[
            "[Script Info]",
            "Title: Multiple Styles Test",
            "",
            "[V4+ Styles]",
            "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding",
            "Style: Default,Tahoma,18,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1",
            "Style: Alternate,Arial,20,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1",
            "Style: Signs,Comic Sans,22,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 10,0:00:01.00,0:00:02.00,Default,,0,0,0,,Hello",
            "Dialogue: 10,0:00:02.00,0:00:03.00,Alternate,,0,0,0,,World",
            "Dialogue: 10,0:00:03.00,0:00:04.00,Default,,0,0,0,,Bye",
        ],
        expected: &[
            "[Script Info]",
            "Title: Multiple Styles Test",
            "PlayResX: 640",
            "PlayResY: 360",
            "ScaledBorderAndShadow: yes",
            "",
            "[V4+ Styles]",
            "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding",
            "Style: Default,Tahoma,18,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1",
            "Style: Alternate,Arial,20,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1",
            "Style: Signs,Comic Sans,22,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1",
            "Style: Main,Trebuchet MS,24,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 10,0:00:01.00,0:00:02.00,Main,,0,0,0,,Hello",
            "Dialogue: 10,0:00:02.00,0:00:03.00,Alternate,,0,0,0,,World",
            "Dialogue: 10,0:00:03.00,0:00:04.00,Main,,0,0,0,,Bye",
        ],
    };

    static FULL_INFO_CASE: TestCase = TestCase {
        name: "Script Info already has PlayResX, PlayResY and ScaledBorderAndShadow",
        input: &[
            "[Script Info]",
            "Title: Full Info Test",
            "PlayResX: 1280",
            "PlayResY: 720",
            "ScaledBorderAndShadow: yes",
            "",
            "[V4+ Styles]",
            "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding",
            "Style: Default,Tahoma,18,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 10,0:00:01.00,0:00:02.00,Default,,0,0,0,,Hello",
        ],
        expected: &[
            "[Script Info]",
            "Title: Full Info Test",
            "PlayResX: 640",
            "PlayResY: 360",
            "ScaledBorderAndShadow: yes",
            "",
            "[V4+ Styles]",
            "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding",
            "Style: Default,Tahoma,18,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1",
            "Style: Main,Trebuchet MS,24,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,1,2,0010,0010,0018,1",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 10,0:00:01.00,0:00:02.00,Main,,0,0,0,,Hello",
        ],
    };

    static CORRUPT_METADATA_CASE: TestCase = TestCase {
        name: "fails on corrupt script info",
        input: &["[Script Info]", "InvalidMetadataLine"],
        expected: &[],
    };

    fn run_test_case(test_case: &TestCase, style_type: &StyleType) -> ParseRes<()> {
        let input: Vec<String> = test_case.input.iter().map(|s| s.to_string()).collect();
        let expected: Vec<String> = test_case.expected.iter().map(|s| s.to_string()).collect();
        let stylist = Stylist::new(style_type);
        let result = stylist.run(&input)?;
        assert_eq!(result, expected, "Failed at case: {}", test_case.name);
        Ok(())
    }

    #[test]
    fn test_main_strategy() -> ParseRes<()> {
        run_test_case(&MAIN_CASE, &StyleType::Main)
    }

    #[test]
    fn test_second_strategy() -> ParseRes<()> {
        run_test_case(&SECOND_CASE, &StyleType::Second)
    }

    #[test]
    fn test_replaces_existing_styles() -> ParseRes<()> {
        run_test_case(&EXISTING_STYLE_CASE, &StyleType::Main)
    }

    #[test]
    fn test_replaces_multiple_styles() -> ParseRes<()> {
        run_test_case(&MULTIPLE_STYLES_CASE, &StyleType::Main)
    }

    #[test]
    fn test_full_info_case() -> ParseRes<()> {
        run_test_case(&FULL_INFO_CASE, &StyleType::Main)
    }

    #[test]
    #[should_panic]
    fn test_fails_on_corrupt_metadata() {
        run_test_case(&CORRUPT_METADATA_CASE, &StyleType::Main).unwrap();
    }
}
