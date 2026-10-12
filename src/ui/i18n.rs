#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    Ru,
    En,
}

impl Language {
    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "en" | "english" => Language::En,
            _ => Language::Ru,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::Ru => "ru",
            Language::En => "en",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Ru => "Русский (Russian)",
            Language::En => "English",
        }
    }

    pub fn is_en(&self) -> bool {
        *self == Language::En
    }

    pub fn tr<'a>(&self, key: &'a str) -> &'a str {
        match (self, key) {
            // Main Window Header & Actions
            (Language::En, "run") => "Run",
            (Language::Ru, "run") => "Выполнить",
            (Language::En, "pause") => "Pause",
            (Language::Ru, "pause") => "Пауза",
            (Language::En, "step") => "Step",
            (Language::Ru, "step") => "Шаг",
            (Language::En, "reset") => "Reset",
            (Language::Ru, "reset") => "Сброс",
            (Language::En, "open") => "Open",
            (Language::Ru, "open") => "Загрузить",
            (Language::En, "save") => "Save",
            (Language::Ru, "save") => "Сохранить",
            (Language::En, "settings") => "Settings",
            (Language::Ru, "settings") => "Настройки",
            (Language::En, "extensions") => "Extensions",
            (Language::Ru, "extensions") => "Расширения",
            (Language::En, "py_console") => "Python Console",
            (Language::Ru, "py_console") => "Консоль Python",
            (Language::En, "about") => "About Pole",
            (Language::Ru, "about") => "О программе",
            (Language::En, "menu_load") => "Open Algorithm...",
            (Language::Ru, "menu_load") => "Загрузить алгоритм...",
            (Language::En, "menu_save") => "Save Algorithm...",
            (Language::Ru, "menu_save") => "Сохранить алгоритм...",
            (Language::En, "menu_reset") => "Stop / Reset Field",
            (Language::Ru, "menu_reset") => "Стоп / Сброс поля",
            (Language::En, "menu_step_back") => "Return Robot Back",
            (Language::Ru, "menu_step_back") => "Возврат робота обратно",
            (Language::En, "syntax_kumir") => "Syntax: KuMir",
            (Language::Ru, "syntax_kumir") => "Синтаксис: КуМир",
            (Language::En, "syntax_python") => "Syntax: Python",
            (Language::Ru, "syntax_python") => "Синтаксис: Python",
            (Language::En, "tip_step_back") => "Return robot back one step (Ctrl+Z)",
            (Language::Ru, "tip_step_back") => "Вернуть робота назад на один шаг (Ctrl+Z)",
            (Language::En, "tip_syntax_toggle") => "Toggle syntax highlighting between KuMir and Python",
            (Language::Ru, "tip_syntax_toggle") => "Переключить синтаксис редактора между КуМир и Python",
            (Language::En, "status_robot_backed") => "Robot returned back ({}, {})",
            (Language::Ru, "status_robot_backed") => "Робот возвращён назад ({}, {})",
            (Language::En, "status_already_at_start") => "Performer is already at start position ({}, {})",
            (Language::Ru, "status_already_at_start") => "Исполнитель уже в исходной позиции ({}, {})",
            (Language::En, "python_run_started") => "Running Python script...",
            (Language::Ru, "python_run_started") => "Запуск скрипта Python...",
            (Language::En, "python_run_done") => "Python script completed.",
            (Language::Ru, "python_run_done") => "Скрипт Python успешно завершён.",
            (Language::En, "main_menu") => "Main Menu",
            (Language::Ru, "main_menu") => "Главное меню",
            (Language::En, "cancel") => "Cancel",
            (Language::Ru, "cancel") => "Отмена",
            (Language::En, "ok") => "OK",
            (Language::Ru, "ok") => "OK",
            (Language::En, "enable") => "Enable",
            (Language::Ru, "enable") => "Включить",

            // Tooltips
            (Language::En, "tip_run") => "Run program execution ({})",
            (Language::Ru, "tip_run") => "Запустить выполнение программы ({})",
            (Language::En, "tip_step") => "Execute single step ({})",
            (Language::Ru, "tip_step") => "Выполнить один шаг программы ({})",
            (Language::En, "tip_reset") => "Reset field and execution ({})",
            (Language::Ru, "tip_reset") => "Сбросить поле в исходное состояние ({})",
            (Language::En, "tip_open") => "Open algorithm file",
            (Language::Ru, "tip_open") => "Загрузить файл алгоритма",
            (Language::En, "tip_save") => "Save algorithm file ({})",
            (Language::Ru, "tip_save") => "Сохранить файл алгоритма ({})",

            // Field Tools
            (Language::En, "field_label") => "Field:",
            (Language::Ru, "field_label") => "Поле:",
            (Language::En, "walls") => "Walls",
            (Language::Ru, "walls") => "Стены",
            (Language::En, "tip_clear_walls") => "Remove all walls from the field",
            (Language::Ru, "tip_clear_walls") => "Удалить все стены на поле",
            (Language::En, "clear_paint") => "Clear paint",
            (Language::Ru, "clear_paint") => "Стереть краску",
            (Language::En, "tip_clear_paint") => "Clear all painted cells",
            (Language::Ru, "tip_clear_paint") => "Очистить все закрашенные клетки",

            // Window Titles & Subtitles
            (Language::En, "title_robot") => "Performer: Robot | Field: {}×{}",
            (Language::Ru, "title_robot") => "Исполнитель: Робот | Поле: {}×{}",

            // Status Bar Messages
            (Language::En, "status_ready") => "Ready",
            (Language::Ru, "status_ready") => "Готов к работе",
            (Language::En, "status_active_robot") => "Active performer: Robot",
            (Language::Ru, "status_active_robot") => "Активен исполнитель: Робот",
            (Language::En, "status_running") => "Running program...",
            (Language::Ru, "status_running") => "Выполнение программы...",
            (Language::En, "status_finished") => "Execution finished",
            (Language::Ru, "status_finished") => "Выполнение завершено",
            (Language::En, "status_step_done") => "Step executed",
            (Language::Ru, "status_step_done") => "Шаг выполнен",
            (Language::En, "status_reset_done") => "Field and robot reset",
            (Language::Ru, "status_reset_done") => "Поле и робот сброшены",
            (Language::En, "status_code_error") => "Code error",
            (Language::Ru, "status_code_error") => "Ошибка в коде",

            // Console output messages
            (Language::En, "compiling") => "Compiling...",
            (Language::Ru, "compiling") => "Компиляция...",
            (Language::En, "prog_success") => "Program completed successfully.",
            (Language::Ru, "prog_success") => "Программа успешно завершена.",
            (Language::En, "field_reset_msg") => "Field reset to initial state.",
            (Language::Ru, "field_reset_msg") => "Поле сброшено в исходную позицию.",
            (Language::En, "tag_error") => "[Error] ",
            (Language::Ru, "tag_error") => "[Ошибка] ",
            (Language::En, "tag_ok") => "[OK] ",
            (Language::Ru, "tag_ok") => "[OK] ",
            (Language::En, "tag_info") => "[Info] ",
            (Language::Ru, "tag_info") => "[Инфо] ",

            // Console Tabs & Labels
            (Language::En, "tab_output") => "Output",
            (Language::Ru, "tab_output") => "Вывод",
            (Language::En, "tab_log") => "Log",
            (Language::Ru, "tab_log") => "Журнал",
            (Language::En, "tip_terminal") => "Launch Python in system terminal",
            (Language::Ru, "tip_terminal") => "Запустить Python в системном терминале",
            (Language::En, "tip_clear_console") => "Clear console",
            (Language::Ru, "tip_clear_console") => "Очистить консоль",
            (Language::En, "py_placeholder") => "Enter Python command (e.g. up(), right(), paint())...",
            (Language::Ru, "py_placeholder") => "Введите команду Python (например: вверх(), вправо(), закрасить())...",

            // File Dialogs
            (Language::En, "dialog_open_title") => "Open KuMir Program",
            (Language::Ru, "dialog_open_title") => "Загрузить алгоритм",
            (Language::En, "dialog_save_title") => "Save KuMir Program",
            (Language::Ru, "dialog_save_title") => "Сохранить алгоритм",
            (Language::En, "filter_kumir") => "KuMir Programs (*.kum, *.kumir)",
            (Language::Ru, "filter_kumir") => "КуМир программы (*.kum, *.kumir)",
            (Language::En, "filter_all") => "All Files (*.*)",
            (Language::Ru, "filter_all") => "Все файлы (*.*)",
            (Language::En, "btn_open") => "Open",
            (Language::Ru, "btn_open") => "Открыть",
            (Language::En, "file_loaded") => "Loaded file: {}",
            (Language::Ru, "file_loaded") => "Загружен файл: {}",
            (Language::En, "file_saved") => "File saved successfully: {}",
            (Language::Ru, "file_saved") => "Файл успешно сохранён: {}",

            // Settings Dialog
            (Language::En, "settings_title") => "Pole Settings",
            (Language::Ru, "settings_title") => "Настройки Поле",
            (Language::En, "tab_general") => "General",
            (Language::Ru, "tab_general") => "Основные",
            (Language::En, "tab_shortcuts") => "Shortcuts",
            (Language::Ru, "tab_shortcuts") => "Горячие клавиши",
            (Language::En, "tab_extensions") => "Extensions",
            (Language::Ru, "tab_extensions") => "Расширения",

            // Settings: Appearance
            (Language::En, "grp_appearance") => "Appearance & Ptyxis Themes",
            (Language::Ru, "grp_appearance") => "Внешний вид и темы Ptyxis",
            (Language::En, "desc_appearance") => "Choose color scheme for code editor and interface",
            (Language::Ru, "desc_appearance") => "Выбор цветовой схемы Ptyxis для редактора кода и интерфейса",
            (Language::En, "theme_title") => "Color Theme",
            (Language::Ru, "theme_title") => "Тема оформления",
            (Language::En, "theme_sub") => "Syntax highlighting and background colors",
            (Language::Ru, "theme_sub") => "Цвета подсветки синтаксиса и фона",

            // Settings: Language
            (Language::En, "grp_language") => "Language & Localization",
            (Language::Ru, "grp_language") => "Язык и локализация",
            (Language::En, "desc_language") => "Choose application interface language",
            (Language::Ru, "desc_language") => "Выбор языка интерфейса приложения",
            (Language::En, "lang_title") => "Interface Language",
            (Language::Ru, "lang_title") => "Язык интерфейса",
            (Language::En, "cur_lang_ru") => "Current language: Russian",
            (Language::Ru, "cur_lang_ru") => "Текущий язык: Русский",
            (Language::En, "cur_lang_en") => "Current language: English",
            (Language::Ru, "cur_lang_en") => "Текущий язык: English",

            // Settings: Performer

            // Settings: Editor & Vim
            (Language::En, "grp_editor") => "Editor",
            (Language::Ru, "grp_editor") => "Редактор",
            (Language::En, "desc_editor") => "Text input options and control modes",
            (Language::Ru, "desc_editor") => "Параметры ввода текста и режимы управления",
            (Language::En, "vim_title") => "Vim Mode (modal editing)",
            (Language::Ru, "vim_title") => "Режим Vim (модальное редактирование)",
            (Language::En, "vim_sub") => "Keys h, j, k, l, i, w, b, x, u, dd for advanced users",
            (Language::Ru, "vim_sub") => "Клавиши h, j, k, l, i, w, b, x, u, dd для профессионалов",
            (Language::En, "vim_alert_title") => "Enable Vim Mode?",
            (Language::Ru, "vim_alert_title") => "Включить режим Vim?",
            (Language::En, "vim_alert_body") => "Vim mode is designed for experienced users familiar with modal editing (NORMAL / INSERT / VISUAL).\n\nPress i to insert text, Esc to return to command mode.",
            (Language::Ru, "vim_alert_body") => "Режим Vim предназначен для опытных пользователей, знакомых с модальным управлением (NORMAL / INSERT / VISUAL).\n\nНажмите i для ввода текста, Esc для возврата в командный режим.",

            // Settings: Shortcuts
            (Language::En, "grp_shortcuts") => "Environment Controls",
            (Language::Ru, "grp_shortcuts") => "Управление средой",
            (Language::En, "desc_shortcuts") => "Click a shortcut button to record a new key combination",
            (Language::Ru, "desc_shortcuts") => "Нажмите кнопку комбинации для назначения новой клавиши",
            (Language::En, "sc_run") => "Run Program",
            (Language::Ru, "sc_run") => "Выполнить программу",
            (Language::En, "sc_step") => "Step Program",
            (Language::Ru, "sc_step") => "Шаг программы",
            (Language::En, "sc_reset") => "Reset Field & Execution",
            (Language::Ru, "sc_reset") => "Сброс поля и выполнения",
            (Language::En, "sc_save") => "Save File",
            (Language::Ru, "sc_save") => "Сохранить файл",
            (Language::En, "btn_defaults") => "Reset to Defaults",
            (Language::Ru, "btn_defaults") => "Сбросить по умолчанию",
            (Language::En, "prompt_record") => "Press any key combination (e.g. F5, Ctrl+R, Shift+F10)...",
            (Language::Ru, "prompt_record") => "Нажмите любую комбинацию клавиш (например, F5, Ctrl+R, Shift+F10)...",

            // Settings: Extensions
            (Language::En, "grp_pkg_install") => "Package Installation",
            (Language::Ru, "grp_pkg_install") => "Установка пакетов",
            (Language::En, "desc_pkg_install") => "Standalone NeoMir plugins in .plug package format are supported",
            (Language::Ru, "desc_pkg_install") => "Поддерживаются самостоятельные плагины NeoMir в формате пакетов .plug",
            (Language::En, "install_from_file") => "Install Extension from File",
            (Language::Ru, "install_from_file") => "Установить расширение из файла",
            (Language::En, "choose_plug_sub") => "Select a .plug package to install",
            (Language::Ru, "choose_plug_sub") => "Выберите пакет .plug для установки",
            (Language::En, "btn_choose_plug") => "Select .plug...",
            (Language::Ru, "btn_choose_plug") => "Выбрать .plug...",
            (Language::En, "grp_installed_ext") => "Installed Extensions",
            (Language::Ru, "grp_installed_ext") => "Установленные расширения",
            (Language::En, "no_installed_ext") => "No installed extensions",
            (Language::Ru, "no_installed_ext") => "Нет установленных расширений",
            (Language::En, "no_ext_sub") => "Install a .plug package using the button above",
            (Language::Ru, "no_ext_sub") => "Установите пакет .plug через кнопку выше",
            (Language::En, "filter_plug") => "NeoMir Plugins (*.plug)",
            (Language::Ru, "filter_plug") => "Плагины NeoMir (*.plug)",
            (Language::En, "error") => "Error",
            (Language::Ru, "error") => "Ошибка",
            (Language::En, "ext_status_fail") => "Failed to update extension status: {}",
            (Language::Ru, "ext_status_fail") => "Не удалось изменить статус расширения: {}",
            (Language::En, "ext_installed") => "Extension Installed",
            (Language::Ru, "ext_installed") => "Расширение установлено",
            (Language::En, "ext_installed_msg") => "Successfully installed extension: {} (version {})",
            (Language::Ru, "ext_installed_msg") => "Успешно установлено расширение: {} (версия {})",
            (Language::En, "ext_install_fail") => "Failed to install extension: {}",
            (Language::Ru, "ext_install_fail") => "Не удалось установить расширение: {}",

            // About Dialog
            (Language::En, "about_comments") => "Modern educational programming environment for KuMir algorithmic language with Robot performer.
Developed with Rust, GTK4, and Libadwaita.",
            (Language::Ru, "about_comments") => "Современная среда учебного программирования на алгоритмическом языке КуМир с Исполнителем «Робот».
Разработано на Rust, GTK4 и Libadwaita.",
            (Language::En, "about_devs") => "Pole Developers",
            (Language::Ru, "about_devs") => "Разработчики Поле",

            // Language Changed Alert
            (Language::En, "lang_changed_title") => "Interface Language",
            (Language::Ru, "lang_changed_title") => "Язык интерфейса",
            (Language::En, "lang_changed_body") => "Interface language changed to English. Settings and window elements updated.",
            (Language::Ru, "lang_changed_body") => "Язык интерфейса изменён на Русский. Настройки и элементы окна обновлены.",

            _ => key,
        }
    }
}
