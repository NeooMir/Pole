use crate::robot::{Direction, RobotField};
use crate::ui::console_view::ConsoleView;
use crate::ui::field_view::RobotFieldView;
use gtk::glib;
use gtk::prelude::*;
use std::cell::RefCell;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct IpcServer;

struct IpcRequest {
    command: String,
    response_tx: Sender<String>,
}

impl IpcServer {
    pub fn socket_path() -> PathBuf {
        let base = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(base).join("neomir.sock")
    }

    pub fn start(
        field: Rc<RefCell<RobotField>>,
        field_view: Rc<RobotFieldView>,
        console: Rc<ConsoleView>,
        status_label: gtk::Label,
    ) {
        let sock_path = Self::socket_path();
        if sock_path.exists() {
            let _ = std::fs::remove_file(&sock_path);
        }

        let listener = match UnixListener::bind(&sock_path) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("[IPC] Не удалось запустить Unix socket listener: {}", e);
                return;
            }
        };

        let (ipc_tx, ipc_rx) = mpsc::channel::<IpcRequest>();
        let ipc_rx = Arc::new(Mutex::new(ipc_rx));

        // Poll command queue in the main GUI thread via GLib timer (15 ms ~ 60 fps)
        let rx_for_main = ipc_rx.clone();
        let f = field.clone();
        let fv = field_view.clone();
        let con = console.clone();
        let status = status_label.clone();

        glib::timeout_add_local(Duration::from_millis(15), move || {
            if let Ok(rx) = rx_for_main.try_lock() {
                while let Ok(req) = rx.try_recv() {
                    let resp = Self::handle_command(&req.command, &f, &fv, &con, &status);
                    let _ = req.response_tx.send(resp);
                }
            }
            glib::ControlFlow::Continue
        });

        // Background thread for listening to incoming socket connections
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(client) => {
                        let tx_clone = ipc_tx.clone();
                        thread::spawn(move || {
                            Self::handle_client(client, tx_clone);
                        });
                    }
                    Err(e) => {
                        eprintln!("[IPC] Ошибка подключения клиента: {}", e);
                        break;
                    }
                }
            }
        });
    }

    fn handle_client(stream: UnixStream, ipc_tx: Sender<IpcRequest>) {
        let mut writer = match stream.try_clone() {
            Ok(s) => s,
            Err(_) => return,
        };
        let reader = BufReader::new(stream);

        for line in reader.lines().flatten() {
            let trimmed = line.trim().to_string();
            if trimmed.is_empty() {
                continue;
            }

            let (tx, rx) = mpsc::channel();
            if ipc_tx
                .send(IpcRequest {
                    command: trimmed,
                    response_tx: tx,
                })
                .is_err()
            {
                break;
            }

            if let Ok(resp) = rx.recv_timeout(Duration::from_millis(2000)) {
                let _ = writeln!(writer, "{}", resp);
                let _ = writer.flush();
            } else {
                let _ = writeln!(writer, "{{\"status\":\"error\",\"message\":\"timeout\"}}");
                let _ = writer.flush();
            }
        }
    }

    fn handle_command(
        raw_cmd: &str,
        field: &Rc<RefCell<RobotField>>,
        field_view: &Rc<RobotFieldView>,
        console: &Rc<ConsoleView>,
        status: &gtk::Label,
    ) -> String {
        let cmd = raw_cmd.trim();

        // Extract command name from JSON or text string
        let cmd_name = if cmd.starts_with('{') {
            extract_cmd_val(cmd, "cmd")
                .or_else(|| extract_cmd_val(cmd, "command"))
                .unwrap_or_else(|| "unknown".to_string())
        } else {
            cmd.to_string()
        };

        match cmd_name.as_str() {
            // Robot movement and actions
            "move_up" | "up" | "вверх" => {
                let mut f = field.borrow_mut();
                match f.move_robot(&Direction::Up) {
                    Ok(_) => {
                        field_view.widget.queue_draw();
                        let msg = format!("Робот: шаг вверх ({}, {})", f.robot_x + 1, f.robot_y + 1);
                        console.log_action(&msg);
                        status.set_text(&msg);
                        format_state(&f, "ok", None)
                    }
                    Err(err) => {
                        field_view.widget.queue_draw();
                        console.log_action(&err);
                        status.set_text(&err);
                        format_state(&f, "crash", Some(&err))
                    }
                }
            }
            "move_down" | "down" | "вниз" => {
                let mut f = field.borrow_mut();
                match f.move_robot(&Direction::Down) {
                    Ok(_) => {
                        field_view.widget.queue_draw();
                        let msg = format!("Робот: шаг вниз ({}, {})", f.robot_x + 1, f.robot_y + 1);
                        console.log_action(&msg);
                        status.set_text(&msg);
                        format_state(&f, "ok", None)
                    }
                    Err(err) => {
                        field_view.widget.queue_draw();
                        console.log_action(&err);
                        status.set_text(&err);
                        format_state(&f, "crash", Some(&err))
                    }
                }
            }
            "move_left" | "left" | "влево" => {
                let mut f = field.borrow_mut();
                match f.move_robot(&Direction::Left) {
                    Ok(_) => {
                        field_view.widget.queue_draw();
                        let msg = format!("Робот: шаг влево ({}, {})", f.robot_x + 1, f.robot_y + 1);
                        console.log_action(&msg);
                        status.set_text(&msg);
                        format_state(&f, "ok", None)
                    }
                    Err(err) => {
                        field_view.widget.queue_draw();
                        console.log_action(&err);
                        status.set_text(&err);
                        format_state(&f, "crash", Some(&err))
                    }
                }
            }
            "move_right" | "right" | "вправо" => {
                let mut f = field.borrow_mut();
                match f.move_robot(&Direction::Right) {
                    Ok(_) => {
                        field_view.widget.queue_draw();
                        let msg = format!("Робот: шаг вправо ({}, {})", f.robot_x + 1, f.robot_y + 1);
                        console.log_action(&msg);
                        status.set_text(&msg);
                        format_state(&f, "ok", None)
                    }
                    Err(err) => {
                        field_view.widget.queue_draw();
                        console.log_action(&err);
                        status.set_text(&err);
                        format_state(&f, "crash", Some(&err))
                    }
                }
            }
            "paint" | "закрасить" => {
                let mut f = field.borrow_mut();
                f.paint_cell();
                field_view.widget.queue_draw();
                let msg = format!("Робот: клетка ({}, {}) закрашена", f.robot_x + 1, f.robot_y + 1);
                console.log_action(&msg);
                status.set_text(&msg);
                format_state(&f, "ok", None)
            }
            "reset" | "сброс" => {
                let mut f = field.borrow_mut();
                f.reset_execution();
                field_view.widget.queue_draw();
                console.log_action("Робот: сброшен в начальное положение");
                status.set_text("Робот сброшен");
                format_state(&f, "ok", None)
            }
            "step_back" | "return_back" | "шаг_назад" | "возврат" => {
                let mut f = field.borrow_mut();
                let ok = f.step_back();
                field_view.widget.queue_draw();
                if ok {
                    let msg = format!("Робот возвращен назад на позицию ({}, {})", f.robot_x + 1, f.robot_y + 1);
                    console.log_action(&msg);
                    status.set_text(&msg);
                    format_state(&f, "ok", None)
                } else {
                    let msg = "Робот уже в начальной точке".to_string();
                    console.log_action(&msg);
                    status.set_text(&msg);
                    format_state(&f, "ok", Some(&msg))
                }
            }
            // State and sensor queries
            "get_state" | "status" => {
                let f = field.borrow();
                format_state(&f, "ok", None)
            }

            // Console interaction
            "set_console_input" | "console_set_input" => {
                let text = extract_cmd_val(cmd, "text").unwrap_or_default();
                console.py_entry.set_text(&text);
                console.switch_to_python();
                format!("{{\"status\":\"ok\",\"input\":\"{}\"}}", text.replace('"', "\\\""))
            }
            "get_console_input" | "console_get_input" => {
                let text = console.py_entry.text().to_string();
                format!("{{\"status\":\"ok\",\"input\":\"{}\"}}", text.replace('"', "\\\""))
            }
            "exec_console" | "console_exec" | "run_console" => {
                if let Some(text) = extract_cmd_val(cmd, "text") {
                    console.py_entry.set_text(&text);
                }
                console.switch_to_python();
                console.py_entry.emit_activate();
                "{\"status\":\"ok\"}".to_string()
            }
            "console_print" | "print" => {
                let text = extract_cmd_val(cmd, "text").unwrap_or_default();
                console.print_output(&text);
                "{\"status\":\"ok\"}".to_string()
            }
            "console_clear" | "clear_console" => {
                console.clear();
                "{\"status\":\"ok\"}".to_string()
            }

            other => {
                format!("{{\"status\":\"error\",\"message\":\"Неизвестная команда: {}\"}}", other)
            }
        }
    }
}

