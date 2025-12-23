#[cfg(test)]
mod tests {
    use durazubs::model::format::ass::extractor::scene_extractor::SceneExtractor;

    struct TestCase {
        name: &'static str,
        input: &'static [&'static str],
        expected: &'static [&'static str],
    }

    static BASIC_CASE: TestCase = TestCase {
        name: "extracts basic additional scenes",
        input: &[
            "[Script Info]",
            "Title: Full Scene Test",
            "ScriptType: v4.00+",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:00.50,0:00:02.50,Default,Character1,0,0,0,,Hey! Are you ready?",
            "Dialogue: 0,0:00:03.00,0:00:05.00,Default,ADDITIONAL SCENE,0,0,0,,Extra scene 1 line 1",
            "Dialogue: 0,0:00:05.50,0:00:07.00,Default,ADDITIONAL SCENE,0,0,0,,Extra scene 1 line 2",
            "Dialogue: 0,0:00:07.50,0:00:09.00,Default,Character2,0,0,0,,Let's go!",
            "Dialogue: 0,0:00:09.50,0:00:11.50,Default,ADDITIONAL SCENE,0,0,0,,Extra scene 2 line 1",
            "Dialogue: 0,0:00:12.00,0:00:14.00,Default,ADDITIONAL SCENE,0,0,0,,Extra scene 2 line 2",
            "Dialogue: 0,0:00:14.50,0:00:16.00,Default,Character3,0,0,0,,See you there!",
            "Dialogue: 0,0:00:16.50,0:00:18.00,Default,ADDITIONAL SCENE,0,0,0,,Final extra line",
            "Dialogue: 0,0:00:18.50,0:00:20.00,Default,Character1,0,0,0,,Bye!",
            "Dialogue: 0,0:00:20.50,0:00:22.50,Default,Character2,0,0,0,,Take care!",
            "Dialogue: 0,0:00:23.00,0:00:25.00,Default,ADDITIONAL SCENE,0,0,0,,Bonus scene line",
        ],
        expected: &[
            "Extra scene 1 line 1",
            "Extra scene 1 line 2",
            "Extra scene 2 line 1",
            "Extra scene 2 line 2",
            "Final extra line",
            "Bonus scene line",
        ],
    };

    static NO_ADDITIONAL_SCENES_CASE: TestCase = TestCase {
        name: "file without additional scenes",
        input: &[
            "[Events]",
            "Dialogue: 0,0:00:01.00,0:00:02.00,Default,CHAR1,0,0,0,,Hello",
            "Dialogue: 0,0:00:02.50,0:00:03.00,Default,CHAR2,0,0,0,,Bye",
        ],
        expected: &[],
    };

    static ONLY_ADDITIONAL_SCENES_CASE: TestCase = TestCase {
        name: "file with only additional scenes",
        input: &[
            "Dialogue: 0,0:00:01.00,0:00:02.00,Default,ADDITIONAL SCENE,0,0,0,,Extra 1",
            "Dialogue: 0,0:00:02.50,0:00:03.50,Default,ADDITIONAL SCENE,0,0,0,,Extra 2",
        ],
        expected: &["Extra 1", "Extra 2"],
    };

    fn run_test_case(test_case: &TestCase) {
        let input: Vec<String> = test_case.input.iter().map(|s| s.to_string()).collect();
        let expected: Vec<String> = test_case.expected.iter().map(|s| s.to_string()).collect();

        let mut extractor = SceneExtractor::new();
        let result = extractor
            .run(&input)
            .unwrap_or_else(|e| panic!("Case '{}': unexpected error {:?}", test_case.name, e));

        assert_eq!(result, expected, "Failed at case: {}", test_case.name);
    }

    #[test]
    fn test_extract_basic_case() {
        run_test_case(&BASIC_CASE);
    }

    #[test]
    fn test_extract_without_additional_scenes() {
        run_test_case(&NO_ADDITIONAL_SCENES_CASE);
    }

    #[test]
    fn test_extract_only_additional_scenes() {
        run_test_case(&ONLY_ADDITIONAL_SCENES_CASE);
    }
}
