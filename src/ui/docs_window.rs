use crate::ui::i18n::Language;
use adw::prelude::*;
use gtk::gdk;
use gtk::glib;
use gtk::prelude::*;

pub struct DocsWindow;

impl DocsWindow {
    pub fn show(parent: &impl IsA<gtk::Window>, lang: Language) {
        let is_en = lang.is_en();
        let win_title = if is_en { "Pole Documentation" } else { "Документация Pole" };

        let window = adw::PreferencesWindow::builder()
            .title(win_title)
            .default_width(940)
            .default_height(680)
            .transient_for(parent)
            .modal(false)
            .build();

        window.set_search_enabled(true);

        // Load custom styling for code cards, syntax badges, and keyboard shortcuts
        let provider = gtk::CssProvider::new();
        provider.load_from_string(
            "
            .docs-code-card {
                background: alpha(@theme_base_color, 0.7);
                border: 1px solid alpha(@borders, 0.7);
                border-radius: 12px;
            }
            .docs-code-header {
                background: alpha(@theme_bg_color, 0.45);
                border-top-left-radius: 12px;
                border-top-right-radius: 12px;
            }
            .docs-lang-badge {
                background-color: alpha(@accent_color, 0.18);
                color: @accent_color;
                font-size: 0.82em;
                font-weight: bold;
                padding: 3px 10px;
                border-radius: 9999px;
            }
            .docs-code-text {
                font-size: 13.5px;
                line-height: 1.5;
            }
            .docs-shortcut-badge {
                background: alpha(@theme_fg_color, 0.1);
                border: 1px solid alpha(@borders, 0.8);
                border-radius: 6px;
                padding: 2px 8px;
                font-weight: bold;
            }
            ",
        );
        if let Some(display) = gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        // Add 6 comprehensive documentation pages
        window.add(&Self::build_kumir_page(is_en));
        window.add(&Self::build_robot_page(is_en));
        window.add(&Self::build_turtle_page(is_en));
        window.add(&Self::build_python_api_page(is_en));
        window.add(&Self::build_blocks_page(is_en));
        window.add(&Self::build_shortcuts_page(is_en));

        window.present();
    }

    fn create_cmd_row(cmd_markup: &str, desc: &str) -> adw::ActionRow {
        let row = adw::ActionRow::new();
        row.set_use_markup(true);
        row.set_title(cmd_markup);
        row.set_subtitle(desc);
        row.set_subtitle_lines(0);
        row.set_activatable(false);
        row
    }

    fn create_shortcut_row(key: &str, desc: &str) -> adw::ActionRow {
        let row = adw::ActionRow::new();
        row.set_use_markup(true);
        row.set_title(desc);
        row.set_activatable(false);

        let badge = gtk::Label::new(Some(key));
        badge.add_css_class("monospace");
        badge.add_css_class("docs-shortcut-badge");
        badge.set_valign(gtk::Align::Center);
        row.add_suffix(&badge);

        row
    }

    fn create_code_card(
        title: &str,
        lang_badge: &str,
        code: &str,
        highlighted_markup: &str,
        copy_tooltip: &str,
    ) -> gtk::Box {
        let card = gtk::Box::new(gtk::Orientation::Vertical, 0);
        card.add_css_class("card");
        card.add_css_class("docs-code-card");
        card.set_margin_top(6);
        card.set_margin_bottom(12);

        // Header bar with title, language badge, and copy button
        let header = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        header.add_css_class("docs-code-header");
        header.set_margin_start(14);
        header.set_margin_end(10);
        header.set_margin_top(8);
        header.set_margin_bottom(8);

        let title_lbl = gtk::Label::new(Some(title));
        title_lbl.add_css_class("heading");
        title_lbl.set_xalign(0.0);
        title_lbl.set_hexpand(true);
        header.append(&title_lbl);

        let badge = gtk::Label::new(Some(lang_badge));
        badge.add_css_class("docs-lang-badge");
        header.append(&badge);

        let copy_btn = gtk::Button::from_icon_name("edit-copy-symbolic");
        copy_btn.add_css_class("flat");
        copy_btn.set_tooltip_text(Some(copy_tooltip));

        let code_string = code.to_string();
        let btn_clone = copy_btn.clone();
        copy_btn.connect_clicked(move |_| {
            if let Some(display) = gdk::Display::default() {
                display.clipboard().set_text(&code_string);
            }
            btn_clone.set_icon_name("emblem-ok-symbolic");
            let btn_reset = btn_clone.clone();
            glib::timeout_add_local_once(std::time::Duration::from_millis(1500), move || {
                btn_reset.set_icon_name("edit-copy-symbolic");
            });
        });
        header.append(&copy_btn);

        card.append(&header);
        card.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

        // Code display area
        let code_scroll = gtk::ScrolledWindow::new();
        code_scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Never);
        code_scroll.set_has_frame(false);

        let code_lbl = gtk::Label::new(None);
        code_lbl.set_markup(highlighted_markup);
        code_lbl.set_selectable(true);
        code_lbl.set_xalign(0.0);
        code_lbl.set_margin_start(16);
        code_lbl.set_margin_end(16);
        code_lbl.set_margin_top(12);
        code_lbl.set_margin_bottom(12);
        code_lbl.add_css_class("monospace");
        code_lbl.add_css_class("docs-code-text");

        code_scroll.set_child(Some(&code_lbl));
        card.append(&code_scroll);

