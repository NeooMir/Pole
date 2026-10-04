use crate::robot::RobotField;
use gtk::cairo::Context;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct RobotFieldView {
    pub widget: gtk::DrawingArea,
    pub field: Rc<RefCell<RobotField>>,
}

impl RobotFieldView {
    pub fn new(field: Rc<RefCell<RobotField>>) -> Self {
        let drawing_area = gtk::DrawingArea::new();
        drawing_area.set_vexpand(true);
        drawing_area.set_hexpand(true);
        drawing_area.set_content_width(450);
        drawing_area.set_content_height(350);

        let field_clone = field.clone();
        drawing_area.set_draw_func(move |_area, cr, width, height| {
            let f = field_clone.borrow();
            Self::draw_field(&f, cr, width as f64, height as f64);
        });

        // Mouse click gesture for interactive field editing
        let click_gesture = gtk::GestureClick::new();
        click_gesture.set_button(0); // handle all mouse buttons
        let field_click = field.clone();
        let da_weak = drawing_area.downgrade();

        click_gesture.connect_pressed(move |gesture, n_press, x, y| {
            if n_press != 1 {
                return;
            }
            let button = gesture.current_button();
            let da = match da_weak.upgrade() {
                Some(d) => d,
                None => return,
            };

            let width = da.width() as f64;
            let height = da.height() as f64;

            let mut f = field_click.borrow_mut();
            let (cell_size, offset_x, offset_y) = Self::compute_layout(&f, width, height);

            let rel_x = x - offset_x;
            let rel_y = y - offset_y;

            if rel_x < 0.0 || rel_y < 0.0 {
                return;
            }

            let col = (rel_x / cell_size) as usize;
            let row = (rel_y / cell_size) as usize;

            if col >= f.width || row >= f.height {
                return;
            }

            match button {
                1 => {
                    // Left click: toggle cell paint or set wall at boundary
                    let fx = (rel_x / cell_size).fract();
                    let fy = (rel_y / cell_size).fract();
                    let threshold = 0.22;

                    if fy < threshold && row > 0 {
                        f.toggle_h_wall(row - 1, col);
                    } else if fy > (1.0 - threshold) && row + 1 < f.height {
                        f.toggle_h_wall(row, col);
                    } else if fx < threshold && col > 0 {
                        f.toggle_v_wall(row, col - 1);
                    } else if fx > (1.0 - threshold) && col + 1 < f.width {
                        f.toggle_v_wall(row, col);
                    } else {
                        f.toggle_paint_cell(col, row);
                    }
                }
                3 => {
                    // Right click: move Robot's start position
                    f.set_start_pos(col, row);
                }
                _ => {}
            }

            da.queue_draw();
        });

        drawing_area.add_controller(click_gesture);

        Self {
            widget: drawing_area,
            field,
        }
    }

    fn compute_layout(field: &RobotField, width: f64, height: f64) -> (f64, f64, f64) {
        let margin = 20.0;
        let available_w = (width - margin * 2.0).max(10.0);
        let available_h = (height - margin * 2.0).max(10.0);

        let cell_w = available_w / field.width as f64;
        let cell_h = available_h / field.height as f64;
        let cell_size = cell_w.min(cell_h).clamp(24.0, 72.0);

        let grid_w = cell_size * field.width as f64;
        let grid_h = cell_size * field.height as f64;

        let offset_x = ((width - grid_w) / 2.0).max(margin);
        let offset_y = ((height - grid_h) / 2.0).max(margin);

        (cell_size, offset_x, offset_y)
    }

