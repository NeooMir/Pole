use gtk::cairo::{Context, LineCap, LineJoin};
use gtk::prelude::*;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TurtleLine {
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub color: (f64, f64, f64),
    pub width: f64,
}

#[derive(Debug, Clone)]
pub struct TurtleSnapshot {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub pen_down: bool,
    pub lines_len: usize,
}

#[derive(Debug, Clone)]
pub struct TurtleState {
    pub x: f64,
    pub y: f64,
    pub angle: f64, // degrees: 0 = right (+X), 90 = up (+Y)
    pub pen_down: bool,
    pub lines: Vec<TurtleLine>,
    pub history: Vec<TurtleSnapshot>,
}

impl TurtleState {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            angle: 90.0, // default heading: Up
            pen_down: true,
            lines: Vec::new(),
            history: Vec::new(),
        }
    }

    fn record_snapshot(&mut self) {
        self.history.push(TurtleSnapshot {
            x: self.x,
            y: self.y,
            angle: self.angle,
            pen_down: self.pen_down,
            lines_len: self.lines.len(),
        });
    }

    pub fn forward(&mut self, dist: f64) {
        self.record_snapshot();
        let rad = self.angle * PI / 180.0;
        let new_x = self.x + dist * rad.cos();
        let new_y = self.y + dist * rad.sin();

        if self.pen_down {
            self.lines.push(TurtleLine {
                start: (self.x, self.y),
                end: (new_x, new_y),
                color: (0.15, 0.45, 0.85),
                width: 2.0,
            });
        }
        self.x = new_x;
        self.y = new_y;
    }

    pub fn backward(&mut self, dist: f64) {
        self.forward(-dist);
    }

    pub fn turn_left(&mut self, deg: f64) {
        self.record_snapshot();
        self.angle = (self.angle + deg) % 360.0;
        if self.angle < 0.0 {
            self.angle += 360.0;
        }
    }

    pub fn turn_right(&mut self, deg: f64) {
        self.record_snapshot();
        self.angle = (self.angle - deg) % 360.0;
        if self.angle < 0.0 {
            self.angle += 360.0;
        }
    }

    pub fn set_pen(&mut self, down: bool) {
        self.record_snapshot();
        self.pen_down = down;
    }

    pub fn clear(&mut self) {
        self.record_snapshot();
        self.lines.clear();
    }

    pub fn reset(&mut self) {
        self.record_snapshot();
        self.x = 0.0;
        self.y = 0.0;
        self.angle = 90.0;
        self.pen_down = true;
        self.lines.clear();
        self.history.clear();
    }

    pub fn step_back(&mut self) -> bool {
        if let Some(snap) = self.history.pop() {
            self.lines.truncate(snap.lines_len);
            self.x = snap.x;
            self.y = snap.y;
            self.angle = snap.angle;
            self.pen_down = snap.pen_down;
            true
        } else {
            false
        }
    }
}

pub struct TurtleView {
    pub widget: gtk::DrawingArea,
    pub state: Rc<RefCell<TurtleState>>,
}

impl TurtleView {
    pub fn new(state: Rc<RefCell<TurtleState>>) -> Self {
        let drawing_area = gtk::DrawingArea::new();
        drawing_area.set_vexpand(true);
        drawing_area.set_hexpand(true);
        drawing_area.set_content_width(450);
        drawing_area.set_content_height(350);

        let state_clone = state.clone();
        drawing_area.set_draw_func(move |_area, cr, width, height| {
            let s = state_clone.borrow();
            Self::draw_canvas(&s, cr, width as f64, height as f64);
        });

        Self {
            widget: drawing_area,
            state,
        }
    }

