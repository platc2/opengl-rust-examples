use std::collections::HashSet;

use sdl2::keyboard::Scancode as SdlKey;

use super::{InputManager, Key};

#[derive(Default)]
pub struct SdlInputManager {
    last_pressed_keys: HashSet<Key>,
    curr_pressed_keys: HashSet<Key>,

    new_mouse_position: (i32, i32),
    mouse_position: (i32, i32),
    mouse_movement: (i32, i32),
    scroll: (i32, i32),
}

impl InputManager for SdlInputManager {
    fn key_pressed(&self, key: Key) -> bool {
        self.key_down(key) && !self.last_pressed_keys.contains(&key)
    }

    fn key_released(&self, key: Key) -> bool {
        self.key_up(key) && self.last_pressed_keys.contains(&key)
    }

    fn key_down(&self, key: Key) -> bool {
        self.curr_pressed_keys.contains(&key)
    }

    fn key_up(&self, key: Key) -> bool {
        !self.curr_pressed_keys.contains(&key)
    }

    fn mouse_position(&self) -> (i32, i32) {
        self.mouse_position
    }

    fn mouse_movement(&self) -> (i32, i32) {
        self.mouse_movement
    }

    fn scroll(&self) -> (i32, i32) { self.scroll }
}

impl SdlInputManager {
    pub fn update(&mut self) {
        self.last_pressed_keys.clear();
        for key in self.curr_pressed_keys.clone() {
            self.last_pressed_keys.insert(key);
        }

        self.mouse_movement = (0, 0);
        self.scroll = (0, 0);
    }

    pub fn set_key_down(&mut self, key: SdlKey) {
        if let Ok(k) = key.try_into() {
            self.curr_pressed_keys.insert(k);
        }
    }

    pub fn set_key_up(&mut self, key: SdlKey) {
        if let Ok(k) = key.try_into() {
            self.curr_pressed_keys.remove(&k);
        }
    }

    pub fn set_mouse_position(&mut self, mouse_position: (i32, i32)) {
        self.new_mouse_position = mouse_position;
    }

    pub fn add_mouse_movement(&mut self, mouse_movement: (i32, i32)) {
        self.mouse_movement = (
            self.mouse_movement.0 + mouse_movement.0,
            self.mouse_movement.1 + mouse_movement.1
        );
    }

    pub fn add_scroll(&mut self, scroll: (i32, i32)) {
        self.scroll = (
            self.scroll.0 + scroll.0,
            self.scroll.1 + scroll.1
        );
    }
}
