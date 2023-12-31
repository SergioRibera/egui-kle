use egui::{Pos2, Rect, Ui};
use kle_serial::f32::Keyboard;

use crate::KeyBox;

use self::press_time::{text_from_key, PressTimesMap};

pub mod press_time;

pub struct KeyboardWidget {
    // different from key numbers (and OSs)
    keyboard_layout: Keyboard,
}

impl KeyboardWidget {
    pub fn new(keyboard_layout: Keyboard) -> Self {
        Self { keyboard_layout }
    }
}

impl From<Keyboard> for KeyboardWidget {
    fn from(keyboard_layout: Keyboard) -> Self {
        Self { keyboard_layout }
    }
}

impl KeyboardWidget {
    pub fn draw(&mut self, ui: &mut Ui, map: Option<&PressTimesMap>) {
        let Pos2 { x: min_x, y: min_y } = ui.min_rect().min;
        let gap = 2.;
        let key_scale = 60.;

        self.keyboard_layout.keys.iter().for_each(|key| {
            let normalized_x = key.x * key_scale + gap * (key.x - 1.);
            let normalized_y = key.y * key_scale + gap * (key.y - 1.);
            let normalized_width = key.width * key_scale - gap;
            let normalized_height = key.height * key_scale - gap;

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
            KeyBox::new(key.clone(), times).ui(ui, rect);
        });
    }
}
