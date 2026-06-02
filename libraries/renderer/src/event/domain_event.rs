use sdl2::mouse::MouseButton;
use crate::application::ApplicationState;
use crate::input::Key;

#[derive(Debug, Clone, PartialEq)]
pub enum DomainEvent {
    QuitRequested,
    KeyDown(Key),
    KeyUp(Key),
    MouseMotion {
        x: i32,
        y: i32,
        delta_x: i32,
        delta_y: i32,
    },
    MouseButtonDown(MouseButton),
    MouseButtonUp(MouseButton),
    MouseWheel {
        x: f32,
        y: f32,
    },
    TextInput(String),
    StateSwitch(ApplicationState),
}