    fn draw_canvas(state: &TurtleState, cr: &Context, width: f64, height: f64) {
        // Canvas background fill (clean Adwaita canvas)
        cr.set_source_rgb(0.98, 0.99, 1.0);
        cr.rectangle(0.0, 0.0, width, height);
        let _ = cr.fill();

        let cx = width / 2.0;
        let cy = height / 2.0;

        // Coordinate grid
        cr.set_source_rgba(0.85, 0.88, 0.92, 0.7);
        cr.set_line_width(1.0);

        let step = 50.0;
        let mut x = cx % step;
        while x < width {
            cr.move_to(x, 0.0);
            cr.line_to(x, height);
            x += step;
        }

        let mut y = cy % step;
        while y < height {
            cr.move_to(0.0, y);
            cr.line_to(width, y);
            y += step;
        }
        let _ = cr.stroke();

        // Main axes (X and Y)
        cr.set_source_rgba(0.55, 0.60, 0.68, 0.85);
        cr.set_line_width(1.5);
        cr.move_to(0.0, cy);
        cr.line_to(width, cy);
        cr.move_to(cx, 0.0);
        cr.line_to(cx, height);
        let _ = cr.stroke();

        // Numeric tick marks on axes
        cr.set_source_rgba(0.45, 0.50, 0.58, 0.9);
        cr.set_font_size(10.0);
        let mut val = 100.0;
        while cx + val < width || cx - val > 0.0 {
            if cx + val < width {
                cr.move_to(cx + val + 2.0, cy - 4.0);
                let _ = cr.show_text(&format!("{:.0}", val));
            }
            if cx - val > 0.0 {
                cr.move_to(cx - val + 2.0, cy - 4.0);
                let _ = cr.show_text(&format!("-{:.0}", val));
            }
            val += 100.0;
        }

        let mut val_y = 100.0;
        while cy - val_y > 0.0 || cy + val_y < height {
            if cy - val_y > 0.0 {
                cr.move_to(cx + 4.0, cy - val_y - 2.0);
                let _ = cr.show_text(&format!("{:.0}", val_y));
            }
            if cy + val_y < height {
                cr.move_to(cx + 4.0, cy + val_y - 2.0);
                let _ = cr.show_text(&format!("-{:.0}", val_y));
            }
            val_y += 100.0;
        }

        // Draw accumulated lines
        cr.set_line_cap(LineCap::Round);
        cr.set_line_join(LineJoin::Round);
        for line in &state.lines {
            let sx1 = cx + line.start.0;
            let sy1 = cy - line.start.1;
            let sx2 = cx + line.end.0;
            let sy2 = cy - line.end.1;
            cr.set_source_rgb(line.color.0, line.color.1, line.color.2);
            cr.set_line_width(line.width);
            cr.move_to(sx1, sy1);
            cr.line_to(sx2, sy2);
            let _ = cr.stroke();
        }

        // Render Turtle performer
        let tx = cx + state.x;
        let ty = cy - state.y;
        let rad = state.angle * PI / 180.0;
        let size = 18.0;

        // Turtle shadow
        cr.set_source_rgba(0.0, 0.0, 0.0, 0.15);
        cr.arc(tx, ty + 2.0, size * 0.9, 0.0, 2.0 * PI);
        let _ = cr.fill();

        // 4 flippers / paws
        cr.set_source_rgb(0.18, 0.62, 0.38);
        for f_angle in [rad + 0.7, rad - 0.7, rad + 2.4, rad - 2.4] {
            let fx = tx + (size * 0.9) * f_angle.cos();
            let fy = ty - (size * 0.9) * f_angle.sin();
            cr.arc(fx, fy, size * 0.32, 0.0, 2.0 * PI);
            let _ = cr.fill();
        }

        // Turtle carapace / shell (green dome)
        cr.set_source_rgb(0.15, 0.68, 0.37);
        cr.arc(tx, ty, size * 0.85, 0.0, 2.0 * PI);
        let _ = cr.fill();

        // Shell rim
        cr.set_source_rgb(0.10, 0.48, 0.26);
        cr.set_line_width(2.0);
        cr.arc(tx, ty, size * 0.85, 0.0, 2.0 * PI);
        let _ = cr.stroke();

        // Turtle head oriented towards direction of travel
        let hx = tx + (size * 1.05) * rad.cos();
        let hy = ty - (size * 1.05) * rad.sin();
        cr.set_source_rgb(0.20, 0.75, 0.42);
        cr.arc(hx, hy, size * 0.40, 0.0, 2.0 * PI);
        let _ = cr.fill();

        cr.set_source_rgb(0.10, 0.48, 0.26);
        cr.set_line_width(1.5);
        cr.arc(hx, hy, size * 0.40, 0.0, 2.0 * PI);
        let _ = cr.stroke();

        // Eyes
        let eye_rad1 = rad + 0.6;
        let eye_rad2 = rad - 0.6;
        let eye_dist = size * 0.25;
        cr.set_source_rgb(0.1, 0.1, 0.1);
        cr.arc(hx + eye_dist * eye_rad1.cos(), hy - eye_dist * eye_rad1.sin(), 1.8, 0.0, 2.0 * PI);
        let _ = cr.fill();
        cr.arc(hx + eye_dist * eye_rad2.cos(), hy - eye_dist * eye_rad2.sin(), 1.8, 0.0, 2.0 * PI);
        let _ = cr.fill();

        // Pen / tail status indicator in the center of the shell
        if state.pen_down {
            cr.set_source_rgb(0.92, 0.22, 0.22); // Red dot: pen down (drawing active)
            cr.arc(tx, ty, size * 0.24, 0.0, 2.0 * PI);
            let _ = cr.fill();
        } else {
            cr.set_source_rgb(0.65, 0.68, 0.72); // Gray ring: pen up (drawing inactive)
            cr.set_line_width(2.0);
            cr.arc(tx, ty, size * 0.24, 0.0, 2.0 * PI);
            let _ = cr.stroke();
        }
    }
}
