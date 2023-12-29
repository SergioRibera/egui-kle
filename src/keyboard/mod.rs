use egui::{Color32, Sense, Ui, Vec2};
use kle_serial::f32::{Key, Keyboard as KeyboardLayout};

use crate::{KeyBox, Keycode};

use self::press_time::PressTimesMap;

pub mod press_time;

pub struct KeyboardWidget {
    // different from key numbers (and OSs)
    keyboard_layout: KeyboardLayout,
    // [0, 1], the hue of the color
    hue: f32,
}

impl KeyboardWidget {
    pub fn new(hue: f32, keyboard_layout: KeyboardLayout) -> Self {
        Self {
            keyboard_layout,
            hue,
        }
    }
}

impl From<KeyboardLayout> for KeyboardWidget {
    fn from(keyboard_layout: KeyboardLayout) -> Self {
        Self {
            keyboard_layout,
            hue: 320. / 260.,
        }
    }
}

impl KeyboardWidget {
    pub fn draw(&mut self, map: &mut PressTimesMap, ui: &mut Ui) {
        self.keyboard_layout.keys.iter().for_each(|key| {
            let size = Vec2::new(key.width, key.height);
            self.draw_single_label_key(map, size, key.clone(), ui);
        });
    }

    fn draw_single_label_key(&self, map: &mut PressTimesMap, size: Vec2, key: Key, ui: &mut Ui) {
        let times = map.get_key_times(Keycode::from(key.clone()));
        let mut key = KeyBox::new(size, key, times, self.hue);
        key.ui(ui);
    }

    #[allow(dead_code)]
    fn draw_empty_key(&self, size: Vec2, ui: &mut Ui) {
        let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
        ui.painter().rect_filled(rect, 0., Color32::TRANSPARENT);
    }
}
