use crate::plugins::ipc::IpcServer;
use crate::plugins::PluginMetadata;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::sync::{Mutex, OnceLock};

pub struct LoadedPlugin {
    pub metadata: PluginMetadata,
    pub process: Option<Child>,
}

pub struct PluginEngine {
    plugins: HashMap<String, LoadedPlugin>,
}

static ENGINE_INSTANCE: OnceLock<Mutex<PluginEngine>> = OnceLock::new();

impl PluginEngine {
    pub fn global() -> &'static Mutex<PluginEngine> {
        ENGINE_INSTANCE.get_or_init(|| {
            let mut engine = PluginEngine {
                plugins: HashMap::new(),
            };
            engine.rescan_plugins();
            Mutex::new(engine)
        })
    }

    pub fn plugins_dir() -> PathBuf {
        let base = glib::user_data_dir();
        let path = base.join("neomir").join("plugins");
        let _ = fs::create_dir_all(&path);
        path
    }

    fn sync_bundled_plugins(&self) {
        let user_dir = Self::plugins_dir();
        let mut bundled_candidates = vec![
            PathBuf::from("plugins"),
            PathBuf::from("/app/share/neomir/plugins"),
            PathBuf::from("/usr/share/neomir/plugins"),
        ];

        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                bundled_candidates.push(parent.join("plugins"));
                bundled_candidates.push(parent.join("../share/neomir/plugins"));
            }
        }

        for cand in &bundled_candidates {
            if cand.exists() && cand.is_dir() {
                if let Ok(entries) = fs::read_dir(cand) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() && path.join("plugin.json").exists() {
                            let plugin_id = entry.file_name();
                            let target = user_dir.join(&plugin_id);
                            if !target.exists() {
                                let _ = copy_dir_recursive(&path, &target);
                            } else if let Ok(file_entries) = fs::read_dir(&path) {
                                for file in file_entries.flatten() {
                                    let fname = file.file_name();
                                    if fname != "plugin.json" {
                                        let src_f = path.join(&fname);
                                        let dst_f = target.join(&fname);
                                        let _ = fs::copy(&src_f, &dst_f);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn rescan_plugins(&mut self) {
        self.sync_bundled_plugins();
        let dir = Self::plugins_dir();
        let mut new_map = HashMap::new();

        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let manifest = path.join("plugin.json");
                    if manifest.exists() {
                        if let Ok(content) = fs::read_to_string(&manifest) {
                            if let Some(meta) = PluginMetadata::from_json(&content) {
                                // Сохраняем запущенный процесс, если плагин уже был загружен
                                let proc = self.plugins.remove(&meta.id).and_then(|lp| lp.process);
                                new_map.insert(
                                    meta.id.clone(),
                                    LoadedPlugin {
                                        metadata: meta,
                                        process: proc,
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }

        self.plugins = new_map;
    }

    pub fn list_plugins(&self) -> Vec<PluginMetadata> {
        let mut list: Vec<PluginMetadata> = self.plugins.values().map(|lp| lp.metadata.clone()).collect();
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    }

    pub fn install_plugin(&mut self, file_path: &Path) -> Result<PluginMetadata, String> {
        if !file_path.exists() {
            return Err("Файл не найден".to_string());
        }

        let temp_dir = std::env::temp_dir().join(format!("neomir_plug_{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;

        // Распаковываем установочный архив tar.gz (.plug)
        let status = Command::new("tar")
            .arg("-xzf")
            .arg(file_path)
            .arg("-C")
            .arg(&temp_dir)
            .status();

        let manifest_path = temp_dir.join("plugin.json");

        if status.is_err() || !status.unwrap().success() || !manifest_path.exists() {
            // Проверяем, является ли переданный файл одиночным манифестом plugin.json
            if let Ok(content) = fs::read_to_string(file_path) {
                if let Some(meta) = PluginMetadata::from_json(&content) {
                    let target_dir = Self::plugins_dir().join(&meta.id);
                    fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
                    fs::write(target_dir.join("plugin.json"), meta.to_json()).map_err(|e| e.to_string())?;
                    let _ = fs::remove_dir_all(&temp_dir);
                    self.rescan_plugins();
                    return Ok(meta);
                }
            }
            let _ = fs::remove_dir_all(&temp_dir);
            return Err("Не удалось распаковать .plug пакет или найти корректный plugin.json".to_string());
        }

        let content = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Ошибка чтения plugin.json: {}", e))?;
        let meta = PluginMetadata::from_json(&content)
            .ok_or_else(|| "Некорректная структура файла plugin.json".to_string())?;

        if meta.id.trim().is_empty() {
            let _ = fs::remove_dir_all(&temp_dir);
            return Err("Идентификатор плагина (id) не может быть пустым".to_string());
        }

        // Рекурсивно копируем файлы плагина в целевую директорию
        let target_dir = Self::plugins_dir().join(&meta.id);
        let _ = fs::remove_dir_all(&target_dir);
        fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;

        copy_dir_recursive(&temp_dir, &target_dir).map_err(|e| e.to_string())?;

        let _ = fs::remove_dir_all(&temp_dir);
        self.rescan_plugins();
        Ok(meta)
    }

    pub fn set_plugin_enabled(&mut self, id: &str, enabled: bool) -> Result<(), String> {
        let manifest = Self::plugins_dir().join(id).join("plugin.json");
        if !manifest.exists() {
            return Err("Плагин не найден".to_string());
        }

        let content = fs::read_to_string(&manifest).map_err(|e| e.to_string())?;
        if let Some(mut meta) = PluginMetadata::from_json(&content) {
            meta.enabled = enabled;
            fs::write(&manifest, meta.to_json()).map_err(|e| e.to_string())?;

            if !enabled {
                self.stop_plugin(id);
            }

            self.rescan_plugins();

            if enabled {
                let _ = self.launch_plugin(id);
            }

            Ok(())
        } else {
            Err("Ошибка чтения манифеста плагина".to_string())
        }
    }

    pub fn delete_plugin(&mut self, id: &str) -> Result<(), String> {
        self.stop_plugin(id);
        let dir = Self::plugins_dir().join(id);
        if dir.exists() {
            fs::remove_dir_all(&dir).map_err(|e| e.to_string())?;
        }
        self.rescan_plugins();
        Ok(())
    }

    pub fn launch_plugin(&mut self, id: &str) -> Result<(), String> {
        let plugin = self.plugins.get_mut(id).ok_or_else(|| "Плагин не найден в реестре".to_string())?;

        let entry = match plugin.metadata.entry {
            Some(ref e) if !e.trim().is_empty() => e.clone(),
            _ => return Err("У плагина не задана точка входа (entry)".to_string()),
        };

        let dir = Self::plugins_dir().join(id);
        let script_path = dir.join(&entry);
        if !script_path.exists() {
            return Err(format!("Файл точки входа не найден: {}", script_path.display()));
        }

        // Если процесс плагина уже активен, не создаем повторный экземпляр
        if let Some(ref mut child) = plugin.process {
            if let Ok(None) = child.try_wait() {
                return Ok(()); // Уже запущен
            }
        }

        let sock_path = IpcServer::socket_path();

        let child = if entry.ends_with(".py") {
            Command::new("python3")
                .arg(&script_path)
                .current_dir(&dir)
                .env("NEOMIR_SOCKET", sock_path.to_string_lossy().to_string())
                .env("PYTHONPATH", &dir)
                .spawn()
                .map_err(|e| format!("Ошибка запуска процесса плагина (python3): {}", e))?
        } else {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = std::fs::metadata(&script_path) {
                    let mut perms = metadata.permissions();
                    let mode = perms.mode();
                    if mode & 0o111 == 0 {
                        perms.set_mode(mode | 0o755);
                        let _ = std::fs::set_permissions(&script_path, perms);
                    }
                }
            }

            Command::new(&script_path)
                .current_dir(&dir)
                .env("NEOMIR_SOCKET", sock_path.to_string_lossy().to_string())
                .spawn()
                .map_err(|e| format!("Ошибка запуска нативного бинарника плагина: {}", e))?
        };

        plugin.process = Some(child);
        Ok(())
    }

    pub fn stop_plugin(&mut self, id: &str) {
        if let Some(plugin) = self.plugins.get_mut(id) {
            if let Some(mut child) = plugin.process.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }

    pub fn stop_all(&mut self) {
        for plugin in self.plugins.values_mut() {
            if let Some(mut child) = plugin.process.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }

    pub fn load_auto_start_plugins(&mut self) {
        let enabled_ids: Vec<String> = self.plugins.values()
            .filter(|lp| lp.metadata.enabled && lp.metadata.entry.is_some())
            .map(|lp| lp.metadata.id.clone())
            .collect();

        for id in enabled_ids {
            let _ = self.launch_plugin(&id);
        }
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
