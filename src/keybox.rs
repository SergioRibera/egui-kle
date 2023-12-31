use egui::{Align2, Color32, Pos2, Rect, Sense, Stroke, Ui};
use kle_serial::f32::Key;

use crate::color::{get_color, get_strike_color, hue_from_rgb};

fn calc_pos(r_rect: Rect, pos: usize, size: usize) -> Pos2 {
    let mut rect = r_rect.clone();
    rect = Rect {
        min: Pos2::new(rect.min.x + 7., rect.min.y + 15. + (size as f32)),
        max: Pos2::new(rect.max.x - 7., rect.max.y - (size as f32 * 2.) - 7.),
    };

    match pos {
        0 => rect.left_top(),
        1 => rect.center_top(),
        2 => rect.right_top(),
        3 => rect.left_center(),
        5 => rect.right_center(),
        6 => rect.left_bottom(),
        7 => rect.center_bottom(),
        8 => rect.right_bottom(),
        9 => rect.with_max_y(r_rect.max.y).left_bottom(),
        10 => rect.with_max_y(r_rect.max.y).center_bottom(),
        11 => rect.with_max_y(r_rect.max.y).right_bottom(),
        _ => rect.center(),
    }
}

/// component of a key on a keyboard
#[allow(dead_code)]
pub struct KeyBox {
    rounding: f32,
    stroke_width: f32,
    key: Key,
    press_times: u32,
    hue: f32,

    scale: f32,
    width: f32,
    height: f32,
}

impl KeyBox {
    pub fn new(key: Key, scale: f32, press_times: u32) -> KeyBox {
        let width = key.width * scale;
        let height = key.height * scale;

        let c = key.color;
        let hue = hue_from_rgb(c.r, c.g, c.b);

        Self {
            key,
            hue,
            scale,
            width,
            height,
            press_times,
            rounding: 5.0,
            stroke_width: 2.0,
        }
    }
    pub fn ui(&self, ui: &mut Ui, rect: Rect) {
        let _resp = ui.allocate_rect(rect, Sense::hover());
        let available_area = rect.max.x - rect.min.x;
        let rounding = self.rounding * (self.scale / available_area);
        let filled_color = get_color(self.hue, self.press_times);

        ui.painter().rect_filled(rect, rounding, filled_color);

        if available_area >= 40. {
            self.key.legends.iter().enumerate().for_each(|(i, key)| {
                if let Some(key) = key {
                    ui.painter().text(
                        calc_pos(rect, i, key.size),
                        Align2::LEFT_BOTTOM,
                        &key.text,
                        egui::FontId::monospace(key.size as f32 * 4. * self.scale / available_area),
                        Color32::from_rgb(key.color.r, key.color.g, key.color.b),
                    );
                }
            });
        }

        ui.painter().rect_stroke(
            rect,
            rounding,
            Stroke {
                width: self.stroke_width,
                color: get_strike_color(filled_color),
            },
        );
    }
}
