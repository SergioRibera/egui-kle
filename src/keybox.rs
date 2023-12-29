use egui::{Align2, Color32, Sense, Stroke, Ui, Vec2};
use kle_serial::f32::Key;

use crate::color::{get_color, get_strike_color};

const POSITIONS: [Align2; 9] = [
    Align2::LEFT_TOP,
    Align2::CENTER_TOP,
    Align2::RIGHT_TOP,
    Align2::LEFT_CENTER,
    Align2::CENTER_CENTER,
    Align2::RIGHT_CENTER,
    Align2::LEFT_BOTTOM,
    Align2::CENTER_BOTTOM,
    Align2::RIGHT_BOTTOM,
];

/// component of a key on a keyboard
#[allow(dead_code)]
pub struct KeyBox {
    size: Vec2,
    rounding: f32,
    stroke_width: f32,
    key: Key,
    press_times: u32,
    hue: f32,
}

impl KeyBox {
    pub fn new(size: Vec2, key: Key, press_times: u32, hue: f32) -> KeyBox {
        Self {
            size,
            rounding: 5.0,
            stroke_width: 2.0,
            key,
            press_times,
            hue,
        }
    }
}
impl KeyBox {
    pub fn ui(&mut self, ui: &mut Ui) {
        let (rect, _resp) = ui.allocate_exact_size(self.size, Sense::hover());
        let filled_color = get_color(self.hue, self.press_times);
        ui.painter().rect_filled(rect, self.rounding, filled_color);
        self.key.legends.iter().enumerate().for_each(|(i, key)| {
            if let Some(key) = key {
                ui.painter().text(
                    rect.center(),
                    POSITIONS[i],
                    &key.text,
                    egui::FontId::monospace(13.),
                    Color32::from_rgb(32, 5, 64),
                );
            }
        });

        ui.painter().rect_stroke(
            rect,
            self.rounding,
            Stroke {
                width: self.stroke_width,
                color: get_strike_color(filled_color),
            },
        );
    }
}
