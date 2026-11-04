use std::path::{Path, PathBuf};
use std::process::Command;

/// Management of embedded and isolated Python runtime environment for Pole.
pub struct PythonRuntime;

impl PythonRuntime {
    /// Discovers the most suitable Python interpreter, prioritizing embedded or bundled
    /// Python distributed with the application to avoid reliance on host OS packages.
    pub fn find_interpreter() -> (PathBuf, bool) {
        // 1. Check explicit POLE_PYTHON environment variable
        if let Ok(custom_path) = std::env::var("POLE_PYTHON") {
            let p = PathBuf::from(custom_path);
            if p.exists() {
                return (p, true);
            }
        }

        // 2. Check layout adjacent to the Pole executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // Windows embedded layout: <exe_dir>/python/python.exe or <exe_dir>/python.exe
                let win_candidates = [
                    exe_dir.join("python").join("python.exe"),
                    exe_dir.join("python.exe"),
                    exe_dir.join("bin").join("python.exe"),
                ];
                for cand in &win_candidates {
                    if cand.exists() {
                        return (cand.clone(), true);
                    }
                }

                // Linux / Flatpak standalone layout:
                let nix_candidates = [
                    exe_dir.join("python").join("bin").join("python3"),
                    exe_dir.join("python3"),
                    exe_dir.join("..").join("lib").join("pole").join("python").join("bin").join("python3"),
                    exe_dir.join("..").join("share").join("pole").join("python-embed").join("bin").join("python3"),
                ];
                for cand in &nix_candidates {
                    if cand.exists() {
                        return (cand.clone(), true);
                    }
                }
            }
        }

        // 3. Flatpak isolated binary inside sandbox (/app/bin/python3)
        let flatpak_py = PathBuf::from("/app/bin/python3");
        if flatpak_py.exists() {
            return (flatpak_py, true);
        }

        // 4. Local virtual environments in project (for development and unit testing)
        let local_candidates = [
            PathBuf::from("./python_embed/bin/python3"),
            PathBuf::from("./.venv/bin/python3"),
            PathBuf::from("./python/venv/bin/python3"),
            PathBuf::from("./python_embed/python.exe"),
            PathBuf::from("./.venv/Scripts/python.exe"),
        ];
        for cand in &local_candidates {
            if cand.exists() {
                return (cand.clone(), true);
            }
        }

        // 5. System fallback interpreter from host OS
        #[cfg(target_os = "windows")]
        {
            (PathBuf::from("python.exe"), false)
        }
        #[cfg(not(target_os = "windows"))]
        {
            (PathBuf::from("python3"), false)
        }
    }

    /// Returns the list of directories for PYTHONPATH, ensuring access to the `pole` module.
    pub fn get_pythonpath_dirs() -> Vec<PathBuf> {
        let mut dirs = Vec::new();

        // Current workspace / repository root
        dirs.push(PathBuf::from("python"));
        dirs.push(PathBuf::from("."));

        // Application installation directories or Flatpak share dirs
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                dirs.push(exe_dir.join("..").join("share").join("pole").join("python"));
                dirs.push(exe_dir.join("python"));
                dirs.push(exe_dir.join("lib").join("pole").join("python"));
            }
        }

        dirs.push(PathBuf::from("/app/share/pole/python"));
        dirs.push(PathBuf::from("/usr/share/pole/python"));
        dirs.push(PathBuf::from("/usr/local/share/pole/python"));

        dirs
    }

    /// Creates a configured `std::process::Command` instance, fully isolated
    /// from host OS packages (PYTHONNOUSERSITE=1, unbuffered I/O, and correct PYTHONPATH).
    pub fn create_command() -> Command {
        let (interp, is_embedded) = Self::find_interpreter();
        let mut cmd = Command::new(&interp);

        // Isolation: do not load user ~/.local/lib/python*/site-packages from host OS
        cmd.env("PYTHONNOUSERSITE", "1");
        // Do not buffer output to allow real-time console streaming
        cmd.env("PYTHONUNBUFFERED", "1");
        // Do not write .pyc bytecode cache files in user folders
        cmd.env("PYTHONDONTWRITEBYTECODE", "1");

        if is_embedded {
            cmd.env("POLE_EMBEDDED_PYTHON", "1");
        }

        // Construct isolated PYTHONPATH with pole module included
        let separator = if cfg!(target_os = "windows") { ";" } else { ":" };
        let mut path_entries = Vec::new();

        for d in Self::get_pythonpath_dirs() {
            if d.exists() {
                if let Ok(abs) = d.canonicalize() {
                    path_entries.push(abs.to_string_lossy().to_string());
                } else {
                    path_entries.push(d.to_string_lossy().to_string());
                }
            }
        }

        if let Ok(existing_pp) = std::env::var("PYTHONPATH") {
            if !existing_pp.is_empty() {
                path_entries.push(existing_pp);
            }
        }

        let combined_pythonpath = path_entries.join(separator);
        cmd.env("PYTHONPATH", combined_pythonpath);

        cmd
    }

    /// Wraps user script code for execution with preloaded `robot` and `turtle` instances.
    pub fn wrap_user_script(user_code: &str) -> String {
        format!(
            r#"import sys, os

# Prepend bundled pole module path to sys.path
for p in os.environ.get("PYTHONPATH", "").split(os.pathsep):
    if p and p not in sys.path:
        sys.path.insert(0, p)

try:
    try:\n    from pypole import robot, turtle, робот, черепаха\nexcept ImportError:\n    from pole import robot, turtle, робот, черепаха
    # Top-level Robot command aliases
    up = robot.up
    down = robot.down
    left = robot.left
    right = robot.right
    paint = robot.paint
    reset = robot.reset
    step_back = robot.step_back
    вверх = robot.вверх
    вниз = robot.вниз
    влево = robot.влево
    вправо = robot.вправо
    закрасить = robot.закрасить
    сброс = robot.сброс
    шаг_назад = robot.шаг_назад

    # Top-level Turtle command aliases
    forward = turtle.forward
    backward = turtle.backward
    turn_left = turtle.left
    turn_right = turtle.right
    pen_down = turtle.pen_down
    pen_up = turtle.pen_up
    clear = turtle.clear
    home = turtle.home
    вперёд = вперед = turtle.вперёд
    назад = turtle.назад
    налево = turtle.влево
    направо = turtle.вправо
    опустить_хвост = turtle.опустить_хвост
    поднять_хвост = turtle.поднять_хвост
    очистить_холст = turtle.очистить
    домой = turtle.домой
except Exception as _e:
    pass

# User script code:
{}
"#,
            user_code
        )
    }

    /// Wraps REPL input line for interactive Python console.
    pub fn wrap_repl_code(code_literal_json: &str) -> String {
        format!(
            r#"import sys, os, json

for p in os.environ.get("PYTHONPATH", "").split(os.pathsep):
    if p and p not in sys.path:
        sys.path.insert(0, p)

try:
    try:\n    from pypole import robot, turtle, робот, черепаха\nexcept ImportError:\n    from pole import robot, turtle, робот, черепаха
    up = robot.up
    down = robot.down
    left = robot.left
    right = robot.right
    paint = robot.paint
    reset = robot.reset
    вверх = robot.вверх
    вниз = robot.вниз
    влево = robot.влево
    вправо = robot.вправо
    закрасить = robot.закрасить
    сброс = robot.сброс
    forward = turtle.forward
    backward = turtle.backward
    turn_left = turtle.left
    turn_right = turtle.right
    pen_down = turtle.pen_down
    pen_up = turtle.pen_up
    clear = turtle.clear
    home = turtle.home
    вперёд = вперед = turtle.вперёд
    назад = turtle.назад
    налево = turtle.влево
    направо = turtle.вправо
except Exception:
    pass

code_input = {}
try:
    res = eval(compile(code_input, "<console>", "eval"))
    if res is not None:
        print(repr(res))
except SyntaxError:
    exec(compile(code_input, "<console>", "exec"))
"#,
            code_literal_json
        )
    }
}
