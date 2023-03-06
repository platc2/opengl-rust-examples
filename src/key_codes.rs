use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use sdl2::keyboard::Keycode;

#[derive(Default)]
pub struct KeyCodes {
    codes: HashMap<Keycode, bool>,
}

impl Index<Keycode> for KeyCodes {
    type Output = bool;

    fn index(&self, index: Keycode) -> &Self::Output {
        self.codes.get(&index).unwrap_or(&false)
    }
}

impl IndexMut<Keycode> for KeyCodes {
    fn index_mut(&mut self, index: Keycode) -> &mut Self::Output {
        self.codes.entry(index).or_insert(false)
    }
}
