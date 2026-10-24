use crate::lang::ast::{Expr, RobotAction, Stmt};
use crate::lang::parser::Parser;
use crate::lang::token::Lexer;
use crate::robot::Direction;
use crate::ui::i18n::Language;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionKind {
    RightFree,
    LeftFree,
    UpFree,
    DownFree,
    RightWall,
    LeftWall,
    UpWall,
    DownWall,
    CellPainted,
    CellClean,
}

impl ConditionKind {
    pub const ALL: [ConditionKind; 10] = [
        ConditionKind::RightFree,
        ConditionKind::LeftFree,
        ConditionKind::UpFree,
        ConditionKind::DownFree,
        ConditionKind::RightWall,
        ConditionKind::LeftWall,
        ConditionKind::UpWall,
        ConditionKind::DownWall,
        ConditionKind::CellPainted,
        ConditionKind::CellClean,
    ];

    pub fn to_kumir(&self) -> &'static str {
        match self {
            ConditionKind::RightFree => "справа свободно",
            ConditionKind::LeftFree => "слева свободно",
            ConditionKind::UpFree => "сверху свободно",
            ConditionKind::DownFree => "снизу свободно",
            ConditionKind::RightWall => "справа стена",
            ConditionKind::LeftWall => "слева стена",
            ConditionKind::UpWall => "сверху стена",
            ConditionKind::DownWall => "снизу стена",
            ConditionKind::CellPainted => "клетка закрашена",
            ConditionKind::CellClean => "клетка чистая",
        }
    }

    pub fn display_name(&self, is_en: bool) -> &'static str {
        if is_en {
            match self {
                ConditionKind::RightFree => "Right is free",
                ConditionKind::LeftFree => "Left is free",
                ConditionKind::UpFree => "Up is free",
                ConditionKind::DownFree => "Down is free",
                ConditionKind::RightWall => "Wall on right",
                ConditionKind::LeftWall => "Wall on left",
                ConditionKind::UpWall => "Wall above",
                ConditionKind::DownWall => "Wall below",
                ConditionKind::CellPainted => "Cell is painted",
                ConditionKind::CellClean => "Cell is clean",
            }
        } else {
            match self {
                ConditionKind::RightFree => "справа свободно",
                ConditionKind::LeftFree => "слева свободно",
                ConditionKind::UpFree => "сверху свободно",
                ConditionKind::DownFree => "снизу свободно",
                ConditionKind::RightWall => "справа стена",
                ConditionKind::LeftWall => "слева стена",
                ConditionKind::UpWall => "сверху стена",
                ConditionKind::DownWall => "снизу стена",
                ConditionKind::CellPainted => "клетка закрашена",
                ConditionKind::CellClean => "клетка чистая",
            }
        }
    }

    pub fn from_index(index: usize) -> Self {
        Self::ALL.get(index).copied().unwrap_or(ConditionKind::RightFree)
    }

    pub fn to_index(&self) -> usize {
        Self::ALL.iter().position(|c| c == self).unwrap_or(0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockKind {
    MoveRight,
    MoveLeft,
    MoveUp,
    MoveDown,
    Paint,
    RepeatTimes { times: usize, body: Vec<Block> },
    RepeatWhile { cond: ConditionKind, body: Vec<Block> },
    If { cond: ConditionKind, then_body: Vec<Block> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub id: u64,
    pub kind: BlockKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetKind {
    FirstSteps,
    Square,
    Stairs,
    Snake,
}

#[derive(Clone)]
pub struct BlocksView {
    pub container: gtk::Box,
    pub blocks: Rc<RefCell<Vec<Block>>>,
    pub next_id: Rc<RefCell<u64>>,
    workspace_cards_box: gtk::Box,
    on_change_cb: Rc<RefCell<Option<Rc<dyn Fn(&str)>>>>,
    pub language: Rc<RefCell<Language>>,
    css_provider: gtk::CssProvider,
    pub btn_run_ref: Rc<RefCell<Option<gtk::Button>>>,
    pub btn_step_ref: Rc<RefCell<Option<gtk::Button>>>,
}

impl BlocksView {
    pub fn new(lang: Language) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        container.set_vexpand(true);
        container.set_hexpand(true);

        let blocks = Rc::new(RefCell::new(Vec::new()));
        let next_id = Rc::new(RefCell::new(100));
        let workspace_cards_box = gtk::Box::new(gtk::Orientation::Vertical, 4);
        let on_change_cb = Rc::new(RefCell::new(None));
        let language = Rc::new(RefCell::new(lang));
        let css_provider = gtk::CssProvider::new();
        let btn_run_ref = Rc::new(RefCell::new(None));
        let btn_step_ref = Rc::new(RefCell::new(None));

        if let Some(display) = gtk::gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &css_provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        let view = Self {
            container,
            blocks,
            next_id,
            workspace_cards_box,
            on_change_cb,
            language,
            css_provider,
            btn_run_ref,
            btn_step_ref,
        };

        view.apply_css();
        view.build_layout();
        view.load_preset(PresetKind::FirstSteps);

        view
    }

    pub fn set_exec_buttons(&self, btn_run: gtk::Button, btn_step: gtk::Button) {
        *self.btn_run_ref.borrow_mut() = Some(btn_run);
        *self.btn_step_ref.borrow_mut() = Some(btn_step);
    }

    pub fn set_on_change<F: Fn(&str) + 'static>(&self, f: F) {
        *self.on_change_cb.borrow_mut() = Some(Rc::new(f));
    }

    pub fn set_language(&self, lang: Language) {
        *self.language.borrow_mut() = lang;
        self.build_layout();
        self.rebuild_workspace();
    }

    pub fn notify_change(&self) {
        let code = self.to_kumir();
        if let Some(cb) = self.on_change_cb.borrow().as_ref() {
            cb(&code);
        }
    }

    pub fn to_kumir(&self) -> String {
        generate_kumir(&self.blocks.borrow())
    }

    pub fn sync_from_kumir(&self, code: &str) -> Result<(), String> {
        let mut nid = *self.next_id.borrow();
        let parsed = parse_kumir_to_blocks(code, &mut nid)?;
        *self.next_id.borrow_mut() = nid;
        *self.blocks.borrow_mut() = parsed;
        self.rebuild_workspace();
        Ok(())
    }

    fn apply_css(&self) {
        let css = r#"
        .scratch-palette-sidebar {
            padding: 8px 10px;
            background-color: alpha(@window_bg_color, 0.4);
            border-right: 1px solid alpha(@borders, 0.35);
            min-width: 215px;
        }
        .scratch-cat-header {
            font-weight: 800;
            font-size: 11px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            color: @dim_label_opacity;
            margin-top: 10px;
            margin-bottom: 4px;
        }
        .scratch-btn-palette {
            border-radius: 8px;
            font-weight: 700;
            font-size: 13px;
            padding: 6px 10px;
            margin: 2px 0;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12);
        }
        .scratch-palette-motion {
            background: linear-gradient(135deg, #3b82f6, #2563eb);
            color: white;
            border: 1px solid #1d4ed8;
        }
        .scratch-palette-motion:hover {
            background: #2563eb;
        }
        .scratch-palette-paint {
            background: linear-gradient(135deg, #10b981, #059669);
            color: white;
            border: 1px solid #047857;
        }
        .scratch-palette-paint:hover {
            background: #059669;
        }
        .scratch-palette-control {
            background: linear-gradient(135deg, #f59e0b, #d97706);
            color: #261a00;
            border: 1px solid #b45309;
        }
        .scratch-palette-control:hover {
            background: #d97706;
        }
        .scratch-palette-cond {
            background: linear-gradient(135deg, #eab308, #ca8a04);
            color: #261a00;
            border: 1px solid #a16207;
        }
        .scratch-palette-cond:hover {
            background: #ca8a04;
        }
        .scratch-palette-preset {
            border-radius: 7px;
            font-weight: 600;
            font-size: 12px;
            padding: 5px 8px;
            margin: 2px 0;
        }

        .scratch-workspace-scroll {
            padding: 12px 16px;
            background-color: alpha(@window_bg_color, 0.2);
        }
        .scratch-start-flag {
            background: linear-gradient(135deg, #22c55e, #16a34a);
            color: white;
            font-weight: 800;
            font-size: 13px;
            border-radius: 12px 12px 6px 6px;
            padding: 9px 14px;
            border: 1px solid #15803d;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.12);
        }
        .scratch-end-flag {
            background: #64748b;
            color: white;
            font-weight: 700;
            font-size: 12px;
            border-radius: 6px 6px 12px 12px;
            padding: 7px 14px;
            margin-top: 6px;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
        }

        .scratch-card-header {
            border-radius: 10px;
            padding: 6px 10px;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.13);
        }
        .scratch-card-motion {
            background: linear-gradient(135deg, #3b82f6, #2563eb);
            color: white;
            border: 1px solid #1d4ed8;
        }
        .scratch-card-paint {
            background: linear-gradient(135deg, #10b981, #059669);
            color: white;
            border: 1px solid #047857;
        }
        .scratch-card-control {
            background: linear-gradient(135deg, #f59e0b, #d97706);
            color: #261a00;
            border: 1px solid #b45309;
        }
        .scratch-card-cond {
            background: linear-gradient(135deg, #eab308, #ca8a04);
            color: #261a00;
            border: 1px solid #a16207;
        }

        .scratch-loop-body-box {
            border-left: 5px solid #f59e0b;
            margin-left: 14px;
            padding-left: 8px;
            padding-top: 4px;
            padding-bottom: 4px;
            border-radius: 0 0 0 8px;
            background-color: alpha(#f59e0b, 0.08);
        }
        .scratch-cond-body-box {
            border-left: 5px solid #eab308;
            margin-left: 14px;
            padding-left: 8px;
            padding-top: 4px;
            padding-bottom: 4px;
            border-radius: 0 0 0 8px;
            background-color: alpha(#eab308, 0.08);
        }
        .scratch-loop-footer {
            font-size: 11px;
            font-style: italic;
            opacity: 0.85;
            margin-top: 2px;
            margin-left: 6px;
        }
        .scratch-ctrl-btn {
            padding: 1px 6px;
            min-height: 22px;
            min-width: 22px;
            font-size: 11px;
            border-radius: 6px;
        }
        "#;
        let _ = self.css_provider.load_from_data(css);
    }

    fn build_layout(&self) {
        // Clear previous content of container
        while let Some(child) = self.container.first_child() {
            self.container.remove(&child);
        }

        let lang = *self.language.borrow();

        // -------------------------------------------------------------
        // Left: Palette Sidebar
        // -------------------------------------------------------------
        let palette_box = gtk::Box::new(gtk::Orientation::Vertical, 4);
        palette_box.add_css_class("scratch-palette-sidebar");

        let title_palette = gtk::Label::new(Some(lang.tr("scratch_palette")));
        title_palette.add_css_class("heading");
        title_palette.set_halign(gtk::Align::Start);
        palette_box.append(&title_palette);

        // Category 1: Robot Actions
        let cat_actions = gtk::Label::new(Some(lang.tr("scratch_motion")));
        cat_actions.add_css_class("scratch-cat-header");
        cat_actions.set_halign(gtk::Align::Start);
        palette_box.append(&cat_actions);

        let add_btn = |box_ref: &gtk::Box, label: &str, cls: &str, kind: BlockKind, this: BlocksView| {
            let btn = gtk::Button::with_label(label);
            btn.add_css_class("scratch-btn-palette");
            btn.add_css_class(cls);
            btn.connect_clicked(move |_| {
                this.add_block(kind.clone());
            });
            box_ref.append(&btn);
        };

        add_btn(&palette_box, lang.tr("scratch_move_right"), "scratch-palette-motion", BlockKind::MoveRight, self.clone());
        add_btn(&palette_box, lang.tr("scratch_move_left"), "scratch-palette-motion", BlockKind::MoveLeft, self.clone());
        add_btn(&palette_box, lang.tr("scratch_move_up"), "scratch-palette-motion", BlockKind::MoveUp, self.clone());
        add_btn(&palette_box, lang.tr("scratch_move_down"), "scratch-palette-motion", BlockKind::MoveDown, self.clone());
        add_btn(&palette_box, lang.tr("scratch_paint"), "scratch-palette-paint", BlockKind::Paint, self.clone());

        // Category 2: Control & Loops
        let cat_control = gtk::Label::new(Some(lang.tr("scratch_control")));
        cat_control.add_css_class("scratch-cat-header");
        cat_control.set_halign(gtk::Align::Start);
        palette_box.append(&cat_control);

        add_btn(
            &palette_box,
            lang.tr("scratch_repeat"),
            "scratch-palette-control",
            BlockKind::RepeatTimes {
                times: 4,
                body: vec![Block { id: 0, kind: BlockKind::MoveRight }, Block { id: 0, kind: BlockKind::Paint }],
            },
            self.clone(),
        );

        add_btn(
            &palette_box,
            lang.tr("scratch_while"),
            "scratch-palette-control",
            BlockKind::RepeatWhile {
                cond: ConditionKind::RightFree,
                body: vec![Block { id: 0, kind: BlockKind::MoveRight }, Block { id: 0, kind: BlockKind::Paint }],
            },
            self.clone(),
        );

        // Category 3: Conditions
        let cat_cond = gtk::Label::new(Some(lang.tr("scratch_conditions")));
        cat_cond.add_css_class("scratch-cat-header");
        cat_cond.set_halign(gtk::Align::Start);
        palette_box.append(&cat_cond);

        add_btn(
            &palette_box,
            lang.tr("scratch_if"),
            "scratch-palette-cond",
            BlockKind::If {
                cond: ConditionKind::RightFree,
                then_body: vec![Block { id: 0, kind: BlockKind::MoveRight }],
            },
            self.clone(),
        );

        // Category 4: Examples for Kids
        let cat_presets = gtk::Label::new(Some(lang.tr("scratch_presets")));
        cat_presets.add_css_class("scratch-cat-header");
        cat_presets.set_halign(gtk::Align::Start);
        palette_box.append(&cat_presets);

        let add_preset_btn = |box_ref: &gtk::Box, label: &str, preset: PresetKind, this: BlocksView| {
            let btn = gtk::Button::with_label(label);
            btn.add_css_class("scratch-palette-preset");
            btn.add_css_class("flat");
            btn.connect_clicked(move |_| {
                this.load_preset(preset);
            });
            box_ref.append(&btn);
        };

        add_preset_btn(&palette_box, lang.tr("scratch_preset_steps"), PresetKind::FirstSteps, self.clone());
        add_preset_btn(&palette_box, lang.tr("scratch_preset_square"), PresetKind::Square, self.clone());
        add_preset_btn(&palette_box, lang.tr("scratch_preset_stairs"), PresetKind::Stairs, self.clone());
        add_preset_btn(&palette_box, lang.tr("scratch_preset_snake"), PresetKind::Snake, self.clone());

        // Category 5: Tools
        let cat_tools = gtk::Label::new(Some(lang.tr("scratch_tools")));
        cat_tools.add_css_class("scratch-cat-header");
        cat_tools.set_halign(gtk::Align::Start);
        palette_box.append(&cat_tools);

        let btn_clear = gtk::Button::with_label(lang.tr("scratch_clear"));
        btn_clear.add_css_class("destructive-action");
        btn_clear.add_css_class("scratch-palette-preset");
        {
            let this = self.clone();
            btn_clear.connect_clicked(move |_| {
                this.clear();
            });
        }
        palette_box.append(&btn_clear);

        let palette_scroll = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .child(&palette_box)
            .build();
        palette_scroll.set_size_request(215, -1);
        self.container.append(&palette_scroll);

        // -------------------------------------------------------------
        // Right: Workspace Area
        // -------------------------------------------------------------
        let workspace_pane = gtk::Box::new(gtk::Orientation::Vertical, 0);
        workspace_pane.set_hexpand(true);
        workspace_pane.set_vexpand(true);

        // Top Toolbar inside Workspace
        let ws_top_bar = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        ws_top_bar.set_margin_start(12);
        ws_top_bar.set_margin_end(12);
        ws_top_bar.set_margin_top(6);
        ws_top_bar.set_margin_bottom(6);

        let ws_title = gtk::Label::new(Some(lang.tr("scratch_title")));
        ws_title.add_css_class("heading");
        ws_top_bar.append(&ws_title);

        let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        ws_top_bar.append(&spacer);

        // Quick Run Button in Blocks Toolbar
        let btn_ws_run = gtk::Button::with_label(lang.tr("scratch_run"));
        btn_ws_run.add_css_class("suggested-action");
        btn_ws_run.set_icon_name("media-playback-start-symbolic");
        {
            let run_ref = self.btn_run_ref.clone();
            btn_ws_run.connect_clicked(move |_| {
                if let Some(b) = run_ref.borrow().as_ref() {
                    b.emit_clicked();
                }
            });
        }
        ws_top_bar.append(&btn_ws_run);

        // Quick Step Button in Blocks Toolbar
        let btn_ws_step = gtk::Button::with_label(lang.tr("scratch_step"));
        btn_ws_step.set_icon_name("media-seek-forward-symbolic");
        {
            let step_ref = self.btn_step_ref.clone();
            btn_ws_step.connect_clicked(move |_| {
                if let Some(b) = step_ref.borrow().as_ref() {
                    b.emit_clicked();
                }
            });
        }
        ws_top_bar.append(&btn_ws_step);

        workspace_pane.append(&ws_top_bar);
        workspace_pane.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

        // Scrollable cards list
        let ws_scroll = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Automatic)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .child(&self.workspace_cards_box)
            .build();
        ws_scroll.set_hexpand(true);
        ws_scroll.set_vexpand(true);
        ws_scroll.add_css_class("scratch-workspace-scroll");

        workspace_pane.append(&ws_scroll);
        self.container.append(&workspace_pane);
    }

    pub fn rebuild_workspace(&self) {
        while let Some(child) = self.workspace_cards_box.first_child() {
            self.workspace_cards_box.remove(&child);
        }

        let lang = *self.language.borrow();

        // 1. Start Flag Card
        let flag_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        flag_box.add_css_class("scratch-start-flag");
        let lbl_flag = gtk::Label::new(Some(lang.tr("scratch_flag")));
        flag_box.append(&lbl_flag);
        self.workspace_cards_box.append(&flag_box);

        // 2. Render all blocks
        let blocks = self.blocks.borrow().clone();
        for b in &blocks {
            let card = self.create_block_card(b);
            self.workspace_cards_box.append(&card);
        }

        // 3. Add block at end button
        let btn_add_end = gtk::MenuButton::new();
        btn_add_end.set_label(lang.tr("scratch_add_block"));
        btn_add_end.add_css_class("flat");
        btn_add_end.set_margin_top(8);
        btn_add_end.set_halign(gtk::Align::Start);

        let pop_menu = gtk::Popover::new();
        let pop_box = gtk::Box::new(gtk::Orientation::Vertical, 3);
        pop_box.set_margin_start(6);
        pop_box.set_margin_end(6);
        pop_box.set_margin_top(6);
        pop_box.set_margin_bottom(6);

        let make_pop_btn = |lbl: &str, kind: BlockKind, this: BlocksView, pop: gtk::Popover| {
            let b = gtk::Button::with_label(lbl);
            b.add_css_class("flat");
            b.connect_clicked(move |_| {
                pop.popdown();
                this.add_block(kind.clone());
            });
            b
        };

        pop_box.append(&make_pop_btn(lang.tr("scratch_move_right"), BlockKind::MoveRight, self.clone(), pop_menu.clone()));
        pop_box.append(&make_pop_btn(lang.tr("scratch_move_left"), BlockKind::MoveLeft, self.clone(), pop_menu.clone()));
        pop_box.append(&make_pop_btn(lang.tr("scratch_move_up"), BlockKind::MoveUp, self.clone(), pop_menu.clone()));
        pop_box.append(&make_pop_btn(lang.tr("scratch_move_down"), BlockKind::MoveDown, self.clone(), pop_menu.clone()));
        pop_box.append(&make_pop_btn(lang.tr("scratch_paint"), BlockKind::Paint, self.clone(), pop_menu.clone()));
        pop_box.append(&make_pop_btn(
            lang.tr("scratch_repeat"),
            BlockKind::RepeatTimes { times: 4, body: vec![Block { id: 0, kind: BlockKind::MoveRight }] },
            self.clone(),
            pop_menu.clone(),
        ));
        pop_box.append(&make_pop_btn(
            lang.tr("scratch_while"),
            BlockKind::RepeatWhile { cond: ConditionKind::RightFree, body: vec![Block { id: 0, kind: BlockKind::MoveRight }] },
            self.clone(),
            pop_menu.clone(),
        ));
        pop_box.append(&make_pop_btn(
            lang.tr("scratch_if"),
            BlockKind::If { cond: ConditionKind::RightFree, then_body: vec![Block { id: 0, kind: BlockKind::MoveRight }] },
            self.clone(),
            pop_menu.clone(),
        ));

        pop_menu.set_child(Some(&pop_box));
        btn_add_end.set_popover(Some(&pop_menu));
        self.workspace_cards_box.append(&btn_add_end);

        // 4. End Flag Card
        let end_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        end_box.add_css_class("scratch-end-flag");
        let lbl_end = gtk::Label::new(Some(lang.tr("scratch_end")));
        end_box.append(&lbl_end);
        self.workspace_cards_box.append(&end_box);
    }

    fn create_block_card(&self, block: &Block) -> gtk::Box {
        let root = gtk::Box::new(gtk::Orientation::Vertical, 2);
        let header = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        header.add_css_class("scratch-card-header");

        let lang = *self.language.borrow();
        let is_en = lang.is_en();
        let block_id = block.id;

        match &block.kind {
            BlockKind::MoveRight => {
                header.add_css_class("scratch-card-motion");
                let lbl = gtk::Label::new(Some(lang.tr("scratch_move_right")));
                lbl.set_halign(gtk::Align::Start);
                header.append(&lbl);
            }
            BlockKind::MoveLeft => {
                header.add_css_class("scratch-card-motion");
                let lbl = gtk::Label::new(Some(lang.tr("scratch_move_left")));
                lbl.set_halign(gtk::Align::Start);
                header.append(&lbl);
            }
            BlockKind::MoveUp => {
                header.add_css_class("scratch-card-motion");
                let lbl = gtk::Label::new(Some(lang.tr("scratch_move_up")));
                lbl.set_halign(gtk::Align::Start);
                header.append(&lbl);
            }
            BlockKind::MoveDown => {
                header.add_css_class("scratch-card-motion");
                let lbl = gtk::Label::new(Some(lang.tr("scratch_move_down")));
                lbl.set_halign(gtk::Align::Start);
                header.append(&lbl);
            }
            BlockKind::Paint => {
                header.add_css_class("scratch-card-paint");
                let lbl = gtk::Label::new(Some(lang.tr("scratch_paint")));
                lbl.set_halign(gtk::Align::Start);
                header.append(&lbl);
            }
            BlockKind::RepeatTimes { times, body } => {
                header.add_css_class("scratch-card-control");
                let lbl_pre = gtk::Label::new(Some(if is_en { "Repeat" } else { "Повторить" }));
                header.append(&lbl_pre);

                let spin = gtk::SpinButton::with_range(1.0, 99.0, 1.0);
                spin.set_value(*times as f64);
                spin.add_css_class("scratch-ctrl-btn");
                {
                    let this = self.clone();
                    spin.connect_value_changed(move |s| {
                        let val = s.value() as usize;
                        this.update_times(block_id, val);
                    });
                }
                header.append(&spin);

                let lbl_post = gtk::Label::new(Some(if is_en { "times" } else { "раз" }));
                header.append(&lbl_post);

                // Body Container
                let body_box = gtk::Box::new(gtk::Orientation::Vertical, 3);
                body_box.add_css_class("scratch-loop-body-box");

                for child_b in body {
                    body_box.append(&self.create_block_card(child_b));
                }

                // Add Inside Loop Button
                let btn_add_inner = gtk::MenuButton::new();
                btn_add_inner.set_label(lang.tr("scratch_add_inside"));
                btn_add_inner.add_css_class("flat");
                btn_add_inner.add_css_class("scratch-ctrl-btn");

                let pop_inner = gtk::Popover::new();
                let pop_inner_box = gtk::Box::new(gtk::Orientation::Vertical, 2);
                let pop_add_btn = |lbl: &str, kind: BlockKind, this: BlocksView, pop: gtk::Popover| {
                    let b = gtk::Button::with_label(lbl);
                    b.add_css_class("flat");
                    b.connect_clicked(move |_| {
                        pop.popdown();
                        this.add_block_to_body(block_id, kind.clone());
                    });
                    b
                };

                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_move_right"), BlockKind::MoveRight, self.clone(), pop_inner.clone()));
                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_move_left"), BlockKind::MoveLeft, self.clone(), pop_inner.clone()));
                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_move_up"), BlockKind::MoveUp, self.clone(), pop_inner.clone()));
                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_move_down"), BlockKind::MoveDown, self.clone(), pop_inner.clone()));
                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_paint"), BlockKind::Paint, self.clone(), pop_inner.clone()));
                pop_inner.set_child(Some(&pop_inner_box));
                btn_add_inner.set_popover(Some(&pop_inner));
                body_box.append(&btn_add_inner);

                root.append(&header);
                root.append(&body_box);

                let lbl_footer = gtk::Label::new(Some(if is_en { "end loop" } else { "конец цикла" }));
                lbl_footer.add_css_class("scratch-loop-footer");
                lbl_footer.set_halign(gtk::Align::Start);
                root.append(&lbl_footer);

                self.attach_control_buttons(&header, block_id);
                return root;
            }
            BlockKind::RepeatWhile { cond, body } => {
                header.add_css_class("scratch-card-control");
                let lbl_pre = gtk::Label::new(Some(if is_en { "While" } else { "Пока" }));
                header.append(&lbl_pre);

                let cond_names: Vec<&str> = ConditionKind::ALL.iter().map(|c| c.display_name(is_en)).collect();
                let dd = gtk::DropDown::from_strings(&cond_names);
                dd.set_selected(cond.to_index() as u32);
                dd.add_css_class("scratch-ctrl-btn");
                {
                    let this = self.clone();
                    dd.connect_selected_notify(move |dropdown| {
                        let idx = dropdown.selected() as usize;
                        let new_cond = ConditionKind::from_index(idx);
                        this.update_cond(block_id, new_cond);
                    });
                }
                header.append(&dd);

                // Body Container
                let body_box = gtk::Box::new(gtk::Orientation::Vertical, 3);
                body_box.add_css_class("scratch-loop-body-box");

                for child_b in body {
                    body_box.append(&self.create_block_card(child_b));
                }

                // Add Inside Loop Button
                let btn_add_inner = gtk::MenuButton::new();
                btn_add_inner.set_label(lang.tr("scratch_add_inside"));
                btn_add_inner.add_css_class("flat");
                btn_add_inner.add_css_class("scratch-ctrl-btn");

                let pop_inner = gtk::Popover::new();
                let pop_inner_box = gtk::Box::new(gtk::Orientation::Vertical, 2);
                let pop_add_btn = |lbl: &str, kind: BlockKind, this: BlocksView, pop: gtk::Popover| {
                    let b = gtk::Button::with_label(lbl);
                    b.add_css_class("flat");
                    b.connect_clicked(move |_| {
                        pop.popdown();
                        this.add_block_to_body(block_id, kind.clone());
                    });
                    b
                };

                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_move_right"), BlockKind::MoveRight, self.clone(), pop_inner.clone()));
                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_move_left"), BlockKind::MoveLeft, self.clone(), pop_inner.clone()));
                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_move_up"), BlockKind::MoveUp, self.clone(), pop_inner.clone()));
                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_move_down"), BlockKind::MoveDown, self.clone(), pop_inner.clone()));
                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_paint"), BlockKind::Paint, self.clone(), pop_inner.clone()));
                pop_inner.set_child(Some(&pop_inner_box));
                btn_add_inner.set_popover(Some(&pop_inner));
                body_box.append(&btn_add_inner);

                root.append(&header);
                root.append(&body_box);

                let lbl_footer = gtk::Label::new(Some(if is_en { "end loop" } else { "конец цикла" }));
                lbl_footer.add_css_class("scratch-loop-footer");
                lbl_footer.set_halign(gtk::Align::Start);
                root.append(&lbl_footer);

                self.attach_control_buttons(&header, block_id);
                return root;
            }
            BlockKind::If { cond, then_body } => {
                header.add_css_class("scratch-card-cond");
                let lbl_pre = gtk::Label::new(Some(if is_en { "If" } else { "Если" }));
                header.append(&lbl_pre);

                let cond_names: Vec<&str> = ConditionKind::ALL.iter().map(|c| c.display_name(is_en)).collect();
                let dd = gtk::DropDown::from_strings(&cond_names);
                dd.set_selected(cond.to_index() as u32);
                dd.add_css_class("scratch-ctrl-btn");
                {
                    let this = self.clone();
                    dd.connect_selected_notify(move |dropdown| {
                        let idx = dropdown.selected() as usize;
                        let new_cond = ConditionKind::from_index(idx);
                        this.update_cond(block_id, new_cond);
                    });
                }
                header.append(&dd);

                let lbl_then = gtk::Label::new(Some(if is_en { "then" } else { "то" }));
                header.append(&lbl_then);

                // Body Container
                let body_box = gtk::Box::new(gtk::Orientation::Vertical, 3);
                body_box.add_css_class("scratch-cond-body-box");

                for child_b in then_body {
                    body_box.append(&self.create_block_card(child_b));
                }

                // Add Inside Button
                let btn_add_inner = gtk::MenuButton::new();
                btn_add_inner.set_label(lang.tr("scratch_add_inside"));
                btn_add_inner.add_css_class("flat");
                btn_add_inner.add_css_class("scratch-ctrl-btn");

                let pop_inner = gtk::Popover::new();
                let pop_inner_box = gtk::Box::new(gtk::Orientation::Vertical, 2);
                let pop_add_btn = |lbl: &str, kind: BlockKind, this: BlocksView, pop: gtk::Popover| {
                    let b = gtk::Button::with_label(lbl);
                    b.add_css_class("flat");
                    b.connect_clicked(move |_| {
                        pop.popdown();
                        this.add_block_to_body(block_id, kind.clone());
                    });
                    b
                };

                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_move_right"), BlockKind::MoveRight, self.clone(), pop_inner.clone()));
                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_move_left"), BlockKind::MoveLeft, self.clone(), pop_inner.clone()));
                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_move_up"), BlockKind::MoveUp, self.clone(), pop_inner.clone()));
                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_move_down"), BlockKind::MoveDown, self.clone(), pop_inner.clone()));
                pop_inner_box.append(&pop_add_btn(lang.tr("scratch_paint"), BlockKind::Paint, self.clone(), pop_inner.clone()));
                pop_inner.set_child(Some(&pop_inner_box));
                btn_add_inner.set_popover(Some(&pop_inner));
                body_box.append(&btn_add_inner);

                root.append(&header);
                root.append(&body_box);

                let lbl_footer = gtk::Label::new(Some(if is_en { "end if" } else { "конец условия" }));
                lbl_footer.add_css_class("scratch-loop-footer");
                lbl_footer.set_halign(gtk::Align::Start);
                root.append(&lbl_footer);

                self.attach_control_buttons(&header, block_id);
                return root;
            }
        }

        self.attach_control_buttons(&header, block_id);
        root.append(&header);
        root
    }

    fn attach_control_buttons(&self, header: &gtk::Box, block_id: u64) {
        let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        header.append(&spacer);

        // Move Up
        let btn_up = gtk::Button::from_icon_name("go-up-symbolic");
        btn_up.add_css_class("flat");
        btn_up.add_css_class("scratch-ctrl-btn");
        {
            let this = self.clone();
            btn_up.connect_clicked(move |_| {
                this.move_up(block_id);
            });
        }
        header.append(&btn_up);

        // Move Down
        let btn_down = gtk::Button::from_icon_name("go-down-symbolic");
        btn_down.add_css_class("flat");
        btn_down.add_css_class("scratch-ctrl-btn");
        {
            let this = self.clone();
            btn_down.connect_clicked(move |_| {
                this.move_down(block_id);
            });
        }
        header.append(&btn_down);

        // Delete Block
        let btn_del = gtk::Button::from_icon_name("edit-delete-symbolic");
        btn_del.add_css_class("flat");
        btn_del.add_css_class("scratch-ctrl-btn");
        btn_del.add_css_class("destructive-action");
        {
            let this = self.clone();
            btn_del.connect_clicked(move |_| {
                this.remove_block(block_id);
            });
        }
        header.append(&btn_del);
    }

    pub fn add_block(&self, kind: BlockKind) {
        let mut nid = *self.next_id.borrow();
        nid += 1;
        *self.next_id.borrow_mut() = nid;

        let block = Block { id: nid, kind };
        self.blocks.borrow_mut().push(block);
        self.rebuild_workspace();
        self.notify_change();
    }

    pub fn add_block_to_body(&self, parent_id: u64, kind: BlockKind) {
        let mut nid = *self.next_id.borrow();
        nid += 1;
        *self.next_id.borrow_mut() = nid;

        let new_block = Block { id: nid, kind };
        add_to_body_recursive(&mut self.blocks.borrow_mut(), parent_id, new_block);
        self.rebuild_workspace();
        self.notify_change();
    }

    pub fn remove_block(&self, id: u64) {
        remove_block_recursive(&mut self.blocks.borrow_mut(), id);
        self.rebuild_workspace();
        self.notify_change();
    }

    pub fn move_up(&self, id: u64) {
        move_block_up_recursive(&mut self.blocks.borrow_mut(), id);
        self.rebuild_workspace();
        self.notify_change();
    }

    pub fn move_down(&self, id: u64) {
        move_block_down_recursive(&mut self.blocks.borrow_mut(), id);
        self.rebuild_workspace();
        self.notify_change();
    }

    pub fn update_times(&self, id: u64, times: usize) {
        update_times_recursive(&mut self.blocks.borrow_mut(), id, times);
        self.notify_change();
    }

    pub fn update_cond(&self, id: u64, cond: ConditionKind) {
        update_condition_recursive(&mut self.blocks.borrow_mut(), id, cond);
        self.notify_change();
    }

    pub fn clear(&self) {
        self.blocks.borrow_mut().clear();
        self.rebuild_workspace();
        self.notify_change();
    }

    pub fn load_preset(&self, preset: PresetKind) {
        let mut nid = *self.next_id.borrow();

        let new_blocks = match preset {
            PresetKind::FirstSteps => {
                let mut b = Vec::new();
                for k in [
                    BlockKind::MoveRight,
                    BlockKind::Paint,
                    BlockKind::MoveRight,
                    BlockKind::Paint,
                    BlockKind::MoveDown,
                    BlockKind::Paint,
                    BlockKind::MoveDown,
                    BlockKind::Paint,
                ] {
                    nid += 1;
                    b.push(Block { id: nid, kind: k });
                }
                b
            }
            PresetKind::Square => {
                let mut loops = Vec::new();
                for dir in [
                    BlockKind::MoveRight,
                    BlockKind::MoveDown,
                    BlockKind::MoveLeft,
                    BlockKind::MoveUp,
                ] {
                    nid += 1;
                    let b1 = Block { id: nid, kind: dir };
                    nid += 1;
                    let b2 = Block { id: nid, kind: BlockKind::Paint };
                    nid += 1;
                    loops.push(Block {
                        id: nid,
                        kind: BlockKind::RepeatTimes {
                            times: 3,
                            body: vec![b1, b2],
                        },
                    });
                }
                loops
            }
            PresetKind::Stairs => {
                nid += 1;
                let b1 = Block { id: nid, kind: BlockKind::MoveRight };
                nid += 1;
                let b2 = Block { id: nid, kind: BlockKind::Paint };
                nid += 1;
                let b3 = Block { id: nid, kind: BlockKind::MoveDown };
                nid += 1;
                let b4 = Block { id: nid, kind: BlockKind::Paint };
                nid += 1;
                vec![Block {
                    id: nid,
                    kind: BlockKind::RepeatTimes {
                        times: 4,
                        body: vec![b1, b2, b3, b4],
                    },
                }]
            }
            PresetKind::Snake => {
                nid += 1;
                let b1 = Block { id: nid, kind: BlockKind::MoveRight };
                nid += 1;
                let b2 = Block { id: nid, kind: BlockKind::Paint };
                nid += 1;
                vec![Block {
                    id: nid,
                    kind: BlockKind::RepeatWhile {
                        cond: ConditionKind::RightFree,
                        body: vec![b1, b2],
                    },
                }]
            }
        };

        *self.next_id.borrow_mut() = nid;
        *self.blocks.borrow_mut() = new_blocks;
        self.rebuild_workspace();
        self.notify_change();
    }
}

// -----------------------------------------------------------------------------
// Recursive State Operations
// -----------------------------------------------------------------------------

fn remove_block_recursive(blocks: &mut Vec<Block>, id: u64) -> bool {
    if let Some(pos) = blocks.iter().position(|b| b.id == id) {
        blocks.remove(pos);
        return true;
    }
    for b in blocks.iter_mut() {
        match &mut b.kind {
            BlockKind::RepeatTimes { body, .. }
            | BlockKind::RepeatWhile { body, .. }
            | BlockKind::If { then_body: body, .. } => {
                if remove_block_recursive(body, id) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn move_block_up_recursive(blocks: &mut Vec<Block>, id: u64) -> bool {
    if let Some(pos) = blocks.iter().position(|b| b.id == id) {
        if pos > 0 {
            blocks.swap(pos, pos - 1);
            return true;
        }
        return false;
    }
    for b in blocks.iter_mut() {
        match &mut b.kind {
            BlockKind::RepeatTimes { body, .. }
            | BlockKind::RepeatWhile { body, .. }
            | BlockKind::If { then_body: body, .. } => {
                if move_block_up_recursive(body, id) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn move_block_down_recursive(blocks: &mut Vec<Block>, id: u64) -> bool {
    if let Some(pos) = blocks.iter().position(|b| b.id == id) {
        if pos + 1 < blocks.len() {
            blocks.swap(pos, pos + 1);
            return true;
        }
        return false;
    }
    for b in blocks.iter_mut() {
        match &mut b.kind {
            BlockKind::RepeatTimes { body, .. }
            | BlockKind::RepeatWhile { body, .. }
            | BlockKind::If { then_body: body, .. } => {
                if move_block_down_recursive(body, id) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn add_to_body_recursive(blocks: &mut [Block], parent_id: u64, new_block: Block) -> bool {
    for b in blocks.iter_mut() {
        if b.id == parent_id {
            match &mut b.kind {
                BlockKind::RepeatTimes { body, .. }
                | BlockKind::RepeatWhile { body, .. }
                | BlockKind::If { then_body: body, .. } => {
                    body.push(new_block);
                    return true;
                }
                _ => {}
            }
        }
        match &mut b.kind {
            BlockKind::RepeatTimes { body, .. }
            | BlockKind::RepeatWhile { body, .. }
            | BlockKind::If { then_body: body, .. } => {
                if add_to_body_recursive(body, parent_id, new_block.clone()) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn update_times_recursive(blocks: &mut [Block], id: u64, times: usize) -> bool {
    for b in blocks.iter_mut() {
        if b.id == id {
            if let BlockKind::RepeatTimes { times: t, .. } = &mut b.kind {
                *t = times;
                return true;
            }
        }
        match &mut b.kind {
            BlockKind::RepeatTimes { body, .. }
            | BlockKind::RepeatWhile { body, .. }
            | BlockKind::If { then_body: body, .. } => {
                if update_times_recursive(body, id, times) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn update_condition_recursive(blocks: &mut [Block], id: u64, new_cond: ConditionKind) -> bool {
    for b in blocks.iter_mut() {
        if b.id == id {
            match &mut b.kind {
                BlockKind::RepeatWhile { cond, .. } | BlockKind::If { cond, .. } => {
                    *cond = new_cond;
                    return true;
                }
                _ => {}
            }
        }
        match &mut b.kind {
            BlockKind::RepeatTimes { body, .. }
            | BlockKind::RepeatWhile { body, .. }
            | BlockKind::If { then_body: body, .. } => {
                if update_condition_recursive(body, id, new_cond) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

// -----------------------------------------------------------------------------
// KuMir Code Generation & Parsing
// -----------------------------------------------------------------------------

pub fn generate_kumir(blocks: &[Block]) -> String {
    let mut out = String::from("использовать Робот\nалг\nнач\n");
    format_blocks_to_kumir(blocks, 1, &mut out);
    out.push_str("кон\n");
    out
}

fn format_blocks_to_kumir(blocks: &[Block], indent: usize, out: &mut String) {
    let pad = "    ".repeat(indent);
    for b in blocks {
        match &b.kind {
            BlockKind::MoveRight => out.push_str(&format!("{pad}вправо\n")),
            BlockKind::MoveLeft => out.push_str(&format!("{pad}влево\n")),
            BlockKind::MoveUp => out.push_str(&format!("{pad}вверх\n")),
            BlockKind::MoveDown => out.push_str(&format!("{pad}вниз\n")),
            BlockKind::Paint => out.push_str(&format!("{pad}закрасить\n")),
            BlockKind::RepeatTimes { times, body } => {
                out.push_str(&format!("{pad}нц {} раз\n", times));
                format_blocks_to_kumir(body, indent + 1, out);
                out.push_str(&format!("{pad}кц\n"));
            }
            BlockKind::RepeatWhile { cond, body } => {
                out.push_str(&format!("{pad}нц пока {}\n", cond.to_kumir()));
                format_blocks_to_kumir(body, indent + 1, out);
                out.push_str(&format!("{pad}кц\n"));
            }
            BlockKind::If { cond, then_body } => {
                out.push_str(&format!("{pad}если {} то\n", cond.to_kumir()));
                format_blocks_to_kumir(then_body, indent + 1, out);
                out.push_str(&format!("{pad}все\n"));
            }
        }
    }
}

pub fn parse_kumir_to_blocks(code: &str, next_id: &mut u64) -> Result<Vec<Block>, String> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    Ok(stmts_to_blocks(&program.main_body, next_id))
}

fn stmts_to_blocks(stmts: &[Stmt], next_id: &mut u64) -> Vec<Block> {
    let mut blocks = Vec::new();
    for stmt in stmts {
        let block_kind = match stmt {
            Stmt::RobotCmd { action, .. } => match action {
                RobotAction::Vpravo => Some(BlockKind::MoveRight),
                RobotAction::Vlevo => Some(BlockKind::MoveLeft),
                RobotAction::Vverh => Some(BlockKind::MoveUp),
                RobotAction::Vniz => Some(BlockKind::MoveDown),
                RobotAction::Zakrasit => Some(BlockKind::Paint),
            },
            Stmt::RepeatTimes { times, body, .. } => {
                let count = match times {
                    Expr::Int(n) => (*n).max(1) as usize,
                    _ => 4,
                };
                Some(BlockKind::RepeatTimes {
                    times: count,
                    body: stmts_to_blocks(body, next_id),
                })
            }
            Stmt::While { cond, body, .. } => {
                let cond_kind = expr_to_condition(cond).unwrap_or(ConditionKind::RightFree);
                Some(BlockKind::RepeatWhile {
                    cond: cond_kind,
                    body: stmts_to_blocks(body, next_id),
                })
            }
            Stmt::If { cond, then_branch, .. } => {
                let cond_kind = expr_to_condition(cond).unwrap_or(ConditionKind::RightFree);
                Some(BlockKind::If {
                    cond: cond_kind,
                    then_body: stmts_to_blocks(then_branch, next_id),
                })
            }
            _ => None,
        };

        if let Some(kind) = block_kind {
            *next_id += 1;
            blocks.push(Block { id: *next_id, kind });
        }
    }
    blocks
}

fn expr_to_condition(expr: &Expr) -> Option<ConditionKind> {
    match expr {
        Expr::RobotWallCheck { dir, expect_wall } => match (dir, expect_wall) {
            (Direction::Right, false) => Some(ConditionKind::RightFree),
            (Direction::Left, false) => Some(ConditionKind::LeftFree),
            (Direction::Up, false) => Some(ConditionKind::UpFree),
            (Direction::Down, false) => Some(ConditionKind::DownFree),
            (Direction::Right, true) => Some(ConditionKind::RightWall),
            (Direction::Left, true) => Some(ConditionKind::LeftWall),
            (Direction::Up, true) => Some(ConditionKind::UpWall),
            (Direction::Down, true) => Some(ConditionKind::DownWall),
        },
        Expr::RobotPaintedCheck { expect_painted } => {
            if *expect_painted {
                Some(ConditionKind::CellPainted)
            } else {
                Some(ConditionKind::CellClean)
            }
        }
        _ => None,
    }
}
