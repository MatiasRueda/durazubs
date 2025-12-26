#[cfg(test)]
mod tests {
    use durazubs::model::{
        format::ass::{
            ass_processor::AssProcessor,
            parser::parser_error::{ParseRes, ParserError},
        },
        subtitle_processor::SubtitleProcessor,
    };

    struct TestCase {
        name: &'static str,
        input_a: &'static [&'static str],
        input_b: &'static [&'static str],
        expected_fields: Option<usize>,
        expected_output: &'static [&'static str],
    }

    static CORRUPT_A_CASE: TestCase = TestCase {
        name: "fails when input A has corrupt dialogue",
        input_a: &["[Events]", "Dialogue: 0,0:00:01.00,Error"],
        input_b: &[
            "[Events]",
            "Dialogue: 0,0:00:01.00,0:00:02.00,Default,,0,0,0,,Ok",
        ],
        expected_fields: Some(3),
        expected_output: &[],
    };

    static INCOMPLETE_A_CASE: TestCase = TestCase {
        name: "fails when input A has incomplete fields",
        input_a: &[
            "[Events]",
            "Dialogue: 0,0:00:01.00,0:00:02.00,Style,Name,0,0",
        ],
        input_b: &[
            "[Events]",
            "Dialogue: 0,0:00:01.00,0:00:02.00,Default,,0,0,0,,Ok",
        ],
        expected_fields: Some(7),
        expected_output: &[],
    };

    static SUCCESS_CASE: TestCase = TestCase {
        name: "basic process success",
        input_a: &[
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:01.00,0:00:02.00,Default,,0,0,0,,Line 1",
        ],
        input_b: &[
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:01.00,0:00:02.00,Default,,0,0,0,,Línea 1",
        ],
        expected_fields: None,
        expected_output: &["Línea 1"],
    };

    static SYNC_CASE: TestCase = TestCase {
        name: "generic sync: timing recalibration and noise removal",
        input_a: &[
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Comment: 0,0:00:00.00,0:00:05.00,Noise,,0,0,0,,DISCARD_ME",
            "Dialogue: 10,0:00:01.50,0:00:03.00,OldStyle,Actor,0,0,0,,Line 1 text A",
            "Dialogue: 10,0:00:04.00,0:00:07.00,OldStyle,Actor,0,0,0,,Line 2 text A",
        ],
        input_b: &[
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:00.50,0:00:02.50,OtherStyle,,0000,0000,0000,,Line 1 text B",
            "Dialogue: 0,0:00:03.00,0:00:05.50,OtherStyle,,0000,0000,0000,,Line 2 Text B",
        ],
        expected_fields: None,
        expected_output: &[
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:01.50,0:00:03.50,OtherStyle,,0,0,0,,Line 1 text B",
            "Dialogue: 0,0:00:04.00,0:00:06.50,OtherStyle,,0,0,0,,Line 2 Text B",
        ],
    };

    static EXTRACT_SUCCESS_CASE: TestCase = TestCase {
        name: "extracts additional scenes correctly",
        input_a: &[
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:20.00,0:00:22.00,Default,,0,0,0,,Normal line",
            "Dialogue: 0,0:00:10.00,0:00:12.00,Default,ADDITIONAL SCENE,0,0,0,,Second extra",
            "Dialogue: 0,0:00:05.00,0:00:07.00,Default,ADDITIONAL SCENE,0,0,0,,First extra",
        ],
        input_b: &[],
        expected_fields: None,
        expected_output: &[
            "Act as an expert anime translator. You will process a block of subtitles from ENGLISH to NEUTRAL LATIN AMERICAN SPANISH.",
            "---",
            "Second extra",
            "First extra",
        ],
    };

    fn run_test_case(test_case: &TestCase) -> ParseRes<Vec<String>> {
        let proc = AssProcessor::new();
        let mut lines_a: Vec<String> = test_case.input_a.iter().map(|s| s.to_string()).collect();
        let lines_b: Vec<String> = test_case.input_b.iter().map(|s| s.to_string()).collect();
        proc.synchronize(&mut lines_a, &lines_b)
    }

    #[test]
    fn test_process_success() -> ParseRes<()> {
        let result = run_test_case(&SUCCESS_CASE)?;
        assert!(
            result
                .iter()
                .any(|l| l.contains(SUCCESS_CASE.expected_output[0])),
            "Case failed: {}",
            SUCCESS_CASE.name
        );
        Ok(())
    }

    #[test]
    fn test_generic_synchronization_logic() -> ParseRes<()> {
        let proc = AssProcessor::new();
        let mut lines_a: Vec<String> = SYNC_CASE.input_a.iter().map(|s| s.to_string()).collect();
        let lines_b: Vec<String> = SYNC_CASE.input_b.iter().map(|s| s.to_string()).collect();
        let synced = proc.synchronize(&mut lines_a, &lines_b)?;
        for (i, expected_line) in SYNC_CASE.expected_output.iter().enumerate() {
            assert_eq!(
                synced[i], *expected_line,
                "Fallo en la sincronización de la línea {}: se esperaba un tiempo y estilo específicos",
                i
            );
        }

        Ok(())
    }

    #[test]
    fn test_error_propagation_short() {
        let result = run_test_case(&CORRUPT_A_CASE);
        let expected = CORRUPT_A_CASE.expected_fields.unwrap();
        match result {
            Err(ParserError::MissingFields { found }) => {
                assert_eq!(
                    found, expected,
                    "Field count mismatch in: {}",
                    CORRUPT_A_CASE.name
                )
            }
            _ => panic!(
                "Case: {} | Expected MissingFields({}), got {:?}",
                CORRUPT_A_CASE.name, expected, result
            ),
        }
    }

    #[test]
    fn test_error_propagation_incomplete() {
        let result = run_test_case(&INCOMPLETE_A_CASE);
        let expected = INCOMPLETE_A_CASE.expected_fields.unwrap();
        match result {
            Err(ParserError::MissingFields { found }) => {
                assert_eq!(
                    found, expected,
                    "Field count mismatch in: {}",
                    INCOMPLETE_A_CASE.name
                )
            }
            _ => panic!(
                "Case: {} | Expected MissingFields({}), got {:?}",
                INCOMPLETE_A_CASE.name, expected, result
            ),
        }
    }

    #[test]
    fn test_extract_lines_structure() -> ParseRes<()> {
        let proc = AssProcessor::new();
        let mut lines: Vec<String> = EXTRACT_SUCCESS_CASE
            .input_a
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = proc.get_lines_to_translate(&mut lines)?;
        assert_eq!(
            result.len(),
            EXTRACT_SUCCESS_CASE.expected_output.len(),
            "Structure mismatch"
        );
        Ok(())
    }

    #[test]
    fn test_extract_lines_content_integrity() -> ParseRes<()> {
        let proc = AssProcessor::new();
        let mut lines: Vec<String> = EXTRACT_SUCCESS_CASE
            .input_a
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = proc.get_lines_to_translate(&mut lines)?;
        for (i, expected_text) in EXTRACT_SUCCESS_CASE.expected_output.iter().enumerate() {
            assert!(
                result[i].contains(expected_text),
                "Content mismatch at line {}",
                i
            );
        }
        Ok(())
    }
}