        card
    }

    fn build_kumir_page(is_en: bool) -> adw::PreferencesPage {
        let page = adw::PreferencesPage::new();
        page.set_title(if is_en { "KuMir Language" } else { "Язык КуМир" });
        page.set_icon_name(Some("format-text-code-symbolic"));
        page.set_name(Some("kumir"));

        if is_en {
            // Group 1: Program Structure
            let grp_struct = adw::PreferencesGroup::new();
            grp_struct.set_title("Program Structure");
            grp_struct.set_description(Some("Basic layout, algorithm declarations, and I/O operators in KuMir"));

            grp_struct.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">алг</span> &lt;имя&gt;",
                "Declares the beginning of an algorithm with its unique name",
            ));
            grp_struct.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">нач ... кон</span>",
                "Executable statements are enclosed between 'нач' (begin) and 'кон' (end)",
            ));
            grp_struct.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">исп</span> &lt;Исполнитель&gt;",
                "Include performer module (e.g. 'исп Робот'), placed before 'алг'",
            ));
            grp_struct.add(&Self::create_cmd_row(
                "<span foreground=\"#9141ac\" weight=\"bold\">вывод</span> &lt;выражения&gt;, <span foreground=\"#9141ac\" weight=\"bold\">нс</span>",
                "Prints values or text to the output console; 'нс' prints a newline",
            ));
            grp_struct.add(&Self::create_cmd_row(
                "<span foreground=\"#9141ac\" weight=\"bold\">ввод</span> &lt;переменные&gt;",
                "Reads values from the input console into specified variables",
            ));

            grp_struct.add(&Self::create_code_card(
                "Basic Algorithm Example",
                "KuMir",
                "алг Приветствие\nнач\n  вывод \"Привет, мир!\", нс\nкон",
                "<span foreground=\"#3584e4\" weight=\"bold\">алг</span> Приветствие\n<span foreground=\"#3584e4\" weight=\"bold\">нач</span>\n  <span foreground=\"#9141ac\" weight=\"bold\">вывод</span> <span foreground=\"#26a269\">\"Привет, мир!\"</span>, <span foreground=\"#9141ac\" weight=\"bold\">нс</span>\n<span foreground=\"#3584e4\" weight=\"bold\">кон</span>",
                "Copy code example",
            ));
            page.add(&grp_struct);

            // Group 2: Data Types
            let grp_types = adw::PreferencesGroup::new();
            grp_types.set_title("Data Types & Variables");
            grp_types.set_description(Some("Primitive variable types and assignment syntax"));

            grp_types.add(&Self::create_cmd_row("<span foreground=\"#1c71d8\" weight=\"bold\">цел</span>", "Signed 32-bit integers (e.g. 10, -5, 0)"));
            grp_types.add(&Self::create_cmd_row("<span foreground=\"#1c71d8\" weight=\"bold\">вещ</span>", "Floating-point numbers (e.g. 3.1415, -0.5)"));
            grp_types.add(&Self::create_cmd_row("<span foreground=\"#1c71d8\" weight=\"bold\">лог</span>", "Boolean logical values: 'да' (true) or 'нет' (false)"));
            grp_types.add(&Self::create_cmd_row("<span foreground=\"#1c71d8\" weight=\"bold\">сим</span> / <span foreground=\"#1c71d8\" weight=\"bold\">лит</span>", "Single character ('а') or string literal (\"Строка\")"));
            grp_types.add(&Self::create_cmd_row("<span foreground=\"#e66100\" weight=\"bold\">:=</span>", "Assignment operator: variable := expression"));

            grp_types.add(&Self::create_code_card(
                "Working with Variables",
                "KuMir",
                "алг Вычисления\nнач\n  цел а, б, сумма\n  а := 15\n  б := 27\n  сумма := а + б\n  вывод \"Сумма = \", сумма, нс\nкон",
                "<span foreground=\"#3584e4\" weight=\"bold\">алг</span> Вычисления\n<span foreground=\"#3584e4\" weight=\"bold\">нач</span>\n  <span foreground=\"#1c71d8\" weight=\"bold\">цел</span> а, б, сумма\n  а := <span foreground=\"#c061cb\">15</span>\n  б := <span foreground=\"#c061cb\">27</span>\n  сумма := а + б\n  <span foreground=\"#9141ac\" weight=\"bold\">вывод</span> <span foreground=\"#26a269\">\"Сумма = \"</span>, сумма, <span foreground=\"#9141ac\" weight=\"bold\">нс</span>\n<span foreground=\"#3584e4\" weight=\"bold\">кон</span>",
                "Copy code example",
            ));
            page.add(&grp_types);

            // Group 3: Control Structures
            let grp_ctrl = adw::PreferencesGroup::new();
            grp_ctrl.set_title("Control Structures: Conditions & Loops");
            grp_ctrl.set_description(Some("Branching statements and loop constructs"));

            grp_ctrl.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">если ... то ... иначе ... все</span>",
                "Conditional branching; 'иначе' block is optional",
            ));
            grp_ctrl.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">нц N раз ... кц</span>",
                "Loop repeating body statements exactly N times",
            ));
            grp_ctrl.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">нц пока ... кц</span>",
                "Precondition loop running while expression evaluates to true",
            ));
            grp_ctrl.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">нц для i от A до B ... кц</span>",
                "Counter loop incrementing index variable from A to B",
            ));

            grp_ctrl.add(&Self::create_code_card(
                "Condition & Loop Example",
                "KuMir",
                "алг ВетвлениеИЦикл\nнач\n  цел i\n  нц для i от 1 до 5\n    если i > 2 то\n      вывод i, \" больше двух\", нс\n    иначе\n      вывод i, \" не больше двух\", нс\n    все\n  кц\nкон",
                "<span foreground=\"#3584e4\" weight=\"bold\">алг</span> ВетвлениеИЦикл\n<span foreground=\"#3584e4\" weight=\"bold\">нач</span>\n  <span foreground=\"#1c71d8\" weight=\"bold\">цел</span> i\n  <span foreground=\"#3584e4\" weight=\"bold\">нц</span> <span foreground=\"#3584e4\" weight=\"bold\">для</span> i <span foreground=\"#3584e4\" weight=\"bold\">от</span> <span foreground=\"#c061cb\">1</span> <span foreground=\"#3584e4\" weight=\"bold\">до</span> <span foreground=\"#c061cb\">5</span>\n    <span foreground=\"#3584e4\" weight=\"bold\">если</span> i &gt; <span foreground=\"#c061cb\">2</span> <span foreground=\"#3584e4\" weight=\"bold\">то</span>\n      <span foreground=\"#9141ac\" weight=\"bold\">вывод</span> i, <span foreground=\"#26a269\">\" больше двух\"</span>, <span foreground=\"#9141ac\" weight=\"bold\">нс</span>\n    <span foreground=\"#3584e4\" weight=\"bold\">иначе</span>\n      <span foreground=\"#9141ac\" weight=\"bold\">вывод</span> i, <span foreground=\"#26a269\">\" не больше двух\"</span>, <span foreground=\"#9141ac\" weight=\"bold\">нс</span>\n    <span foreground=\"#3584e4\" weight=\"bold\">все</span>\n  <span foreground=\"#3584e4\" weight=\"bold\">кц</span>\n<span foreground=\"#3584e4\" weight=\"bold\">кон</span>",
                "Copy code example",
            ));
            page.add(&grp_ctrl);
        } else {
            // Group 1: Program Structure
            let grp_struct = adw::PreferencesGroup::new();
            grp_struct.set_title("Структура программы");
            grp_struct.set_description(Some("Ключевые слова, структура алгоритма и команды ввода-вывода языка КуМир"));

            grp_struct.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">алг</span> &lt;имя алгоритма&gt;",
                "Объявление начала алгоритма с указанием его названия",
            ));
            grp_struct.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">нач ... кон</span>",
                "Тело алгоритма: исполняемые команды всегда заключаются между 'нач' и 'кон'",
            ));
            grp_struct.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">исп</span> &lt;Исполнитель&gt;",
                "Подключение исполнителя (например, 'исп Робот'), указывается до строки 'алг'",
            ));
            grp_struct.add(&Self::create_cmd_row(
                "<span foreground=\"#9141ac\" weight=\"bold\">вывод</span> &lt;значения&gt;, <span foreground=\"#9141ac\" weight=\"bold\">нс</span>",
                "Вывод данных в консоль; 'нс' осуществляет перевод на новую строку",
            ));
            grp_struct.add(&Self::create_cmd_row(
                "<span foreground=\"#9141ac\" weight=\"bold\">ввод</span> &lt;переменные&gt;",
                "Считывание данных из консоли ввода в указанные переменные",
            ));

            grp_struct.add(&Self::create_code_card(
                "Пример базового алгоритма",
                "КуМир",
                "алг Приветствие\nнач\n  вывод \"Привет, мир!\", нс\nкон",
                "<span foreground=\"#3584e4\" weight=\"bold\">алг</span> Приветствие\n<span foreground=\"#3584e4\" weight=\"bold\">нач</span>\n  <span foreground=\"#9141ac\" weight=\"bold\">вывод</span> <span foreground=\"#26a269\">\"Привет, мир!\"</span>, <span foreground=\"#9141ac\" weight=\"bold\">нс</span>\n<span foreground=\"#3584e4\" weight=\"bold\">кон</span>",
                "Копировать пример в буфер",
            ));
            page.add(&grp_struct);

            // Group 2: Data Types
            let grp_types = adw::PreferencesGroup::new();
            grp_types.set_title("Типы данных и переменные");
            grp_types.set_description(Some("Базовые типы данных и оператор присваивания"));

            grp_types.add(&Self::create_cmd_row("<span foreground=\"#1c71d8\" weight=\"bold\">цел</span>", "Целочисленный тип: положительные и отрицательные целые числа (10, -5, 0)"));
            grp_types.add(&Self::create_cmd_row("<span foreground=\"#1c71d8\" weight=\"bold\">вещ</span>", "Вещественные числа с плавающей точкой (3.1415, -0.5)"));
            grp_types.add(&Self::create_cmd_row("<span foreground=\"#1c71d8\" weight=\"bold\">лог</span>", "Логический тип: принимает значения 'да' (истина) или 'нет' (ложь)"));
            grp_types.add(&Self::create_cmd_row("<span foreground=\"#1c71d8\" weight=\"bold\">сим</span> / <span foreground=\"#1c71d8\" weight=\"bold\">лит</span>", "Одиночный символ ('а') или текстовая строка (\"Привет\")"));
            grp_types.add(&Self::create_cmd_row("<span foreground=\"#e66100\" weight=\"bold\">:=</span>", "Оператор присваивания: переменная := выражение"));

            grp_types.add(&Self::create_code_card(
                "Работа с переменными",
                "КуМир",
                "алг Вычисления\nнач\n  цел а, б, сумма\n  а := 15\n  б := 27\n  сумма := а + б\n  вывод \"Сумма = \", сумма, нс\nкон",
                "<span foreground=\"#3584e4\" weight=\"bold\">алг</span> Вычисления\n<span foreground=\"#3584e4\" weight=\"bold\">нач</span>\n  <span foreground=\"#1c71d8\" weight=\"bold\">цел</span> а, б, сумма\n  а := <span foreground=\"#c061cb\">15</span>\n  б := <span foreground=\"#c061cb\">27</span>\n  сумма := а + б\n  <span foreground=\"#9141ac\" weight=\"bold\">вывод</span> <span foreground=\"#26a269\">\"Сумма = \"</span>, сумма, <span foreground=\"#9141ac\" weight=\"bold\">нс</span>\n<span foreground=\"#3584e4\" weight=\"bold\">кон</span>",
                "Копировать пример в буфер",
            ));
            page.add(&grp_types);

            // Group 3: Control Structures
            let grp_ctrl = adw::PreferencesGroup::new();
            grp_ctrl.set_title("Управляющие конструкции");
            grp_ctrl.set_description(Some("Ветвления и циклические конструкции"));

            grp_ctrl.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">если ... то ... иначе ... все</span>",
                "Ветвление в зависимости от условия; блок 'иначе' может отсутствовать",
            ));
            grp_ctrl.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">нц N раз ... кц</span>",
                "Цикл с фиксированным числом повторений N раз",
            ));
            grp_ctrl.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">нц пока ... кц</span>",
                "Цикл с предусловием (выполняется, пока условие истинно)",
            ));
            grp_ctrl.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">нц для i от A до B ... кц</span>",
                "Цикл со счётчиком i от начального значения A до конечного значения B",
            ));

            grp_ctrl.add(&Self::create_code_card(
                "Пример ветвления и цикла",
                "КуМир",
                "алг ВетвлениеИЦикл\nнач\n  цел i\n  нц для i от 1 до 5\n    если i > 2 то\n      вывод i, \" больше двух\", нс\n    иначе\n      вывод i, \" не больше двух\", нс\n    все\n  кц\nкон",
                "<span foreground=\"#3584e4\" weight=\"bold\">алг</span> ВетвлениеИЦикл\n<span foreground=\"#3584e4\" weight=\"bold\">нач</span>\n  <span foreground=\"#1c71d8\" weight=\"bold\">цел</span> i\n  <span foreground=\"#3584e4\" weight=\"bold\">нц</span> <span foreground=\"#3584e4\" weight=\"bold\">для</span> i <span foreground=\"#3584e4\" weight=\"bold\">от</span> <span foreground=\"#c061cb\">1</span> <span foreground=\"#3584e4\" weight=\"bold\">до</span> <span foreground=\"#c061cb\">5</span>\n    <span foreground=\"#3584e4\" weight=\"bold\">если</span> i &gt; <span foreground=\"#c061cb\">2</span> <span foreground=\"#3584e4\" weight=\"bold\">то</span>\n      <span foreground=\"#9141ac\" weight=\"bold\">вывод</span> i, <span foreground=\"#26a269\">\" больше двух\"</span>, <span foreground=\"#9141ac\" weight=\"bold\">нс</span>\n    <span foreground=\"#3584e4\" weight=\"bold\">иначе</span>\n      <span foreground=\"#9141ac\" weight=\"bold\">вывод</span> i, <span foreground=\"#26a269\">\" не больше двух\"</span>, <span foreground=\"#9141ac\" weight=\"bold\">нс</span>\n    <span foreground=\"#3584e4\" weight=\"bold\">все</span>\n  <span foreground=\"#3584e4\" weight=\"bold\">кц</span>\n<span foreground=\"#3584e4\" weight=\"bold\">кон</span>",
                "Копировать пример в буфер",
            ));
            page.add(&grp_ctrl);
        }

        page
    }

    fn build_robot_page(is_en: bool) -> adw::PreferencesPage {
        let page = adw::PreferencesPage::new();
        page.set_title(if is_en { "Robot Performer" } else { "Исполнитель Робот" });
        page.set_icon_name(Some("applications-engineering-symbolic"));
        page.set_name(Some("robot"));

        if is_en {
            let grp_motion = adw::PreferencesGroup::new();
            grp_motion.set_title("Movement & Action Commands");
            grp_motion.set_description(Some("Commands for moving the Robot on the rectangular grid field"));

            grp_motion.add(&Self::create_cmd_row("<span foreground=\"#e66100\" weight=\"bold\">вправо</span>", "Move Robot one cell to the right"));
            grp_motion.add(&Self::create_cmd_row("<span foreground=\"#e66100\" weight=\"bold\">влево</span>", "Move Robot one cell to the left"));
            grp_motion.add(&Self::create_cmd_row("<span foreground=\"#e66100\" weight=\"bold\">вверх</span>", "Move Robot one cell upwards"));
            grp_motion.add(&Self::create_cmd_row("<span foreground=\"#e66100\" weight=\"bold\">вниз</span>", "Move Robot one cell downwards"));
            grp_motion.add(&Self::create_cmd_row("<span foreground=\"#1c71d8\" weight=\"bold\">закрасить</span>", "Paint current cell (marks it with blue fill)"));
            page.add(&grp_motion);

            let grp_sensors = adw::PreferencesGroup::new();
            grp_sensors.set_title("Sensors & Conditions");
            grp_sensors.set_description(Some("Checking surrounding walls and cell states for 'если' and 'нц пока'"));

            grp_sensors.add(&Self::create_cmd_row(
                "<span foreground=\"#26a269\" weight=\"bold\">справа свободно</span> / <span foreground=\"#c01c28\" weight=\"bold\">справа стена</span>",
                "True if there is no wall (or a wall) immediately to the right",
            ));
            grp_sensors.add(&Self::create_cmd_row(
                "<span foreground=\"#26a269\" weight=\"bold\">слева свободно</span> / <span foreground=\"#c01c28\" weight=\"bold\">слева стена</span>",
                "True if there is no wall (or a wall) immediately to the left",
            ));
            grp_sensors.add(&Self::create_cmd_row(
                "<span foreground=\"#26a269\" weight=\"bold\">сверху свободно</span> / <span foreground=\"#c01c28\" weight=\"bold\">сверху стена</span>",
                "True if there is no wall (or a wall) immediately above",
            ));
            grp_sensors.add(&Self::create_cmd_row(
                "<span foreground=\"#26a269\" weight=\"bold\">снизу свободно</span> / <span foreground=\"#c01c28\" weight=\"bold\">снизу стена</span>",
                "True if there is no wall (or a wall) immediately below",
            ));
            grp_sensors.add(&Self::create_cmd_row(
                "<span foreground=\"#1c71d8\" weight=\"bold\">клетка закрашена</span> / <span foreground=\"#1c71d8\" weight=\"bold\">клетка чистая</span>",
                "True if current cell is already painted (or not painted)",
            ));
            page.add(&grp_sensors);

            let grp_example = adw::PreferencesGroup::new();
            grp_example.set_title("Practical Algorithm Example");
            grp_example.set_description(Some("Moving to the right boundary wall and painting each visited cell"));

            grp_example.add(&Self::create_code_card(
                "Move to Wall and Paint Corridor",
                "KuMir",
                "исп Робот\nалг ЗакраситьКоридор\nнач\n  закрасить\n  нц пока справа свободно\n    вправо\n    закрасить\n  кц\nкон",
                "<span foreground=\"#3584e4\" weight=\"bold\">исп</span> Робот\n<span foreground=\"#3584e4\" weight=\"bold\">алг</span> ЗакраситьКоридор\n<span foreground=\"#3584e4\" weight=\"bold\">нач</span>\n  <span foreground=\"#1c71d8\" weight=\"bold\">закрасить</span>\n  <span foreground=\"#3584e4\" weight=\"bold\">нц</span> <span foreground=\"#3584e4\" weight=\"bold\">пока</span> <span foreground=\"#26a269\">справа свободно</span>\n    <span foreground=\"#e66100\" weight=\"bold\">вправо</span>\n    <span foreground=\"#1c71d8\" weight=\"bold\">закрасить</span>\n  <span foreground=\"#3584e4\" weight=\"bold\">кц</span>\n<span foreground=\"#3584e4\" weight=\"bold\">кон</span>",
                "Copy code example",
            ));
            page.add(&grp_example);
        } else {
            let grp_motion = adw::PreferencesGroup::new();
            grp_motion.set_title("Команды движения и действий");
            grp_motion.set_description(Some("Управление перемещением Робота по прямоугольному клеточному полю"));

            grp_motion.add(&Self::create_cmd_row("<span foreground=\"#e66100\" weight=\"bold\">вправо</span>", "Переместить Робота на одну клетку вправо"));
            grp_motion.add(&Self::create_cmd_row("<span foreground=\"#e66100\" weight=\"bold\">влево</span>", "Переместить Робота на одну клетку влево"));
            grp_motion.add(&Self::create_cmd_row("<span foreground=\"#e66100\" weight=\"bold\">вверх</span>", "Переместить Робота на одну клетку вверх"));
            grp_motion.add(&Self::create_cmd_row("<span foreground=\"#e66100\" weight=\"bold\">вниз</span>", "Переместить Робота на одну клетку вниз"));
            grp_motion.add(&Self::create_cmd_row("<span foreground=\"#1c71d8\" weight=\"bold\">закрасить</span>", "Закрасить клетку, в которой находится Робот"));
            page.add(&grp_motion);

            let grp_sensors = adw::PreferencesGroup::new();
            grp_sensors.set_title("Датчики обстановки");
            grp_sensors.set_description(Some("Проверка наличия стен и состояния клетки для ветвлений и циклов"));

            grp_sensors.add(&Self::create_cmd_row(
                "<span foreground=\"#26a269\" weight=\"bold\">справа свободно</span> / <span foreground=\"#c01c28\" weight=\"bold\">справа стена</span>",
                "Проверка наличия стены непосредственно справа от Робота",
            ));
            grp_sensors.add(&Self::create_cmd_row(
                "<span foreground=\"#26a269\" weight=\"bold\">слева свободно</span> / <span foreground=\"#c01c28\" weight=\"bold\">слева стена</span>",
                "Проверка наличия стены непосредственно слева от Робота",
            ));
            grp_sensors.add(&Self::create_cmd_row(
                "<span foreground=\"#26a269\" weight=\"bold\">сверху свободно</span> / <span foreground=\"#c01c28\" weight=\"bold\">сверху стена</span>",
                "Проверка наличия стены непосредственно сверху от Робота",
            ));
            grp_sensors.add(&Self::create_cmd_row(
                "<span foreground=\"#26a269\" weight=\"bold\">снизу свободно</span> / <span foreground=\"#c01c28\" weight=\"bold\">снизу стена</span>",
                "Проверка наличия стены непосредственно снизу от Робота",
            ));
            grp_sensors.add(&Self::create_cmd_row(
                "<span foreground=\"#1c71d8\" weight=\"bold\">клетка закрашена</span> / <span foreground=\"#1c71d8\" weight=\"bold\">клетка чистая</span>",
                "Проверка: закрашена ли текущая клетка поля",
            ));
            page.add(&grp_sensors);

            let grp_example = adw::PreferencesGroup::new();
            grp_example.set_title("Практический пример");
            grp_example.set_description(Some("Проход вдоль ряда до стены с непрерывным закрашиванием клеток"));

            grp_example.add(&Self::create_code_card(
                "Движение до стены с закрашиванием ряда",
                "КуМир",
                "исп Робот\nалг ЗакраситьКоридор\nнач\n  закрасить\n  нц пока справа свободно\n    вправо\n    закрасить\n  кц\nкон",
                "<span foreground=\"#3584e4\" weight=\"bold\">исп</span> Робот\n<span foreground=\"#3584e4\" weight=\"bold\">алг</span> ЗакраситьКоридор\n<span foreground=\"#3584e4\" weight=\"bold\">нач</span>\n  <span foreground=\"#1c71d8\" weight=\"bold\">закрасить</span>\n  <span foreground=\"#3584e4\" weight=\"bold\">нц</span> <span foreground=\"#3584e4\" weight=\"bold\">пока</span> <span foreground=\"#26a269\">справа свободно</span>\n    <span foreground=\"#e66100\" weight=\"bold\">вправо</span>\n    <span foreground=\"#1c71d8\" weight=\"bold\">закрасить</span>\n  <span foreground=\"#3584e4\" weight=\"bold\">кц</span>\n<span foreground=\"#3584e4\" weight=\"bold\">кон</span>",
                "Копировать пример в буфер",
            ));
            page.add(&grp_example);
        }

        page
    }

    fn build_turtle_page(is_en: bool) -> adw::PreferencesPage {
        let page = adw::PreferencesPage::new();
        page.set_title(if is_en { "Turtle Performer" } else { "Исполнитель Черепаха" });
        page.set_icon_name(Some("draw-freehand-symbolic"));
        page.set_name(Some("turtle"));

        if is_en {
            let grp_coord = adw::PreferencesGroup::new();
            grp_coord.set_title("Coordinate Plane & Movement");
            grp_coord.set_description(Some("Vector graphics: 2D plane with origin (0, 0) at center, heading upwards (90 deg)"));

            grp_coord.add(&Self::create_cmd_row(
                "<span foreground=\"#e66100\" weight=\"bold\">вперёд(n)</span> / forward(n)",
                "Move forward by n pixels along current heading",
            ));
            grp_coord.add(&Self::create_cmd_row(
                "<span foreground=\"#e66100\" weight=\"bold\">назад(n)</span> / backward(n)",
                "Move backward by n pixels along current heading",
            ));
            grp_coord.add(&Self::create_cmd_row(
                "<span foreground=\"#e66100\" weight=\"bold\">вправо(угол)</span> / turn_right(deg)",
                "Turn clockwise by specified degrees",
            ));
            grp_coord.add(&Self::create_cmd_row(
                "<span foreground=\"#e66100\" weight=\"bold\">влево(угол)</span> / turn_left(deg)",
                "Turn counter-clockwise by specified degrees",
            ));
            page.add(&grp_coord);

            let grp_pen = adw::PreferencesGroup::new();
            grp_pen.set_title("Pen & Canvas Control");
            grp_pen.set_description(Some("Lowering or raising tail, clearing canvas lines, resetting position"));

            grp_pen.add(&Self::create_cmd_row(
                "<span foreground=\"#26a269\" weight=\"bold\">опустить_хвост()</span> / pen_down()",
                "Lower tail: Turtle draws lines when moving",
            ));
            grp_pen.add(&Self::create_cmd_row(
                "<span foreground=\"#c01c28\" weight=\"bold\">поднять_хвост()</span> / pen_up()",
                "Raise tail: move without leaving drawn traces",
            ));
            grp_pen.add(&Self::create_cmd_row(
                "<span foreground=\"#1c71d8\" weight=\"bold\">очистить_холст()</span> / clear()",
                "Erase all drawn lines from the vector canvas",
            ));
            grp_pen.add(&Self::create_cmd_row(
                "<span foreground=\"#9141ac\" weight=\"bold\">домой()</span> / home()",
                "Return Turtle to origin (0, 0) and reset heading upwards",
            ));
            page.add(&grp_pen);

            let grp_examples = adw::PreferencesGroup::new();
            grp_examples.set_title("Python Code Examples");
            grp_examples.set_description(Some("Geometric shapes and patterns executed via Python"));

            grp_examples.add(&Self::create_code_card(
                "Square with Side 120",
                "Python",
                "# Draw a square:\nfor _ in range(4):\n    вперёд(120)\n    вправо(90)",
                "<span foreground=\"#77767b\" style=\"italic\"># Draw a square:</span>\n<span foreground=\"#e66100\" weight=\"bold\">for</span> _ <span foreground=\"#e66100\" weight=\"bold\">in</span> range(<span foreground=\"#c061cb\">4</span>):\n    вперёд(<span foreground=\"#c061cb\">120</span>)\n    вправо(<span foreground=\"#c061cb\">90</span>)",
                "Copy code example",
            ));

            grp_examples.add(&Self::create_code_card(
                "Five-Point Star",
                "Python",
                "# Draw a 5-point star:\nfor _ in range(5):\n    вперёд(140)\n    вправо(144)",
                "<span foreground=\"#77767b\" style=\"italic\"># Draw a 5-point star:</span>\n<span foreground=\"#e66100\" weight=\"bold\">for</span> _ <span foreground=\"#e66100\" weight=\"bold\">in</span> range(<span foreground=\"#c061cb\">5</span>):\n    вперёд(<span foreground=\"#c061cb\">140</span>)\n    вправо(<span foreground=\"#c061cb\">144</span>)",
                "Copy code example",
            ));
            page.add(&grp_examples);
        } else {
            let grp_coord = adw::PreferencesGroup::new();
            grp_coord.set_title("Координатная плоскость и движение");
            grp_coord.set_description(Some("Векторная графика: центр (0, 0), по умолчанию взгляд направлен вверх (угол 90°)"));

            grp_coord.add(&Self::create_cmd_row(
                "<span foreground=\"#e66100\" weight=\"bold\">вперёд(n)</span> / forward(n)",
                "Переместиться вперёд по направлению взгляда на n пикселей",
            ));
            grp_coord.add(&Self::create_cmd_row(
                "<span foreground=\"#e66100\" weight=\"bold\">назад(n)</span> / backward(n)",
                "Переместиться назад на n пикселей",
            ));
            grp_coord.add(&Self::create_cmd_row(
                "<span foreground=\"#e66100\" weight=\"bold\">вправо(угол)</span> / turn_right(deg)",
                "Повернуть по часовой стрелке на указанный угол в градусах",
            ));
            grp_coord.add(&Self::create_cmd_row(
                "<span foreground=\"#e66100\" weight=\"bold\">влево(угол)</span> / turn_left(deg)",
                "Повернуть против часовой стрелки на указанный угол в градусах",
            ));
            page.add(&grp_coord);

            let grp_pen = adw::PreferencesGroup::new();
            grp_pen.set_title("Управление пером и холстом");
            grp_pen.set_description(Some("Поднятие и опускание хвоста, очистка линий и возвращение в центр"));

            grp_pen.add(&Self::create_cmd_row(
                "<span foreground=\"#26a269\" weight=\"bold\">опустить_хвост()</span> / pen_down()",
                "Опустить хвост: Черепаха оставляет след при движении",
            ));
            grp_pen.add(&Self::create_cmd_row(
                "<span foreground=\"#c01c28\" weight=\"bold\">поднять_хвост()</span> / pen_up()",
                "Поднять хвост: перемещение без рисования линий",
            ));
            grp_pen.add(&Self::create_cmd_row(
                "<span foreground=\"#1c71d8\" weight=\"bold\">очистить_холст()</span> / clear()",
                "Стереть все нарисованные линии с холста",
            ));
            grp_pen.add(&Self::create_cmd_row(
                "<span foreground=\"#9141ac\" weight=\"bold\">домой()</span> / home()",
                "Вернуть Черепаху в центр поля (0, 0) и направить вверх",
            ));
            page.add(&grp_pen);

            let grp_examples = adw::PreferencesGroup::new();
            grp_examples.set_title("Примеры программ на Python");
            grp_examples.set_description(Some("Геометрические фигуры и узоры, запускаемые в режиме Python"));

            grp_examples.add(&Self::create_code_card(
                "Квадрат со стороной 120",
                "Python",
                "# Рисуем квадрат со стороной 120:\nfor _ in range(4):\n    вперёд(120)\n    вправо(90)",
                "<span foreground=\"#77767b\" style=\"italic\"># Рисуем квадрат со стороной 120:</span>\n<span foreground=\"#e66100\" weight=\"bold\">for</span> _ <span foreground=\"#e66100\" weight=\"bold\">in</span> range(<span foreground=\"#c061cb\">4</span>):\n    вперёд(<span foreground=\"#c061cb\">120</span>)\n    вправо(<span foreground=\"#c061cb\">90</span>)",
                "Копировать пример в буфер",
            ));

            grp_examples.add(&Self::create_code_card(
                "Пятиконечная звезда",
                "Python",
                "# Рисуем правильную пятиконечную звезду:\nfor _ in range(5):\n    вперёд(140)\n    вправо(144)",
                "<span foreground=\"#77767b\" style=\"italic\"># Рисуем правильную пятиконечную звезду:</span>\n<span foreground=\"#e66100\" weight=\"bold\">for</span> _ <span foreground=\"#e66100\" weight=\"bold\">in</span> range(<span foreground=\"#c061cb\">5</span>):\n    вперёд(<span foreground=\"#c061cb\">140</span>)\n    вправо(<span foreground=\"#c061cb\">144</span>)",
                "Копировать пример в буфер",
            ));
            page.add(&grp_examples);
        }

        page
    }

    fn build_python_api_page(is_en: bool) -> adw::PreferencesPage {
        let page = adw::PreferencesPage::new();
        page.set_title(if is_en { "PyPole API" } else { "PyPole API" });
        page.set_icon_name(Some("applications-development-symbolic"));
        page.set_name(Some("python_api"));

        if is_en {
            let grp_ide = adw::PreferencesGroup::new();
            grp_ide.set_title("PyCharm & IDE Integration");
            grp_ide.set_description(Some("Connect external Python IDEs and scripts directly to the Pole environment"));

            grp_ide.add(&Self::create_cmd_row(
                "Package Installation",
                "Run <b>pip install -e ./python</b> in terminal or open the Pole repository in PyCharm",
            ));
            grp_ide.add(&Self::create_cmd_row(
                "Automatic IPC Socket Discovery",
                "The library automatically discovers and connects to the active Pole socket (neomir.sock / pole.sock)",
            ));
            grp_ide.add(&Self::create_cmd_row(
                "Real-time Output in PyCharm Console",
                "Calling performer methods prints immediate state and sensor diagnostics directly to the PyCharm Run window",
            ));
            grp_ide.add(&Self::create_cmd_row(
                "Embedded & Isolated Python Runtime",
                "Pole features an isolated execution engine (PYTHONNOUSERSITE=1) with bundled libraries, ensuring zero dependency on host OS Python packages",
            ));
            page.add(&grp_ide);

            let grp_robot = adw::PreferencesGroup::new();
            grp_robot.set_title("Robot API Reference");
            grp_robot.set_description(Some("Methods available on the robot instance"));

            grp_robot.add(&Self::create_cmd_row(
                "robot.right(n), left(n), up(n), down(n)",
                "Move robot by n steps in the specified direction",
            ));
            grp_robot.add(&Self::create_cmd_row(
                "robot.paint(), reset(), step_back()",
                "Paint current cell, return to start position, or undo last action",
            ));
            grp_robot.add(&Self::create_cmd_row(
                "robot.wall_up(), wall_down(), wall_left(), wall_right()",
                "Return True if there is an adjacent wall",
            ));
            grp_robot.add(&Self::create_cmd_row(
                "robot.free_up(), free_down(), free_left(), free_right()",
                "Return True if the corresponding direction is free of walls",
            ));
            grp_robot.add(&Self::create_cmd_row(
                "robot.is_painted(), is_clean()",
                "Check whether the current cell is painted or clean",
            ));
            page.add(&grp_robot);

            let grp_turtle = adw::PreferencesGroup::new();
            grp_turtle.set_title("Turtle API Reference");
            grp_turtle.set_description(Some("Methods available on the turtle instance"));

            grp_turtle.add(&Self::create_cmd_row(
                "turtle.forward(dist), backward(dist)",
                "Move turtle forward or backward along its current heading",
            ));
            grp_turtle.add(&Self::create_cmd_row(
                "turtle.left(angle), right(angle)",
                "Turn turtle counter-clockwise or clockwise by angle degrees",
            ));
            grp_turtle.add(&Self::create_cmd_row(
                "turtle.pen_down(), pen_up()",
                "Lower pen to draw trails, or raise pen to move without drawing",
            ));
            grp_turtle.add(&Self::create_cmd_row(
                "turtle.home(), clear(), step_back()",
                "Reset position to origin (0, 0), clear canvas, or undo last step",
            ));
            page.add(&grp_turtle);

            let grp_examples = adw::PreferencesGroup::new();
            grp_examples.set_title("PyCharm Code Examples");
            grp_examples.set_description(Some("Ready-to-run Python scripts with real-time PyCharm console output"));

            grp_examples.add(&Self::create_code_card(
                "PyCharm Robot Script",
                "Python",
                "from pypole import robot\n\n# Move and paint corridor with live output in PyCharm console\nrobot.reset()\nwhile robot.free_right():\n    robot.right()\n    robot.paint()\n\nprint('Corridor painted successfully!')",
                "<span foreground=\"#e66100\" weight=\"bold\">from</span> pole <span foreground=\"#e66100\" weight=\"bold\">import</span> robot\n\n<span foreground=\"#77767b\" style=\"italic\"># Move and paint corridor with live output in PyCharm console</span>\nrobot.reset()\n<span foreground=\"#e66100\" weight=\"bold\">while</span> robot.free_right():\n    robot.right()\n    robot.paint()\n\nprint(<span foreground=\"#26a269\">'Corridor painted successfully!'</span>)",
                "Copy code example",
            ));

            grp_examples.add(&Self::create_code_card(
                "PyCharm Turtle Script",
                "Python",
                "from pypole import turtle\n\n# Draw a polygon with live status updates\nturtle.home()\nturtle.clear()\nfor _ in range(6):\n    turtle.forward(80)\n    turtle.right(60)\n\nprint('Hexagon drawn!')",
                "<span foreground=\"#e66100\" weight=\"bold\">from</span> pole <span foreground=\"#e66100\" weight=\"bold\">import</span> turtle\n\n<span foreground=\"#77767b\" style=\"italic\"># Draw a polygon with live status updates</span>\nturtle.home()\nturtle.clear()\n<span foreground=\"#e66100\" weight=\"bold\">for</span> _ <span foreground=\"#e66100\" weight=\"bold\">in</span> range(<span foreground=\"#c061cb\">6</span>):\n    turtle.forward(<span foreground=\"#c061cb\">80</span>)\n    turtle.right(<span foreground=\"#c061cb\">60</span>)\n\nprint(<span foreground=\"#26a269\">'Hexagon drawn!'</span>)",
                "Copy code example",
            ));
            page.add(&grp_examples);
        } else {
            let grp_ide = adw::PreferencesGroup::new();
            grp_ide.set_title("Подключение к PyCharm и внешним IDE");
            grp_ide.set_description(Some("Управление средой Pole напрямую из ваших любимых редакторов кода на Python"));

            grp_ide.add(&Self::create_cmd_row(
                "Установка библиотеки PyPole",
                "Выполните <b>pip install -e ./python</b> в терминале или откройте проект Pole в PyCharm",
            ));
            grp_ide.add(&Self::create_cmd_row(
                "Автоматическое обнаружение сокета",
                "Клиент автоматически находит активный сокет среды Pole (neomir.sock / pole.sock) в системе",
            ));
            grp_ide.add(&Self::create_cmd_row(
                "Вывод результатов в консоль PyCharm",
                "Каждый шаг и сенсорные данные мгновенно форматируются и выводятся прямо в окно выполнения вашей IDE",
            ));
            grp_ide.add(&Self::create_cmd_row(
                "Встроенный и изолированный Python",
                "Pole использует изолированное окружение (PYTHONNOUSERSITE=1) со встроенной библиотекой pypole, не зависящее от установленных в ОС пакетов",
            ));
            page.add(&grp_ide);

            let grp_robot = adw::PreferencesGroup::new();
            grp_robot.set_title("Методы исполнителя Робот");
            grp_robot.set_description(Some("Доступны как на русском, так и на английском языке"));

            grp_robot.add(&Self::create_cmd_row(
                "робот.вправо(n), влево(n), вверх(n), вниз(n)",
                "Шаг в указанном направлении на n клеток (по умолчанию 1)",
            ));
            grp_robot.add(&Self::create_cmd_row(
                "робот.закрасить(), сброс(), шаг_назад()",
                "Закрасить клетку, вернуть в начало или отменить последний ход",
            ));
            grp_robot.add(&Self::create_cmd_row(
                "робот.сверху_стена(), снизу_стена(), слева_стена(), справа_стена()",
                "Датчики стен (возвращают True при наличии стены)",
            ));
            grp_robot.add(&Self::create_cmd_row(
                "робот.сверху_свободно(), снизу_свободно(), слева_свободно(), справа_свободно()",
                "Проверка свободного пути для циклов «пока» и условий «если»",
            ));
            grp_robot.add(&Self::create_cmd_row(
                "робот.клетка_закрашена(), клетка_чистая()",
                "Проверка статуса текущей клетки поля",
            ));
            page.add(&grp_robot);

            let grp_turtle = adw::PreferencesGroup::new();
            grp_turtle.set_title("Методы исполнителя Черепаха");
            grp_turtle.set_description(Some("Команды векторного рисования и навигации"));

            grp_turtle.add(&Self::create_cmd_row(
                "черепаха.вперёд(дист), назад(дист)",
                "Движение вдоль направления взгляда (forward / backward)",
            ));
            grp_turtle.add(&Self::create_cmd_row(
                "черепаха.влево(угол), вправо(угол)",
                "Поворот на угол в градусах (left / right)",
            ));
            grp_turtle.add(&Self::create_cmd_row(
                "черепаха.опустить_хвост(), поднять_хвост()",
                "Управление рисованием: с пером или без следа (pen_down / pen_up)",
            ));
            grp_turtle.add(&Self::create_cmd_row(
                "черепаха.домой(), очистить(), шаг_назад()",
                "Возврат в центр (0, 0), очистка холста или отмена шага",
            ));
            page.add(&grp_turtle);

            let grp_examples = adw::PreferencesGroup::new();
            grp_examples.set_title("Примеры программ для запуска из PyCharm");
            grp_examples.set_description(Some("Готовые скрипты с отображением выполнения в окне Pole и выводом в PyCharm"));

            grp_examples.add(&Self::create_code_card(
                "Управление Роботом из PyCharm",
                "Python",
                "from pypole import робот\n\n# Робот выполняет команды в Pole, а результат выводится в консоль PyCharm:\nробот.сброс()\nwhile робот.справа_свободно():\n    робот.вправо()\n    робот.закрасить()\n\nprint('Коридор успешно закрашен!')",
                "<span foreground=\"#e66100\" weight=\"bold\">from</span> pole <span foreground=\"#e66100\" weight=\"bold\">import</span> робот\n\n<span foreground=\"#77767b\" style=\"italic\"># Робот выполняет команды в Pole, а результат выводится в консоль PyCharm:</span>\nробот.сброс()\n<span foreground=\"#e66100\" weight=\"bold\">while</span> робот.справа_свободно():\n    робот.вправо()\n    робот.закрасить()\n\nprint(<span foreground=\"#26a269\">'Коридор успешно закрашен!'</span>)",
                "Копировать пример в буфер",
            ));

            grp_examples.add(&Self::create_code_card(
                "Рисование Черепахой из PyCharm",
                "Python",
                "from pypole import черепаха\n\n# Рисование шестиугольника с выводом координат в консоль:\nчерепаха.домой()\nчерепаха.очистить()\nfor _ in range(6):\n    черепаха.вперёд(80)\n    черепаха.вправо(60)\n\nprint('Шестиугольник готов!')",
                "<span foreground=\"#e66100\" weight=\"bold\">from</span> pole <span foreground=\"#e66100\" weight=\"bold\">import</span> черепаха\n\n<span foreground=\"#77767b\" style=\"italic\"># Рисование шестиугольника с выводом координат в консоль:</span>\nчерепаха.домой()\nчерепаха.очистить()\n<span foreground=\"#e66100\" weight=\"bold\">for</span> _ <span foreground=\"#e66100\" weight=\"bold\">in</span> range(<span foreground=\"#c061cb\">6</span>):\n    черепаха.вперёд(<span foreground=\"#c061cb\">80</span>)\n    черепаха.вправо(<span foreground=\"#c061cb\">60</span>)\n\nprint(<span foreground=\"#26a269\">'Шестиугольник готов!'</span>)",
                "Копировать пример в буфер",
            ));
            page.add(&grp_examples);
        }

        page
    }

    fn build_blocks_page(is_en: bool) -> adw::PreferencesPage {
        let page = adw::PreferencesPage::new();
        page.set_title(if is_en { "Visual Blocks" } else { "Визуальные блоки" });
        page.set_icon_name(Some("view-grid-symbolic"));
        page.set_name(Some("blocks"));

        if is_en {
            let grp_arch = adw::PreferencesGroup::new();
            grp_arch.set_title("Visual Block Programming");
            grp_arch.set_description(Some("Assemble algorithms visually without typing mistakes and syntax errors"));

            grp_arch.add(&Self::create_cmd_row(
                "Mode Switching",
                "Quickly toggle between 'Code' and 'Blocks' modes using the toolbar buttons",
            ));
            grp_arch.add(&Self::create_cmd_row(
                "Two-way Synchronization",
                "Any modifications made in visual blocks instantly update the KuMir source code",
            ));
            page.add(&grp_arch);

            let grp_palette = adw::PreferencesGroup::new();
            grp_palette.set_title("Block Palette");
            grp_palette.set_description(Some("Categories of functional blocks available in the palette"));

            grp_palette.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">Actions</span>",
                "Motion and painting commands: Right, Left, Up, Down, Paint",
            ));
            grp_palette.add(&Self::create_cmd_row(
                "<span foreground=\"#e66100\" weight=\"bold\">Control & Loops</span>",
                "Loop containers: Repeat N times, While free",
            ));
            grp_palette.add(&Self::create_cmd_row(
                "<span foreground=\"#9141ac\" weight=\"bold\">Conditions</span>",
                "Conditional branching block with wall sensor selector dropdown",
            ));
            grp_palette.add(&Self::create_cmd_row(
                "<span foreground=\"#26a269\" weight=\"bold\">Presets</span>",
                "Instant ready-to-run templates: First Steps, Square, Stairs",
            ));
            page.add(&grp_palette);

            let grp_ops = adw::PreferencesGroup::new();
            grp_ops.set_title("Editing & Reordering");
            grp_ops.set_description(Some("Managing block order and nested statements"));

            grp_ops.add(&Self::create_cmd_row(
                "Reorder Arrows",
                "Use the up and down arrows on each card to change execution sequence",
            ));
            grp_ops.add(&Self::create_cmd_row(
                "Trash Icon",
                "Delete a block from the program with a single click",
            ));
            page.add(&grp_ops);
        } else {
            let grp_arch = adw::PreferencesGroup::new();
            grp_arch.set_title("Принцип работы");
            grp_arch.set_description(Some("Составление алгоритмов из наглядных блоков без синтаксических ошибок"));

            grp_arch.add(&Self::create_cmd_row(
                "Переключение режимов",
                "Быстрое переключение между редактором «Код» и холстом «Блоки» на верхней панели",
            ));
            grp_arch.add(&Self::create_cmd_row(
                "Двусторонняя синхронизация",
                "Любые изменения в блоках мгновенно обновляют исходный текст программы на языке КуМир",
            ));
            page.add(&grp_arch);

            let grp_palette = adw::PreferencesGroup::new();
            grp_palette.set_title("Палитра блоков");
            grp_palette.set_description(Some("Категории доступных блоков в боковой палитре"));

            grp_palette.add(&Self::create_cmd_row(
                "<span foreground=\"#3584e4\" weight=\"bold\">Действия Робота</span>",
                "Команды перемещения и закрашивания: Вправо, Влево, Вверх, Вниз, Закрасить",
            ));
            grp_palette.add(&Self::create_cmd_row(
                "<span foreground=\"#e66100\" weight=\"bold\">Управление и циклы</span>",
                "Блоки циклов: Повторить N раз, Пока свободно",
            ));
            grp_palette.add(&Self::create_cmd_row(
                "<span foreground=\"#9141ac\" weight=\"bold\">Условия</span>",
                "Ветвление «Если» с удобным выпадающим списком датчиков стен",
            ));
            grp_palette.add(&Self::create_cmd_row(
                "<span foreground=\"#26a269\" weight=\"bold\">Готовые примеры</span>",
                "Шаблоны для быстрого старта: Первые шаги, Квадрат, Лесенка",
            ));
            page.add(&grp_palette);

            let grp_ops = adw::PreferencesGroup::new();
            grp_ops.set_title("Управление структурой");
            grp_ops.set_description(Some("Изменение порядка следования и редактирование"));

            grp_ops.add(&Self::create_cmd_row(
                "Стрелки перемещения",
                "Кнопки со стрелками «Вверх» и «Вниз» на карточке блока меняют порядок команд",
            ));
            grp_ops.add(&Self::create_cmd_row(
                "Кнопка корзины",
                "Удаляет выбранный блок из программы одним кликом",
            ));
            page.add(&grp_ops);
        }

        page
    }

    fn build_shortcuts_page(is_en: bool) -> adw::PreferencesPage {
        let page = adw::PreferencesPage::new();
        page.set_title(if is_en { "Shortcuts" } else { "Горячие клавиши" });
        page.set_icon_name(Some("preferences-desktop-keyboard-shortcuts-symbolic"));
        page.set_name(Some("shortcuts"));

        if is_en {
            let grp_exec = adw::PreferencesGroup::new();
            grp_exec.set_title("Execution Controls");
            grp_exec.set_description(Some("Shortcuts for running, pausing, and stepping through programs"));

            grp_exec.add(&Self::create_shortcut_row("F1", "Open Documentation & Reference window"));
            grp_exec.add(&Self::create_shortcut_row("F5 / Ctrl+R", "Run or Pause algorithm execution"));
            grp_exec.add(&Self::create_shortcut_row("F10", "Execute single step in debugger"));
            grp_exec.add(&Self::create_shortcut_row("F8", "Stop program and reset field / performer"));
            grp_exec.add(&Self::create_shortcut_row("Ctrl+Z", "Step back performer / Undo in editor"));
            grp_exec.add(&Self::create_shortcut_row("Ctrl+Y", "Redo last undone change in editor"));
            page.add(&grp_exec);

            let grp_file = adw::PreferencesGroup::new();
            grp_file.set_title("File Operations");
            grp_file.set_description(Some("Saving and opening algorithm source files"));

            grp_file.add(&Self::create_shortcut_row("Ctrl+O", "Open algorithm file (*.kum, *.py)"));
            grp_file.add(&Self::create_shortcut_row("Ctrl+S", "Save current algorithm to disk"));
            page.add(&grp_file);

            let grp_vim = adw::PreferencesGroup::new();
            grp_vim.set_title("Vim Modal Editing");
            grp_vim.set_description(Some("Keybindings available when Vim mode is enabled in Settings"));

            grp_vim.add(&Self::create_shortcut_row("h / j / k / l", "Move cursor Left / Down / Up / Right"));
            grp_vim.add(&Self::create_shortcut_row("i", "Enter Insert mode at cursor"));
            grp_vim.add(&Self::create_shortcut_row("Esc", "Return to Normal command mode"));
            grp_vim.add(&Self::create_shortcut_row("x", "Delete character under cursor"));
            grp_vim.add(&Self::create_shortcut_row("dd", "Delete entire line"));
            grp_vim.add(&Self::create_shortcut_row("u", "Undo last modification"));
            page.add(&grp_vim);
        } else {
            let grp_exec = adw::PreferencesGroup::new();
            grp_exec.set_title("Управление выполнением");
            grp_exec.set_description(Some("Горячие клавиши для запуска, пошаговой отладки и сброса"));

            grp_exec.add(&Self::create_shortcut_row("F1", "Открыть окно документации и справки"));
            grp_exec.add(&Self::create_shortcut_row("F5 / Ctrl+R", "Запустить или приостановить выполнение программы"));
            grp_exec.add(&Self::create_shortcut_row("F10", "Выполнить один шаг алгоритма"));
            grp_exec.add(&Self::create_shortcut_row("F8", "Остановить выполнение и сбросить поле / исполнителя"));
            grp_exec.add(&Self::create_shortcut_row("Ctrl+Z", "Шаг назад исполнителя / отмена действия в редакторе"));
            grp_exec.add(&Self::create_shortcut_row("Ctrl+Y", "Повторить отменённое действие в редакторе"));
            page.add(&grp_exec);

            let grp_file = adw::PreferencesGroup::new();
            grp_file.set_title("Работа с файлами");
            grp_file.set_description(Some("Сохранение и открытие файлов программ"));

            grp_file.add(&Self::create_shortcut_row("Ctrl+O", "Открыть файл алгоритма (*.kum, *.py)"));
            grp_file.add(&Self::create_shortcut_row("Ctrl+S", "Сохранить текущий алгоритм в файл"));
            page.add(&grp_file);

            let grp_vim = adw::PreferencesGroup::new();
            grp_vim.set_title("Модальный режим Vim");
            grp_vim.set_description(Some("Сочетания клавиш при включённом режиме Vim в «Настройках»"));

            grp_vim.add(&Self::create_shortcut_row("h / j / k / l", "Перемещение курсора: влево, вниз, вверх, вправо"));
            grp_vim.add(&Self::create_shortcut_row("i", "Вход в режим вставки текста (Insert)"));
            grp_vim.add(&Self::create_shortcut_row("Esc", "Возврат в командный режим (Normal)"));
            grp_vim.add(&Self::create_shortcut_row("x", "Удалить символ под курсором"));
            grp_vim.add(&Self::create_shortcut_row("dd", "Удалить текущую строку целиком"));
            grp_vim.add(&Self::create_shortcut_row("u", "Отменить последнее изменение"));
            page.add(&grp_vim);
        }

        page
    }
}
