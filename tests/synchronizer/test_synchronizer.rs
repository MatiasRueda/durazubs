#[cfg(test)]
mod tests {
    use durazubs::model::format::ass::{
        parser::parser_error::ParseRes, synchronizer::synchronizer::Synchronizer,
    };

    struct TestCase {
        name: &'static str,
        input_a: &'static [&'static str],
        input_b: &'static [&'static str],
        expected: &'static [&'static str],
    }

    static BASIC_SYNC_CASE: TestCase = TestCase {
        name: "basic synchronization with time drift",
        input_a: &[
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 10,0:00:03.77,0:00:05.48,Default,,0,0,0,,It's really chilly today!",
            "Dialogue: 10,0:00:05.48,0:00:08.36,Default,,0,0,0,,Did you notice the broken window in the kitchen?",
            "Dialogue: 10,0:00:08.36,0:00:12.11,Default,,0,0,0,,Yeah, I tripped and knocked it over while carrying boxes.",
            "Dialogue: 10,0:00:12.11,0:00:16.11,Default,,0,0,0,,Oh, so it was an accident. That makes sense.",
            "Dialogue: 10,0:00:16.11,0:00:18.74,Default,,0,0,0,,Exactly. I wasn't trying to show off or anything.",
            "Dialogue: 10,0:00:20.37,0:00:22.62,Default,,0,0,0,,No worries. Everyone makes mistakes sometimes.",
            "Dialogue: 10,0:00:22.62,0:00:27.00,Default,,0,0,0,,Thanks. It's just one of those days, I guess.",
            "Dialogue: 10,0:00:27.50,0:00:30.00,Default-Italic,ADDITIONAL SCENE,0,0,0,,A few minutes before the meeting.",
            "Dialogue: 10,0:00:30.50,0:00:32.50,Default,ADDITIONAL SCENE,0,0,0,,Someone is preparing coffee in the office.",
            "Dialogue: 10,0:00:32.80,0:00:34.50,Default,,0,0,0,,Hey, can you help me with this task?",
            "Dialogue: 10,0:00:34.80,0:00:36.95,Default,,0,0,0,,Sure, but I'm not very experienced.",
            "Dialogue: 10,0:00:36.95,0:00:38.86,Default,,0,0,0,,That's okay. We'll figure it out together.",
            "Dialogue: 10,0:00:38.86,0:00:41.44,Default,,0,0,0,,Thanks, I appreciate it.",
            "Dialogue: 10,0:00:41.73,0:00:44.92,Default,,0,0,0,,No problem! Teamwork is key.",
            "Dialogue: 10,0:00:44.92,0:00:47.33,Default,,0,0,0,,Let's get started then.",
        ],
        input_b: &[
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 10,0:00:01.77,0:00:03.48,Default,,0,0,0,,¡Hace mucho frío hoy!",
            "Dialogue: 10,0:00:03.48,0:00:06.36,Default,,0,0,0,,¿Viste la ventana rota en la cocina?",
            "Dialogue: 10,0:00:06.36,0:00:10.11,Default,,0,0,0,,Sí, tropecé y la rompí mientras llevaba cajas.",
            "Dialogue: 10,0:00:10.11,0:00:14.11,Default,,0,0,0,,Ah, entonces fue un accidente. Eso tiene sentido.",
            "Dialogue: 10,0:00:14.11,0:00:16.74,Default,,0,0,0,,Exacto. No estaba tratando de presumir ni nada.",
            "Dialogue: 10,0:00:18.37,0:00:20.62,Default,,0,0,0,,No te preocupes. Todos cometemos errores a veces.",
            "Dialogue: 10,0:00:20.62,0:00:25.00,Default,,0,0,0,,Gracias. Supongo que es uno de esos días.",
            "Dialogue: 10,0:00:30.80,0:00:32.50,Default,,0,0,0,,Oye, ¿puedes ayudarme con esta tarea?",
            "Dialogue: 10,0:00:32.80,0:00:34.95,Default,,0,0,0,,Claro, pero no tengo mucha experiencia.",
            "Dialogue: 10,0:00:34.95,0:00:36.86,Default,,0,0,0,,No importa. Lo resolveremos juntos.",
            "Dialogue: 10,0:00:36.86,0:00:39.44,Default,,0,0,0,,Gracias, lo aprecio.",
            "Dialogue: 10,0:00:39.73,0:00:42.92,Default,,0,0,0,,¡De nada! El trabajo en equipo es clave.",
            "Dialogue: 10,0:00:42.92,0:00:45.33,Default,,0,0,0,,Entonces, empecemos.",
        ],
        expected: &[
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 10,0:00:03.77,0:00:05.48,Default,,0,0,0,,¡Hace mucho frío hoy!",
            "Dialogue: 10,0:00:05.48,0:00:08.36,Default,,0,0,0,,¿Viste la ventana rota en la cocina?",
            "Dialogue: 10,0:00:08.36,0:00:12.11,Default,,0,0,0,,Sí, tropecé y la rompí mientras llevaba cajas.",
            "Dialogue: 10,0:00:12.11,0:00:16.11,Default,,0,0,0,,Ah, entonces fue un accidente. Eso tiene sentido.",
            "Dialogue: 10,0:00:16.11,0:00:18.74,Default,,0,0,0,,Exacto. No estaba tratando de presumir ni nada.",
            "Dialogue: 10,0:00:20.37,0:00:22.62,Default,,0,0,0,,No te preocupes. Todos cometemos errores a veces.",
            "Dialogue: 10,0:00:22.62,0:00:27.00,Default,,0,0,0,,Gracias. Supongo que es uno de esos días.",
            "Dialogue: 10,0:00:27.50,0:00:30.00,Default-Italic,ADDITIONAL SCENE,0,0,0,,A few minutes before the meeting.",
            "Dialogue: 10,0:00:30.50,0:00:32.50,Default,ADDITIONAL SCENE,0,0,0,,Someone is preparing coffee in the office.",
            "Dialogue: 10,0:00:32.80,0:00:34.50,Default,,0,0,0,,Oye, ¿puedes ayudarme con esta tarea?",
            "Dialogue: 10,0:00:34.80,0:00:36.95,Default,,0,0,0,,Claro, pero no tengo mucha experiencia.",
            "Dialogue: 10,0:00:36.95,0:00:38.86,Default,,0,0,0,,No importa. Lo resolveremos juntos.",
            "Dialogue: 10,0:00:38.86,0:00:41.44,Default,,0,0,0,,Gracias, lo aprecio.",
            "Dialogue: 10,0:00:41.73,0:00:44.92,Default,,0,0,0,,¡De nada! El trabajo en equipo es clave.",
            "Dialogue: 10,0:00:44.92,0:00:47.33,Default,,0,0,0,,Entonces, empecemos.",
        ],
    };

    static START_LATER_CASE: TestCase = TestCase {
        name: "input B starts later than input A",
        input_a: &[
            "[Script Info]",
            "Title: Example A",
            "ScriptType: v4.00+",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:01.00,0:00:03.00,Default,Character1,0,0,0,,Hola",
            "Dialogue: 0,0:00:04.00,0:00:06.00,Default,Character2,0,0,0,,¿Cómo estás?",
            "Dialogue: 0,0:00:06.50,0:00:08.00,Default,ADDITIONAL SCENE,0,0,0,,Escena extra",
            "Dialogue: 0,0:00:08.50,0:00:09.50,Default,ADDITIONAL SCENE,0,0,0,,Continuación extra",
            "Dialogue: 0,0:00:10.00,0:00:12.00,Default,Character3,0,0,0,,Bien, gracias",
        ],
        input_b: &[
            "[Script Info]",
            "Title: Example B",
            "ScriptType: v4.00+",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 0,0:00:01.10,0:00:03.10,Default,Character1,0,0,0,,Hola",
            "Dialogue: 0,0:00:04.10,0:00:05.10,Default,Character2,0,0,0,,¿Cómo estás?",
            "Dialogue: 0,0:00:05.50,0:00:07.50,Default,Character3,0,0,0,,Bien, gracias",
        ],
        expected: &[
            "[Script Info]",
            "Title: Example B",
            "ScriptType: v4.00+",
            "",
            "[Events]",
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            "Dialogue: 10,0:00:01.00,0:00:03.00,Default,Character1,0,0,0,,Hola",
            "Dialogue: 10,0:00:04.00,0:00:05.00,Default,Character2,0,0,0,,¿Cómo estás?",
            "Dialogue: 0,0:00:06.50,0:00:08.00,Default,ADDITIONAL SCENE,0,0,0,,Escena extra",
            "Dialogue: 0,0:00:08.50,0:00:09.50,Default,ADDITIONAL SCENE,0,0,0,,Continuación extra",
            "Dialogue: 10,0:00:10.00,0:00:12.00,Default,Character3,0,0,0,,Bien, gracias",
        ],
    };

    static MISSING_EVENTS_CASE: TestCase = TestCase {
        name: "fails when events section is missing",
        input_a: &["[Script Info]"],
        input_b: &["[Script Info]"],
        expected: &[],
    };

    fn run_test_case(test_case: &TestCase) -> ParseRes<()> {
        let input_a: Vec<String> = test_case.input_a.iter().map(|s| s.to_string()).collect();
        let input_b: Vec<String> = test_case.input_b.iter().map(|s| s.to_string()).collect();
        let expected: Vec<String> = test_case.expected.iter().map(|s| s.to_string()).collect();
        let mut synchronizer = Synchronizer::new();
        let result = synchronizer.run(&input_a, &input_b)?;
        assert_eq!(result, expected, "Failed at case: {}", test_case.name);
        Ok(())
    }

    #[test]
    fn test_basic_synchronization() -> ParseRes<()> {
        run_test_case(&BASIC_SYNC_CASE)
    }

    #[test]
    fn test_sync_when_b_starts_later() -> ParseRes<()> {
        run_test_case(&START_LATER_CASE)
    }

    #[test]
    #[should_panic]
    fn test_fails_on_missing_events() {
        run_test_case(&MISSING_EVENTS_CASE).unwrap();
    }
}
