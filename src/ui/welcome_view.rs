// Welcome and project creation view for Pole IDE.

use adw::prelude::*;
use crate::ui::i18n::Language;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectType {
    KumirRobot,
    PythonRobot,
    PythonTurtle,
}

impl ProjectType {
    pub fn all() -> &'static [ProjectType] {
        &[
            ProjectType::KumirRobot,
            ProjectType::PythonRobot,
            ProjectType::PythonTurtle,
        ]
    }

    pub fn title(&self, lang: Language) -> &'static str {
        match (lang, self) {
            (Language::En, ProjectType::KumirRobot) => "KuMir — Robot (Algorithmic Language)",
            (Language::Ru, ProjectType::KumirRobot) => "КуМир — Робот (Алгоритмический язык)",
            (Language::En, ProjectType::PythonRobot) => "Python — Robot (neoomir-pypole)",
            (Language::Ru, ProjectType::PythonRobot) => "Python — Робот (библиотека neoomir-pypole)",
            (Language::En, ProjectType::PythonTurtle) => "Python — Turtle (Vector Graphics)",
            (Language::Ru, ProjectType::PythonTurtle) => "Python — Черепаха (Векторная графика)",
        }
    }

    pub fn subtitle(&self, lang: Language) -> &'static str {
        match (lang, self) {
            (Language::En, ProjectType::KumirRobot) => "Text code, Scratch visual blocks, Robot field",
            (Language::Ru, ProjectType::KumirRobot) => "Текстовый код, визуальные блоки Scratch, поле Робота",
            (Language::En, ProjectType::PythonRobot) => "Standard Python 3 script controlling Robot via IPC",
            (Language::Ru, ProjectType::PythonRobot) => "Скрипт Python 3 для управления Роботом через IPC",
            (Language::En, ProjectType::PythonTurtle) => "Python vector graphics, geometry and canvas drawing",
            (Language::Ru, ProjectType::PythonTurtle) => "Векторная графика на Python, геометрия и рисование",
        }
    }

    pub fn default_project_name(&self, lang: Language) -> &'static str {
        match (lang, self) {
            (Language::En, ProjectType::KumirRobot) => "RobotProject",
            (Language::Ru, ProjectType::KumirRobot) => "Проект_Робот",
            (Language::En, ProjectType::PythonRobot) => "python_robot",
            (Language::Ru, ProjectType::PythonRobot) => "робот_питон",
            (Language::En, ProjectType::PythonTurtle) => "turtle_art",
            (Language::Ru, ProjectType::PythonTurtle) => "черепаха_графика",
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            ProjectType::KumirRobot => "kum",
            ProjectType::PythonRobot | ProjectType::PythonTurtle => "py",
        }
    }

    pub fn starter_code(&self) -> &'static str {
        match self {
            ProjectType::KumirRobot => {
                "использовать Робот\nалг\nнач\n    | Напишите команды для Робота здесь\n    вправо\n    закрасить\nкон\n"
            }
            ProjectType::PythonRobot => {
                "# Программа управления Роботом на Python\nfrom neoomir_pypole import robot\n\n# Сброс поля в начальное состояние\nrobot.reset()\n\n# Команды управления Роботом:\nwhile robot.free_right():\n    robot.right()\n    robot.paint()\n"
            }
            ProjectType::PythonTurtle => {
                "# Векторная графика с Черепахой на Python\nfrom neoomir_pypole import turtle\n\nturtle.home()\nturtle.clear()\nturtle.pen_down()\n\n# Рисуем фигуру (квадрат):\nfor _ in range(4):\n    turtle.forward(100)\n    turtle.left(90)\n"
            }
        }
    }
}

pub struct WelcomeView {
    pub container: gtk::Box,
    pub title_widget: adw::WindowTitle,
    pub name_entry: adw::EntryRow,
    pub lang_combo: adw::ComboRow,
    pub btn_create: gtk::Button,
    pub btn_open: gtk::Button,
    pub btn_back: gtk::Button,
    pub group_project: adw::PreferencesGroup,
    pub group_quick: adw::PreferencesGroup,
    pub row_quick_kumir: adw::ActionRow,
    pub row_quick_py_robot: adw::ActionRow,
    pub row_quick_py_turtle: adw::ActionRow,
    pub lbl_subtitle: gtk::Label,
    pub current_lang: Rc<RefCell<Language>>,
}

impl WelcomeView {
    pub fn new(lang: Language) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // Header bar
        let header = adw::HeaderBar::new();
        let title_widget = adw::WindowTitle::new("Pole", lang.tr("welcome_title"));
        header.set_title_widget(Some(&title_widget));

        let btn_back = gtk::Button::new();
        let back_content = adw::ButtonContent::new();
        back_content.set_icon_name("go-previous-symbolic");
        back_content.set_label(lang.tr("btn_back_to_workspace"));
        btn_back.set_child(Some(&back_content));
        btn_back.add_css_class("flat");
        btn_back.set_visible(false); // Only visible if active project already loaded
        header.pack_start(&btn_back);

