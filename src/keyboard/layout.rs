use std::slice::Iter;

use crate::keycode::Keycode;

pub struct KeyboardLayout {
    layout: Vec<Vec<Option<Keycode>>>,
}

impl KeyboardLayout {
    pub fn iter(&self) -> Iter<Vec<Option<Keycode>>> {
        self.layout.iter()
    }
}

impl FromIterator<IntoIteratorIter<'_, Option<Keycode>>> for KeyboardLayout {
    fn from_iter<T: IntoIterator<Item = Iter<'_, Option<Keycode>>>>(iter: T) -> Self {
        Self {
            layout: iter
                .into_iter()
                .map(|r| r.into_iter().collect::<Vec<Option<Keycode>>>())
                .collect(),
        }
    }
}

impl From<&[&[Option<Keycode>]]> for KeyboardLayout {
    fn from(value: &[&[Option<Keycode>]]) -> Self {
        Self {
            layout: value.iter().map(|r| r.to_vec()).collect(),
        }
    }
}