    fn draw_field(field: &RobotField, cr: &Context, width: f64, height: f64) {
        // Field background fill
        cr.set_source_rgb(0.97, 0.98, 0.99);
        cr.rectangle(0.0, 0.0, width, height);
        let _ = cr.fill();

        let (cell_size, offset_x, offset_y) = Self::compute_layout(field, width, height);

        // Draw painted cells
        cr.set_source_rgb(0.72, 0.78, 0.85); // Classic KuMir gray-blue paint color
        for r in 0..field.height {
            for c in 0..field.width {
                if field.painted[r][c] {
                    let x = offset_x + c as f64 * cell_size;
                    let y = offset_y + r as f64 * cell_size;
                    cr.rectangle(x + 1.0, y + 1.0, cell_size - 2.0, cell_size - 2.0);
                    let _ = cr.fill();
                }
            }
        }

        // Draw coordinate grid lines
        cr.set_source_rgb(0.78, 0.80, 0.84);
        cr.set_line_width(1.0);

        for c in 0..=field.width {
            let x = offset_x + c as f64 * cell_size;
            cr.move_to(x, offset_y);
            cr.line_to(x, offset_y + field.height as f64 * cell_size);
            let _ = cr.stroke();
        }

        for r in 0..=field.height {
            let y = offset_y + r as f64 * cell_size;
            cr.move_to(offset_x, y);
            cr.line_to(offset_x + field.width as f64 * cell_size, y);
            let _ = cr.stroke();
        }

        // Draw trace of the passed path
        if field.trace.len() > 1 {
            cr.set_source_rgba(0.2, 0.5, 0.9, 0.35);
            for &(tx, ty) in &field.trace {
                let cx = offset_x + tx as f64 * cell_size + cell_size * 0.5;
                let cy = offset_y + ty as f64 * cell_size + cell_size * 0.5;
                cr.arc(cx, cy, 3.0, 0.0, std::f64::consts::TAU);
                let _ = cr.fill();
            }
        }

        // Draw internal horizontal walls
        cr.set_source_rgb(0.15, 0.18, 0.22);
        cr.set_line_width(4.5);
        cr.set_line_cap(gtk::cairo::LineCap::Round);

        for r in 0..field.h_walls.len() {
            for c in 0..field.width {
                if field.h_walls[r][c] {
                    let x1 = offset_x + c as f64 * cell_size;
                    let x2 = x1 + cell_size;
                    let y = offset_y + (r + 1) as f64 * cell_size;
                    cr.move_to(x1, y);
                    cr.line_to(x2, y);
                    let _ = cr.stroke();
                }
            }
        }

        // Draw internal vertical walls
        for r in 0..field.height {
            for c in 0..field.v_walls[0].len() {
                if field.v_walls[r][c] {
                    let x = offset_x + (c + 1) as f64 * cell_size;
                    let y1 = offset_y + r as f64 * cell_size;
                    let y2 = y1 + cell_size;
                    cr.move_to(x, y1);
                    cr.line_to(x, y2);
                    let _ = cr.stroke();
                }
            }
        }

        // Draw outer field boundaries (thick KuMir frame)
        cr.set_source_rgb(0.12, 0.14, 0.18);
        cr.set_line_width(5.0);
        let grid_w = cell_size * field.width as f64;
        let grid_h = cell_size * field.height as f64;
        cr.rectangle(offset_x, offset_y, grid_w, grid_h);
        let _ = cr.stroke();

        // Calculate screen coordinates of the Robot
        let rx = offset_x + field.robot_x as f64 * cell_size + cell_size * 0.5;
        let ry = offset_y + field.robot_y as f64 * cell_size + cell_size * 0.5;
        let radius = cell_size * 0.36;

        // Draw the Robot actor
        if field.crashed {
            // Crashed state (red warning cross)
            cr.set_source_rgb(0.9, 0.2, 0.2);
            cr.arc(rx, ry, radius, 0.0, std::f64::consts::TAU);
            let _ = cr.fill();

            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.set_line_width(3.0);
            let d = radius * 0.55;
            cr.move_to(rx - d, ry - d);
            cr.line_to(rx + d, ry + d);
            cr.move_to(rx + d, ry - d);
            cr.line_to(rx - d, ry + d);
            let _ = cr.stroke();
        } else {
            // Robot actor: classic rhombus with an eye
            // Robot shadow
            cr.set_source_rgba(0.0, 0.0, 0.0, 0.18);
            cr.arc(rx, ry + 2.0, radius * 0.9, 0.0, std::f64::consts::TAU);
            let _ = cr.fill();

            // Body (rhombus)
            cr.set_source_rgb(0.18, 0.45, 0.85); // Classic KuMir blue color
            cr.move_to(rx, ry - radius);
            cr.line_to(rx + radius, ry);
            cr.line_to(rx, ry + radius);
            cr.line_to(rx - radius, ry);
            cr.close_path();
            let _ = cr.fill();

            // Body stroke
            cr.set_source_rgb(0.08, 0.25, 0.55);
            cr.set_line_width(2.0);
            cr.move_to(rx, ry - radius);
            cr.line_to(rx + radius, ry);
            cr.line_to(rx, ry + radius);
            cr.line_to(rx - radius, ry);
            cr.close_path();
            let _ = cr.stroke();

            // Robot eye
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.arc(rx, ry, radius * 0.38, 0.0, std::f64::consts::TAU);
            let _ = cr.fill();

            // Pupil
            cr.set_source_rgb(0.15, 0.2, 0.3);
            cr.arc(rx, ry, radius * 0.18, 0.0, std::f64::consts::TAU);
            let _ = cr.fill();
        }
    }
}