        container.append(&header);

        // Content area inside ScrolledWindow and Clamp
        let scroll = gtk::ScrolledWindow::new();
        scroll.set_vexpand(true);
        scroll.set_hexpand(true);

        let clamp = adw::Clamp::new();
        clamp.set_maximum_size(560);
        clamp.set_tightening_threshold(420);

        let body_box = gtk::Box::new(gtk::Orientation::Vertical, 18);
        body_box.set_margin_top(28);
        body_box.set_margin_bottom(28);
        body_box.set_margin_start(16);
        body_box.set_margin_end(16);

        // Banner: Icon, Title, Subtitle
        let banner_box = gtk::Box::new(gtk::Orientation::Vertical, 8);
        banner_box.set_halign(gtk::Align::Center);

        let app_icon = gtk::Image::from_icon_name("io.github.NeooMir.Pole");
        app_icon.set_pixel_size(84);
        banner_box.append(&app_icon);

        let lbl_title = gtk::Label::new(Some("Pole"));
        lbl_title.add_css_class("title-1");
        banner_box.append(&lbl_title);

        let lbl_subtitle = gtk::Label::new(Some(lang.tr("welcome_subtitle")));
        lbl_subtitle.add_css_class("body");
        lbl_subtitle.add_css_class("dim-label");
        lbl_subtitle.set_wrap(true);
        lbl_subtitle.set_justify(gtk::Justification::Center);
        banner_box.append(&lbl_subtitle);

        body_box.append(&banner_box);

        // Project Creation Group
        let group_project = adw::PreferencesGroup::new();
        group_project.set_title(lang.tr("new_project"));
        group_project.set_description(Some(lang.tr("new_project_desc")));

        let name_entry = adw::EntryRow::new();
        name_entry.set_title(lang.tr("project_name"));
        name_entry.set_text(ProjectType::KumirRobot.default_project_name(lang));
        group_project.add(&name_entry);

        let lang_combo = adw::ComboRow::new();
        lang_combo.set_title(lang.tr("programming_lang"));
        lang_combo.set_subtitle(ProjectType::KumirRobot.subtitle(lang));

        let titles: Vec<&str> = ProjectType::all().iter().map(|p| p.title(lang)).collect();
        let string_list = gtk::StringList::new(&titles);
        lang_combo.set_model(Some(&string_list));
        lang_combo.set_selected(0);
        group_project.add(&lang_combo);

        body_box.append(&group_project);

        // Primary & Secondary Action Buttons
        let actions_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        actions_box.set_halign(gtk::Align::Center);
        actions_box.set_margin_top(4);

        let btn_create = gtk::Button::new();
        let create_content = adw::ButtonContent::new();
        create_content.set_icon_name("document-new-symbolic");
        create_content.set_label(lang.tr("btn_create_project"));
        btn_create.set_child(Some(&create_content));
        btn_create.add_css_class("suggested-action");
        btn_create.add_css_class("pill");
        actions_box.append(&btn_create);

        let btn_open = gtk::Button::new();
        let open_content = adw::ButtonContent::new();
        open_content.set_icon_name("document-open-symbolic");
        open_content.set_label(lang.tr("btn_open_file"));
        btn_open.set_child(Some(&open_content));
        btn_open.add_css_class("flat");
        actions_box.append(&btn_open);

        body_box.append(&actions_box);

        // Quick Starter Templates Group
        let group_quick = adw::PreferencesGroup::new();
        group_quick.set_title(lang.tr("quick_start"));
        group_quick.set_margin_top(12);

        let row_quick_kumir = adw::ActionRow::new();
        row_quick_kumir.set_title(ProjectType::KumirRobot.title(lang));
        row_quick_kumir.set_subtitle(ProjectType::KumirRobot.subtitle(lang));
        row_quick_kumir.set_activatable(true);
        let icon_kum = gtk::Image::from_icon_name("media-playback-start-symbolic");
        row_quick_kumir.add_prefix(&icon_kum);
        let arrow_kum = gtk::Image::from_icon_name("go-next-symbolic");
        row_quick_kumir.add_suffix(&arrow_kum);
        group_quick.add(&row_quick_kumir);

        let row_quick_py_robot = adw::ActionRow::new();
        row_quick_py_robot.set_title(ProjectType::PythonRobot.title(lang));
        row_quick_py_robot.set_subtitle(ProjectType::PythonRobot.subtitle(lang));
        row_quick_py_robot.set_activatable(true);
        let icon_py_r = gtk::Image::from_icon_name("utilities-terminal-symbolic");
        row_quick_py_robot.add_prefix(&icon_py_r);
        let arrow_py_r = gtk::Image::from_icon_name("go-next-symbolic");
        row_quick_py_robot.add_suffix(&arrow_py_r);
        group_quick.add(&row_quick_py_robot);

