use egui::ahash::HashMap;

use crate::keycode::Keycode;

#[derive(Default, Debug)]
pub struct PressTimesMap {
    map: HashMap<Keycode, u32>,
}

impl PressTimesMap {
    pub fn key_press(&mut self, key: Keycode) {
        self.map.entry(key).and_modify(|old| *old += 1).or_insert(0);
    }

    pub fn get_key_times(&self, key: Keycode) -> u32 {
        self.map.get(&key).map(|&v| v).unwrap_or_default()
    }
}
