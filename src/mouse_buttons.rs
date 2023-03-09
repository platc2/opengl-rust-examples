use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use sdl2::mouse::MouseButton;

#[derive(Default)]
pub struct MouseButtons {
    buttons: HashMap<MouseButton, bool>,
}

impl Index<MouseButton> for MouseButtons {
    type Output = bool;

    fn index(&self, index: MouseButton) -> &Self::Output {
        self.buttons.get(&index).unwrap_or(&false)
    }
}

impl IndexMut<MouseButton> for MouseButtons {
    fn index_mut(&mut self, index: MouseButton) -> &mut Self::Output {
        self.buttons.entry(index).or_insert(false)
    }
}