        let row_quick_py_turtle = adw::ActionRow::new();
        row_quick_py_turtle.set_title(ProjectType::PythonTurtle.title(lang));
        row_quick_py_turtle.set_subtitle(ProjectType::PythonTurtle.subtitle(lang));
        row_quick_py_turtle.set_activatable(true);
        let icon_py_t = gtk::Image::from_icon_name("draw-spiral-symbolic");
        row_quick_py_turtle.add_prefix(&icon_py_t);
        let arrow_py_t = gtk::Image::from_icon_name("go-next-symbolic");
        row_quick_py_turtle.add_suffix(&arrow_py_t);
        group_quick.add(&row_quick_py_turtle);

        body_box.append(&group_quick);

        clamp.set_child(Some(&body_box));
        scroll.set_child(Some(&clamp));
        container.append(&scroll);

        let lang_ref = Rc::new(RefCell::new(lang));

        // Update subtitle and suggested name when combo selection changes
        {
            let lr = lang_ref.clone();
            let n_entry = name_entry.clone();
            lang_combo.connect_selected_notify(move |combo| {
                let l = *lr.borrow();
                let idx = combo.selected() as usize;
                if let Some(pt) = ProjectType::all().get(idx) {
                    combo.set_subtitle(pt.subtitle(l));
                    // Update entry text if user hasn't typed a custom name yet
                    let cur_text = n_entry.text().to_string();
                    let matches_default = ProjectType::all()
                        .iter()
                        .any(|p| cur_text == p.default_project_name(l) || cur_text.is_empty());
                    if matches_default {
                        n_entry.set_text(pt.default_project_name(l));
                    }
                }
            });
        }

        Self {
            container,
            title_widget,
            name_entry,
            lang_combo,
            btn_create,
            btn_open,
            btn_back,
            group_project,
            group_quick,
            row_quick_kumir,
            row_quick_py_robot,
            row_quick_py_turtle,
            lbl_subtitle,
            current_lang: lang_ref,
        }
    }

    pub fn set_has_active_workspace(&self, has_active: bool) {
        self.btn_back.set_visible(has_active);
    }

    pub fn selected_project_type(&self) -> ProjectType {
        let idx = self.lang_combo.selected() as usize;
        *ProjectType::all().get(idx).unwrap_or(&ProjectType::KumirRobot)
    }

    pub fn project_name(&self) -> String {
        let text = self.name_entry.text().to_string().trim().to_string();
        if text.is_empty() {
            let lang = *self.current_lang.borrow();
            self.selected_project_type().default_project_name(lang).to_string()
        } else {
            text
        }
    }

    pub fn update_language(&self, lang: Language) {
        *self.current_lang.borrow_mut() = lang;
        self.title_widget.set_subtitle(lang.tr("welcome_title"));
        self.lbl_subtitle.set_text(lang.tr("welcome_subtitle"));
        self.group_project.set_title(lang.tr("new_project"));
        self.group_project.set_description(Some(lang.tr("new_project_desc")));
        self.name_entry.set_title(lang.tr("project_name"));
        self.lang_combo.set_title(lang.tr("programming_lang"));
        self.group_quick.set_title(lang.tr("quick_start"));

        let titles: Vec<&str> = ProjectType::all().iter().map(|p| p.title(lang)).collect();
        let string_list = gtk::StringList::new(&titles);
        let selected = self.lang_combo.selected();
        self.lang_combo.set_model(Some(&string_list));
        self.lang_combo.set_selected(selected);

        if let Some(pt) = ProjectType::all().get(selected as usize) {
            self.lang_combo.set_subtitle(pt.subtitle(lang));
        }

        let back_content = adw::ButtonContent::new();
        back_content.set_icon_name("go-previous-symbolic");
        back_content.set_label(lang.tr("btn_back_to_workspace"));
        self.btn_back.set_child(Some(&back_content));

        let create_content = adw::ButtonContent::new();
        create_content.set_icon_name("document-new-symbolic");
        create_content.set_label(lang.tr("btn_create_project"));
        self.btn_create.set_child(Some(&create_content));

        let open_content = adw::ButtonContent::new();
        open_content.set_icon_name("document-open-symbolic");
        open_content.set_label(lang.tr("btn_open_file"));
        self.btn_open.set_child(Some(&open_content));

        self.row_quick_kumir.set_title(ProjectType::KumirRobot.title(lang));
        self.row_quick_kumir.set_subtitle(ProjectType::KumirRobot.subtitle(lang));

        self.row_quick_py_robot.set_title(ProjectType::PythonRobot.title(lang));
        self.row_quick_py_robot.set_subtitle(ProjectType::PythonRobot.subtitle(lang));

        self.row_quick_py_turtle.set_title(ProjectType::PythonTurtle.title(lang));
        self.row_quick_py_turtle.set_subtitle(ProjectType::PythonTurtle.subtitle(lang));
    }
}
