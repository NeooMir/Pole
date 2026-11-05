mod lang;
mod plugins;
mod robot;
mod ui;

use crate::ui::AppWindow;
use adw::prelude::*;

const APP_ID: &str = "io.github.NeooMir.Pole";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &adw::Application) {
    let app_window = AppWindow::new(app);
    app_window.window.present();
}

#[cfg(test)]
mod tests {
    use crate::lang::compile_source;
    use crate::lang::vm::StepResult;
    use crate::plugins::PluginMetadata;
    use crate::robot::RobotField;
    use crate::ui::keybindings::{KeybindingsConfig, Shortcut};
    use crate::ui::settings_dialog::AppSettings;
    use gtk::gdk;

    #[test]
    fn test_robot_loop_execution() {
        let code = r#"использовать Робот
алг
нач
    нц 3 раз
        вправо
        закрасить
    кц
кон
"#;
        let mut vm = compile_source(code).expect("Compilation must succeed");
        let mut field = RobotField::new(10, 10);
        assert_eq!(field.robot_x, 0);

        let mut steps = 0;
        loop {
            let res = vm.step(&mut field);
            match res {
                StepResult::Finished => break,
                StepResult::Error { message, .. } => panic!("VM error: {}", message),
                _ => {}
            }
            steps += 1;
            assert!(steps < 50, "Infinite loop detected");
        }

        assert_eq!(field.robot_x, 3);
        assert!(field.painted[0][1]);
        assert!(field.painted[0][2]);
        assert!(field.painted[0][3]);
    }

    #[test]
    fn test_multiple_runs_reexecution() {
        let code = r#"использовать Робот
алг
нач
    вправо
    закрасить
кон
"#;
        let mut field = RobotField::new(10, 10);

        // Run 1
        let mut vm1 = compile_source(code).expect("Compilation must succeed");
        loop {
            match vm1.step(&mut field) {
                StepResult::Finished => break,
                _ => {}
            }
        }
        assert_eq!(field.robot_x, 1);
        assert!(field.painted[0][1]);

        // Re-execute: reset field execution and compile fresh VM
        field.reset_execution();
        assert_eq!(field.robot_x, 0);
        assert!(!field.painted[0][1]);

        // Run 2 must succeed identically
        let mut vm2 = compile_source(code).expect("Compilation must succeed");
        loop {
            match vm2.step(&mut field) {
                StepResult::Finished => break,
                _ => {}
            }
        }
        assert_eq!(field.robot_x, 1);
        assert!(field.painted[0][1]);
    }



    #[test]
    fn test_turtle_plugin_discovery() {
        use crate::plugins::PluginEngine;
        let mut engine = PluginEngine::global().lock().unwrap();
        engine.rescan_plugins();
        let plugins = engine.list_plugins();
        let found = plugins.iter().any(|p| p.id == "turtle");
        assert!(found, "Turtle plugin must be discovered from plugins directory");
    }

    #[test]
    fn test_plugin_metadata_json() {
        let json = r#"{
            "id": "test.plugin",
            "name": "Test Plugin",
            "version": "1.2.3",
            "author": "Alice",
            "description": "A demo plugin",
            "enabled": true,
            "entry": "main.py"
        }"#;

        let meta = PluginMetadata::from_json(json).expect("Must parse plugin json");
        assert_eq!(meta.id, "test.plugin");
        assert_eq!(meta.name, "Test Plugin");
        assert_eq!(meta.version, "1.2.3");
        assert_eq!(meta.author, "Alice");
        assert!(meta.enabled);
        assert_eq!(meta.entry, Some("main.py".to_string()));

