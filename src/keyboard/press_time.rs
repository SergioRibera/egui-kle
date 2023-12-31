use std::fmt::Debug;

use dashmap::DashMap;
use kle_serial::f32::Key;

#[derive(Clone, Default, Debug)]
pub struct PressTimesMap {
    map: DashMap<String, u32>,
}

impl PressTimesMap {
    pub fn key_press<T: Debug>(&self, key: T) {
        self.map
            .entry(format!("{key:?}"))
            .and_modify(|old| *old += 1)
            .or_insert(0);
    }

    pub fn key_release<T: Debug>(&self, key: T) {
        self.map
            .entry(format!("{key:?}"))
            .and_modify(|old| {
                if *old > 0 {
                    *old -= 1;
                }
            })
            .or_insert(0);
    }

    pub fn get_key_times(&self, key: String) -> u32 {
        self.map.get(&key).map(|v| *v).unwrap_or_default()
    }
}

pub fn text_from_key(value: Key) -> String {
    let keycodes = value
        .legends
        .iter()
        .filter_map(|v| v.as_ref())
        .collect::<Vec<_>>();

    keycodes.first().unwrap().text.clone()
}
