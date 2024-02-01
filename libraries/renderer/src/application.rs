use std::collections::HashMap;

use anyhow::{anyhow, Result};
use imgui::Ui;
use sdl2::event::Event;
use sdl2::keyboard::Mod;
use sdl2::mouse::MouseButton;

use crate::imgui_wrapper::Imgui;
use crate::input_manager::{InputManager, Key, SdlInputManager};
use crate::renderer_context::RendererContext;
use crate::time::Time;

pub trait Application {
    #[allow(unused)]
    fn tick(&mut self, time: &Time<std::time::Instant>, input_manager: &dyn InputManager) {}

    fn gui(&mut self, #[allow(unused)] ui: &Ui) {}

    fn quit(&self) -> bool { false }
}

pub fn main_loop<T: Application>(context: RendererContext, mut application: T) -> Result<()> {
    let mut time: Time<std::time::Instant> = Time::default();
    let mut event_pump = context.sdl().event_pump()
        .map_err(|e| anyhow!(e))?;
    let mut imgui_context = Imgui::init();
    let mut input_manager = SdlInputManager::default();
    let mut mouse_pos = (0, 0);
    let mut left_mouse = false;
    let mut right_mouse = false;
    while !application.quit() {
        time.update();

        let mut key_changes = HashMap::new();
        let mut text_input: Vec<String> = Vec::new();
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { scancode: Some(scancode), keymod, .. } => {
                    key_changes.insert(Key::MOD_CONTROL, !(keymod & Mod::LCTRLMOD).is_empty());
                    key_changes.insert(Key::MOD_SHIFT, !(keymod & Mod::LSHIFTMOD).is_empty());
                    key_changes.insert(Key::MOD_ALT, !(keymod & Mod::LALTMOD).is_empty());

                    if let Ok(k) = scancode.try_into() {
                        input_manager.set_key_down(scancode);
                        key_changes.insert(k, true);
                    }
                }
                Event::KeyUp { scancode: Some(scancode), keymod, .. } => {
                    key_changes.insert(Key::MOD_CONTROL, !(keymod & Mod::LCTRLMOD).is_empty());
                    key_changes.insert(Key::MOD_SHIFT, !(keymod & Mod::LSHIFTMOD).is_empty());
                    key_changes.insert(Key::MOD_ALT, !(keymod & Mod::LALTMOD).is_empty());

                    if let Ok(k) = scancode.try_into() {
                        input_manager.set_key_up(scancode);
                        key_changes.insert(k, false);
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    mouse_pos = (x as i16, y as i16);
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    match mouse_btn {
                        MouseButton::Left => left_mouse = true,
                        MouseButton::Right => right_mouse = true,
                        _ => (),
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    match mouse_btn {
                        MouseButton::Left => left_mouse = false,
                        MouseButton::Right => right_mouse = false,
                        _ => (),
                    }
                }
                Event::Quit { .. } => std::process::exit(1),
                Event::TextInput { text, .. } => text_input.push(text),
                _ => (),
            }
        }

        let (w, h) = context.window().drawable_size();
        imgui_context.prepare(
            [w as _, h as _],
            [mouse_pos.0.into(), mouse_pos.1.into()],
            [left_mouse, right_mouse],
            &key_changes,
            &text_input,
            time.duration(),
        );

        application.tick(&time, &input_manager);

        imgui_context.render(|ui| application.gui(ui));

        context.window().gl_swap_window();
    }

    Ok(())
}