        let serialized = meta.to_json();
        let meta2 = PluginMetadata::from_json(&serialized).expect("Must re-parse serialized json");
        assert_eq!(meta2.id, meta.id);
        assert_eq!(meta2.name, meta.name);
        assert_eq!(meta2.entry, Some("main.py".to_string()));
    }

    #[test]
    fn test_keybindings_parsing_and_format() {
        let sc = Shortcut::new(gdk::Key::F10, false, true, false);
        assert_eq!(sc.to_display_string(), "Shift + F10");
        assert_eq!(sc.to_config_string(), "Shift+F10");

        let parsed = Shortcut::from_config_string("Shift+F10").expect("Must parse Shift+F10");
        assert_eq!(parsed.key, gdk::Key::F10);
        assert!(parsed.shift);
        assert!(!parsed.ctrl);
        assert!(!parsed.alt);

        let parsed_ctrl_s = Shortcut::from_config_string("Ctrl+S").expect("Must parse Ctrl+S");
        assert!(parsed_ctrl_s.ctrl);
        assert_eq!(parsed_ctrl_s.to_display_string(), "Ctrl + S");

        let default_cfg = KeybindingsConfig::default();
        assert_eq!(default_cfg.run.to_display_string(), "F5");
        assert_eq!(default_cfg.step.to_display_string(), "F10");
        assert_eq!(default_cfg.reset.to_display_string(), "F8");
    }

    #[test]
    fn test_app_settings_persistence() {
        let default_settings = AppSettings::default();
        assert_eq!(default_settings.language, "ru");
        assert!(!default_settings.vim_mode);

        let json = serde_json::to_string(&default_settings).expect("Serialization must succeed");
        let loaded: AppSettings = serde_json::from_str(&json).expect("Deserialization must succeed");
        assert_eq!(loaded.language, "ru");
        assert_eq!(loaded.theme, default_settings.theme);
        assert!(!loaded.vim_mode);

        let mut custom = default_settings.clone();
        custom.language = "en".to_string();
        custom.vim_mode = true;
        let json_custom = serde_json::to_string(&custom).unwrap();
        let loaded_custom: AppSettings = serde_json::from_str(&json_custom).unwrap();
        assert_eq!(loaded_custom.language, "en");
        assert!(loaded_custom.vim_mode);
    }

    #[test]
    fn test_english_kumir_program_execution() {
        let code = "alg test_en\nbegin\n  right\n  down\n  paint\nend\n";
        let mut vm = compile_source(code).expect("Should compile English KuMir");
        let mut field = RobotField::new(10, 10);
        loop {
            match vm.step(&mut field) {
                StepResult::Finished => break,
                StepResult::Error { message, .. } => panic!("Execution error: {}", message),
                _ => {}
            }
        }
        assert_eq!(field.robot_x, 1);
        assert_eq!(field.robot_y, 1);
        assert!(field.painted[1][1]);
    }

    #[test]
    fn test_i18n_dictionary() {
        use crate::ui::i18n::Language;
        let en = Language::En;
        let ru = Language::Ru;
        assert_eq!(en.tr("run"), "Run");
        assert_eq!(ru.tr("run"), "Выполнить");
        assert_eq!(en.tr("settings_title"), "Pole Settings");
        assert_eq!(ru.tr("settings_title"), "Настройки Поле");
        assert_eq!(en.tr("theme_title"), "Color Theme");
        assert_eq!(ru.tr("theme_title"), "Тема оформления");
        assert!(en.is_en());
        assert!(!ru.is_en());
        assert_eq!(Language::from_code("en"), Language::En);
        assert_eq!(Language::from_code("ru"), Language::Ru);
    }

    #[test]
    fn test_robot_step_back() {
        let mut field = RobotField::new(10, 10);
        assert_eq!(field.robot_x, 0);
        assert_eq!(field.robot_y, 0);

        // Move right then down
        let _ = field.move_robot(&crate::robot::Direction::Right);
        assert_eq!(field.robot_x, 1);
        assert_eq!(field.robot_y, 0);

        let _ = field.move_robot(&crate::robot::Direction::Down);
        assert_eq!(field.robot_x, 1);
        assert_eq!(field.robot_y, 1);

        // Step back should return to (1, 0)
        let stepped = field.step_back();
        assert!(stepped);
        assert_eq!(field.robot_x, 1);
        assert_eq!(field.robot_y, 0);

        // Step back again should return to (0, 0)
        let stepped2 = field.step_back();
        assert!(stepped2);
        assert_eq!(field.robot_x, 0);
        assert_eq!(field.robot_y, 0);

        // Third step back at origin should return false and keep at start
        let stepped3 = field.step_back();
        assert!(!stepped3);
        assert_eq!(field.robot_x, 0);
        assert_eq!(field.robot_y, 0);
    }

    #[test]
    fn test_syntax_mode_toggle() {
        use crate::ui::editor_view::SyntaxMode;
        assert_eq!(SyntaxMode::from_str("kumir"), SyntaxMode::Kumir);
        assert_eq!(SyntaxMode::from_str("python"), SyntaxMode::Python);
        assert_eq!(SyntaxMode::from_str("py"), SyntaxMode::Python);
        assert_eq!(SyntaxMode::from_str("unknown"), SyntaxMode::Kumir);
        assert_eq!(SyntaxMode::Kumir.as_str(), "kumir");
        assert_eq!(SyntaxMode::Python.as_str(), "python");
    }

    #[test]
    fn test_scratch_blocks_to_kumir_and_execution() {
        use crate::ui::blocks_view::{generate_kumir, Block, BlockKind, ConditionKind};
        let blocks = vec![
            Block { id: 1, kind: BlockKind::MoveRight },
            Block { id: 2, kind: BlockKind::Paint },
            Block {
                id: 3,
                kind: BlockKind::RepeatTimes {
                    times: 2,
                    body: vec![
                        Block { id: 4, kind: BlockKind::MoveDown },
                        Block { id: 5, kind: BlockKind::Paint },
                    ],
                },
            },
            Block {
                id: 6,
                kind: BlockKind::RepeatWhile {
                    cond: ConditionKind::RightFree,
                    body: vec![
                        Block { id: 7, kind: BlockKind::MoveRight },
                    ],
                },
            },
            Block {
                id: 8,
                kind: BlockKind::If {
                    cond: ConditionKind::CellClean,
                    then_body: vec![
                        Block { id: 9, kind: BlockKind::Paint },
                    ],
                },
            },
        ];

        let code = generate_kumir(&blocks);
        assert!(code.contains("использовать Робот"));
        assert!(code.contains("алг"));
        assert!(code.contains("нач"));
        assert!(code.contains("вправо"));
        assert!(code.contains("закрасить"));
        assert!(code.contains("нц 2 раз"));
        assert!(code.contains("нц пока справа свободно"));
        assert!(code.contains("если клетка чистая то"));
        assert!(code.contains("кц"));
        assert!(code.contains("все"));
        assert!(code.contains("кон"));

        let mut vm = compile_source(&code).expect("Generated KuMir code must compile");
        let mut field = RobotField::new(10, 10);
        let mut steps = 0;
        loop {
            match vm.step(&mut field) {
                StepResult::Finished => break,
                StepResult::Error { message, .. } => panic!("VM step failed: {}", message),
                _ => {}
            }
            steps += 1;
            assert!(steps < 200, "Too many steps");
        }

        assert_eq!(field.robot_y, 2);
        assert_eq!(field.robot_x, 9);
        assert!(field.painted[2][9]);
    }

    #[test]
    fn test_scratch_kumir_to_blocks_roundtrip() {
        use crate::ui::blocks_view::{generate_kumir, parse_kumir_to_blocks, BlockKind};
        let src = r#"использовать Робот
алг
нач
    вправо
    закрасить
    нц 3 раз
        вниз
        закрасить
    кц
    нц пока справа свободно
        вправо
    кц
кон
"#;
        let mut nid = 0;
        let blocks = parse_kumir_to_blocks(src, &mut nid).expect("Should parse into blocks");
        assert_eq!(blocks.len(), 4);
        assert_eq!(blocks[0].kind, BlockKind::MoveRight);
        assert_eq!(blocks[1].kind, BlockKind::Paint);
        match &blocks[2].kind {
            BlockKind::RepeatTimes { times, body } => {
                assert_eq!(*times, 3);
                assert_eq!(body.len(), 2);
            }
            _ => panic!("Expected RepeatTimes block"),
        }
        match &blocks[3].kind {
            BlockKind::RepeatWhile { cond, body } => {
                assert_eq!(*cond, crate::ui::blocks_view::ConditionKind::RightFree);
                assert_eq!(body.len(), 1);
            }
            _ => panic!("Expected RepeatWhile block"),
        }

        let gen_code = generate_kumir(&blocks);
        let vm = compile_source(&gen_code).expect("Roundtripped code must compile");
        assert!(!vm.instructions.is_empty());
    }

    #[test]
    fn test_scratch_presets_compilation_and_execution() {
        use crate::ui::blocks_view::{generate_kumir, Block, BlockKind, ConditionKind};

        let presets = vec![
            // FirstSteps
            vec![
                Block { id: 1, kind: BlockKind::MoveRight },
                Block { id: 2, kind: BlockKind::Paint },
                Block { id: 3, kind: BlockKind::MoveRight },
                Block { id: 4, kind: BlockKind::Paint },
                Block { id: 5, kind: BlockKind::MoveDown },
                Block { id: 6, kind: BlockKind::Paint },
            ],
            // Square
            vec![
                Block {
                    id: 1,
                    kind: BlockKind::RepeatTimes {
                        times: 3,
                        body: vec![Block { id: 2, kind: BlockKind::MoveRight }, Block { id: 3, kind: BlockKind::Paint }],
                    },
                },
                Block {
                    id: 4,
                    kind: BlockKind::RepeatTimes {
                        times: 3,
                        body: vec![Block { id: 5, kind: BlockKind::MoveDown }, Block { id: 6, kind: BlockKind::Paint }],
                    },
                },
            ],
            // Stairs
            vec![Block {
                id: 1,
                kind: BlockKind::RepeatTimes {
                    times: 4,
                    body: vec![
                        Block { id: 2, kind: BlockKind::MoveRight },
                        Block { id: 3, kind: BlockKind::Paint },
                        Block { id: 4, kind: BlockKind::MoveDown },
                        Block { id: 5, kind: BlockKind::Paint },
                    ],
                },
            }],
            // Snake
            vec![Block {
                id: 1,
                kind: BlockKind::RepeatWhile {
                    cond: ConditionKind::RightFree,
                    body: vec![Block { id: 2, kind: BlockKind::MoveRight }, Block { id: 3, kind: BlockKind::Paint }],
                },
            }],
        ];

        for blocks in presets {
            let code = generate_kumir(&blocks);
            let mut vm = compile_source(&code).expect("Preset must compile to valid KuMir VM");
            let mut field = RobotField::new(10, 10);
            let mut steps = 0;
            loop {
                match vm.step(&mut field) {
                    StepResult::Finished => break,
                    StepResult::Error { message, .. } => panic!("Execution error: {}", message),
                    _ => {}
                }
                steps += 1;
                assert!(steps < 200, "Overrun");
            }
        }
    }
    #[test]
    fn test_turtle_state_operations_and_step_back() {
        use crate::ui::turtle_view::TurtleState;
        let mut t = TurtleState::new();
        assert_eq!(t.x, 0.0);
        assert_eq!(t.y, 0.0);
        assert_eq!(t.angle, 90.0);
        assert!(t.pen_down);
        assert!(t.lines.is_empty());

        // Move forward 100 units (upwards since angle is 90)
        t.forward(100.0);
        assert!((t.x - 0.0).abs() < 1e-4);
        assert!((t.y - 100.0).abs() < 1e-4);
        assert_eq!(t.lines.len(), 1);

        // Turn right 90 degrees -> angle 0 (facing east / right)
        t.turn_right(90.0);
        assert!((t.angle - 0.0).abs() < 1e-4);

        // Move forward 50 units
        t.forward(50.0);
        assert!((t.x - 50.0).abs() < 1e-4);
        assert!((t.y - 100.0).abs() < 1e-4);
        assert_eq!(t.lines.len(), 2);

        // Raise pen and move
        t.set_pen(false);
        t.forward(30.0);
        assert_eq!(t.lines.len(), 2); // No new line when pen is up

        // Test step back
        assert!(t.step_back()); // undo forward(30.0)
        assert!((t.x - 50.0).abs() < 1e-4);

        assert!(t.step_back()); // undo set_pen(false)
        assert!(t.pen_down);

        assert!(t.step_back()); // undo forward(50.0)
        assert!((t.x - 0.0).abs() < 1e-4);
        assert_eq!(t.lines.len(), 1);

        // Test reset
        t.reset();
        assert_eq!(t.x, 0.0);
        assert_eq!(t.y, 0.0);
        assert_eq!(t.angle, 90.0);
        assert!(t.pen_down);
        assert!(t.lines.is_empty());
        assert!(!t.step_back()); // No history after reset
    }

    #[test]
    fn test_docs_window_and_i18n_integrity() {
        use crate::ui::i18n::Language;
        let ru = Language::Ru;
        let en = Language::En;

        // Check documentation titles and menu keys
        assert_eq!(ru.tr("menu_docs"), "Документация");
        assert_eq!(en.tr("menu_docs"), "Documentation");

        // Verify keys do not contain emojis or forbidden parenthesized suffixes
        let checked_keys = [
            "run", "step", "reset", "menu_docs", "menu_load", "menu_save",
            "syntax_kumir", "syntax_python", "mode_text", "mode_blocks",
            "scratch_title", "scratch_motion", "scratch_control",
        ];

        for key in &checked_keys {
            let val_ru = ru.tr(key);
            let val_en = en.tr(key);
            assert!(!val_ru.contains("(Scratch)"), "Must not contain (Scratch)");
            assert!(!val_ru.contains("(КуМир)"), "Must not contain (КуМир)");
            assert!(!val_en.contains("(Scratch)"), "Must not contain (Scratch)");
            // Ensure no emojis in UI labels
            for ch in val_ru.chars().chain(val_en.chars()) {
                let code = ch as u32;
                let is_emoji = (0x1F300..=0x1F9FF).contains(&code) || (0x2600..=0x26FF).contains(&code);
                assert!(!is_emoji, "Found forbidden emoji in translation key {}: {}", key, ch);
            }
        }
    }
    #[test]
    fn test_kumir_vm_marginalia_realtime_state() {
        let code = r#"использовать Робот
алг
нач
цел х
х := 10 + 5
вправо
вывод "Привет"
кон"#;
        let mut vm = compile_source(code).expect("Should compile code");
        let mut field = RobotField::new(10, 10);

        loop {
            match vm.step(&mut field) {
                StepResult::Finished => break,
                StepResult::Error { message, line } => panic!("Unexpected error at line {}: {}", line, message),
                _ => {}
            }
        }

        let m = vm.marginalia();
        assert!(m.values().any(|v| v.contains("15") && v.contains("х")), "Marginalia should capture x = 15");
        assert!(m.values().any(|v| v == "вправо"), "Marginalia should capture вправо");
        assert!(m.values().any(|v| v.contains("Привет")), "Marginalia should capture print output");
    }
    #[test]
    fn test_python_runtime_isolation_and_command_setup() {
        use crate::lang::python_runtime::PythonRuntime;

        let (interp, _) = PythonRuntime::find_interpreter();
        assert!(!interp.to_string_lossy().is_empty());

        let cmd = PythonRuntime::create_command();
        let envs: Vec<_> = cmd.get_envs().collect();
        assert!(envs.iter().any(|(k, v)| k.to_str() == Some("PYTHONNOUSERSITE") && v.and_then(|val| val.to_str()) == Some("1")),
            "Command must isolate python from user OS packages (PYTHONNOUSERSITE=1)");
        assert!(envs.iter().any(|(k, v)| k.to_str() == Some("PYTHONUNBUFFERED") && v.and_then(|val| val.to_str()) == Some("1")),
            "Command must enable unbuffered stdout");
        assert!(envs.iter().any(|(k, _v)| k.to_str() == Some("PYTHONPATH")),
            "Command must inject isolated PYTHONPATH with pole library");

        let wrapped = PythonRuntime::wrap_user_script("print('Hello from embedded Python')");
        assert!(wrapped.contains("from pole import robot"), "Wrapped script must import pole");
        assert!(wrapped.contains("print('Hello from embedded Python')"));
    }

    #[test]
    fn test_welcome_project_types() {
        use crate::ui::i18n::Language;
        use crate::ui::welcome_view::ProjectType;

        let types = ProjectType::all();
        assert_eq!(types.len(), 3);

        for pt in types {
            assert!(!pt.title(Language::En).is_empty());
            assert!(!pt.title(Language::Ru).is_empty());
            assert!(!pt.subtitle(Language::En).is_empty());
            assert!(!pt.subtitle(Language::Ru).is_empty());
            assert!(!pt.default_project_name(Language::En).is_empty());
            assert!(!pt.default_project_name(Language::Ru).is_empty());
            assert!(!pt.file_extension().is_empty());
            assert!(!pt.starter_code().is_empty());
        }

        assert_eq!(ProjectType::KumirRobot.file_extension(), "kum");
        assert_eq!(ProjectType::PythonRobot.file_extension(), "py");
        assert_eq!(ProjectType::PythonTurtle.file_extension(), "py");
    }
}
