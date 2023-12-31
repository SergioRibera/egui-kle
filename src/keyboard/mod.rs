use egui::ahash::HashMap;
use egui::{Pos2, Rect, Ui};
use kle_serial::f32::Keyboard;

use crate::KeyBox;

use self::press_time::{text_from_key, PressTimesMap};

pub mod press_time;

pub struct KeyboardWidget {
    // different from key numbers (and OSs)
    keyboard_layout: Keyboard,

    total_width: f32,
}

impl KeyboardWidget {
    pub fn new(keyboard_layout: Keyboard) -> Self {
        let mut row_counts = HashMap::default();
        for key in &keyboard_layout.keys {
            let _ = *row_counts
                .entry(format!("{}", key.y.round()))
                .and_modify(|old: &mut f32| {
                    *old = old.max(key.x);
                })
                .or_insert(0.);
        }
        let total_width = row_counts
            .iter()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .unwrap()
            .1;
        Self {
            total_width: *total_width + 2.,
            keyboard_layout,
        }
    }
    pub fn draw(&mut self, ui: &mut Ui, map: Option<&PressTimesMap>) {
        let max_width = ui.available_width();
        let scale = max_width / self.total_width;

        let Pos2 { x: min_x, y: min_y } = ui.min_rect().min;
        let gap = 2.;

        self.keyboard_layout.keys.iter().for_each(|key| {
            let normalized_x = key.x * scale + gap * (key.x - 1.);
            let normalized_y = key.y * scale + gap * (key.y - 1.);
            let normalized_width = key.width * scale - gap;
            let normalized_height = key.height * scale - gap;

            let rect = Rect {
                min: Pos2::new(min_x + key.rx + normalized_x, min_y + normalized_y),
                max: Pos2::new(
                    min_x + key.rx + normalized_x + normalized_width,
                    min_y + normalized_y + normalized_height,
                ),
            };
            let times = if let Some(map) = map {
                map.get_key_times(text_from_key(key.clone()))
            } else {
                20
            };
            KeyBox::new(key.clone(), scale, times).ui(ui, rect);
        });
    }
}
