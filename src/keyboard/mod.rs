use egui::{Pos2, Rect, Ui};
use kle_serial::f32::Keyboard;

use crate::{KeyBox, Keycode};

use self::press_time::PressTimesMap;

pub mod press_time;

pub struct KeyboardWidget {
    // different from key numbers (and OSs)
    keyboard_layout: Keyboard,
    // [0, 1], the hue of the color
    hue: f32,
}

impl KeyboardWidget {
    pub fn new(hue: f32, keyboard_layout: Keyboard) -> Self {
        Self {
            keyboard_layout,
            hue,
        }
    }
}

impl From<Keyboard> for KeyboardWidget {
    fn from(keyboard_layout: Keyboard) -> Self {
        Self {
            keyboard_layout,
            hue: 320. / 260.,
        }
    }
}

impl KeyboardWidget {
    pub fn draw(&mut self, map: &mut PressTimesMap, ui: &mut Ui) {
        let Pos2 { x: min_x, y: min_y } = ui.min_rect().min;
        let gap = 2.;
        let key_scale = 60.;

        self.keyboard_layout
            .keys
            .iter()
            .enumerate()
            .for_each(|(i, key)| {
                let normalized_x = key.x * key_scale + gap * (key.x - 1.);
                let normalized_y = key.y * key_scale + gap * (key.y - 1.);
                let normalized_width = key.width * key_scale - gap;
                let normalized_height = key.height * key_scale - gap;

                let rect = Rect {
                    min: Pos2::new(
                        min_x + key.rx + normalized_x,
                        min_y + normalized_y
                    ),
                    max: Pos2::new(
                        min_x + key.rx + normalized_x + normalized_width,
                        min_y + normalized_y + normalized_height,
                    ),
                };
                let times = map.get_key_times(Keycode::from(key.clone()));
                KeyBox::new(key.clone(), times, self.hue).ui(ui, rect, i as u32 + 2);
            });

    }
}
