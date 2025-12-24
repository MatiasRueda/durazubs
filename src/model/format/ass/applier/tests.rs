use super::*;

struct TestCase {
    name: &'static str,
    input: &'static [&'static str],
    translations: &'static [&'static str],
    expected: &'static [&'static str],
}

static SCENE_REPLACEMENT_CASE: TestCase = TestCase {
    name: "replaces text in ADDITIONAL SCENE lines",
    input: &[
        "Dialogue: 0,0:00:10.00,0:00:12.00,Sign,ADDITIONAL SCENE,0,0,0,,[Original Text A]",
        "Dialogue: 0,0:00:15.00,0:00:17.00,Sign,ADDITIONAL SCENE,0,0,0,,[Original Text B]",
    ],
    translations: &["Translated Text 1", "Translated Text 2"],
    expected: &[
        "Dialogue: 10,0:00:10.00,0:00:12.00,Sign,ADDITIONAL SCENE,0,0,0,,Translated Text 1",
        "Dialogue: 10,0:00:15.00,0:00:17.00,Sign,ADDITIONAL SCENE,0,0,0,,Translated Text 2",
    ],
};

static SELECTIVE_APPLY_CASE: TestCase = TestCase {
    name: "only applies translations to lines with the scene tag",
    input: &[
        "Dialogue: 0,0:00:01.00,0:00:03.00,Default,,0,0,0,,Normal Subtitle",
        "Dialogue: 0,0:00:20.00,0:00:22.00,Sign,ADDITIONAL SCENE,0,0,0,,Target Scene",
        "Dialogue: 0,0:00:25.00,0:00:27.00,Default,,0,0,0,,Another Subtitle",
    ],
    translations: &["Processed Scene"],
    expected: &[
        "Dialogue: 0,0:00:01.00,0:00:03.00,Default,,0,0,0,,Normal Subtitle",
        "Dialogue: 10,0:00:20.00,0:00:22.00,Sign,ADDITIONAL SCENE,0,0,0,,Processed Scene",
        "Dialogue: 0,0:00:25.00,0:00:27.00,Default,,0,0,0,,Another Subtitle",
    ],
};

fn run_test_case(test_case: &TestCase) -> ParseRes<()> {
    let input: Vec<String> = test_case.input.iter().map(|s| s.to_string()).collect();
    let translations: Vec<String> = test_case
        .translations
        .iter()
        .map(|s| s.to_string())
        .collect();
    let expected: Vec<String> = test_case.expected.iter().map(|s| s.to_string()).collect();
    let mut applier = SceneApplier::new();
    let result = applier.run(&input, &translations)?;
    assert_eq!(result, expected, "Failed at case: {}", test_case.name);
    Ok(())
}

#[test]
fn test_scene_replacement() -> ParseRes<()> {
    run_test_case(&SCENE_REPLACEMENT_CASE)
}

#[test]
fn test_selective_apply() -> ParseRes<()> {
    run_test_case(&SELECTIVE_APPLY_CASE)
}