fn extract_cmd_val(json: &str, field: &str) -> Option<String> {
    let key = format!("\"{}\"", field);
    let key_pos = json.find(&key)?;
    let after_key = &json[key_pos + key.len()..];
    let colon_pos = after_key.find(':')?;
    let after_colon = after_key[colon_pos + 1..].trim_start();

    if !after_colon.starts_with('"') {
        return None;
    }

    let end_quote = after_colon[1..].find('"')?;
    Some(after_colon[1..1 + end_quote].to_string())
}

fn format_state(f: &RobotField, status: &str, err_msg: Option<&str>) -> String {
    let error_part = if let Some(m) = err_msg {
        format!(",\"message\":\"{}\"", m.replace('"', "\\\""))
    } else {
        "".to_string()
    };

    format!(
        "{{\"status\":\"{}\",\"x\":{},\"y\":{},\"is_painted\":{},\"crashed\":{},\"wall_up\":{},\"wall_down\":{},\"wall_left\":{},\"wall_right\":{}{}}}",
        status,
        f.robot_x + 1,
        f.robot_y + 1,
        f.is_painted(),
        f.crashed,
        f.has_wall(&Direction::Up),
        f.has_wall(&Direction::Down),
        f.has_wall(&Direction::Left),
        f.has_wall(&Direction::Right),
        error_part
    )
}