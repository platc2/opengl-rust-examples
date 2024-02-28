pub use key::Key;
pub(crate) use sdl_input_manager::SdlInputManager;

mod key;
mod key_helpers;
mod sdl_input_manager;

pub trait InputManager {
    #[must_use]
    fn key_pressed(&self, key: Key) -> bool;

    #[must_use]
    fn key_released(&self, key: Key) -> bool;

    #[must_use]
    fn key_down(&self, key: Key) -> bool;

    #[must_use]
    fn key_up(&self, key: Key) -> bool { !self.key_down(key) }

    #[must_use]
    fn mouse_position(&self) -> (i32, i32);

    #[must_use]
    fn mouse_movement(&self) -> (i32, i32);

    #[must_use]
    fn scroll(&self) -> (i32, i32);
}
