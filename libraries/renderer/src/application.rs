use std::collections::HashMap;

use anyhow::{anyhow, Result};
#[cfg(feature = "imgui")]
use imgui::Ui;
use sdl2::event::Event;
use sdl2::keyboard::Mod;
use sdl2::mouse::MouseButton;

#[cfg(feature = "imgui")]
use crate::imgui_impl::Imgui;
use crate::input_manager::{InputManager, Key, SdlInputManager};
use crate::renderer_context::RendererContext;
use crate::time::Time;

pub trait Application {
    #[allow(unused)]
    fn tick(&mut self, time: &Time<std::time::Instant>, input_manager: &dyn InputManager) {}

    #[cfg(feature = "imgui")]
    fn gui(&mut self, #[allow(unused)] ui: &Ui) {}

    fn quit(&self) -> bool { false }
}

pub fn main_loop<T: Application>(context: RendererContext, mut application: T) -> Result<()> {
    let mut time: Time<std::time::Instant> = Time::default();
    let mut event_pump = context.sdl().event_pump()
        .map_err(|e| anyhow!(e))?;
    #[cfg(feature = "imgui")]
        let mut imgui_context = Imgui::init();
    let mut input_manager = SdlInputManager::default();
    let mut relative_mouse_mode = false;
    'mainloop: while !application.quit() {
        time.update();
        if relative_mouse_mode {
            input_manager.update();
        }

        let mut key_changes = HashMap::new();
        let mut text_input: Vec<String> = Vec::new();
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { scancode: Some(scancode), keymod, .. } => {
                    if sdl2::keyboard::Scancode::Escape == scancode {
                        if relative_mouse_mode {
                            relative_mouse_mode = false;
                            context.sdl().mouse().set_relative_mouse_mode(false);
                        } else {
                            break 'mainloop;
                        }
                    }

                    insert_mod_keys(&mut key_changes, keymod);

                    if let Ok(k) = scancode.try_into() {
                        if relative_mouse_mode {
                            input_manager.set_key_down(scancode);
                        }
                        key_changes.insert(k, true);
                    }
                }
                Event::KeyUp { scancode: Some(scancode), keymod, .. } => {
                    insert_mod_keys(&mut key_changes, keymod);

                    if let Ok(k) = scancode.try_into() {
                        if relative_mouse_mode {
                            input_manager.set_key_up(scancode);
                        }
                        key_changes.insert(k, false);
                    }
                }
                Event::MouseMotion { xrel, yrel, .. } => {
                    if relative_mouse_mode {
                        input_manager.add_mouse_movement((xrel, yrel));
                    }
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left && !relative_mouse_mode && !imgui_context.want_capture_mouse() {
                        relative_mouse_mode = true;
                        context.sdl().mouse().set_relative_mouse_mode(true);
                    }
                }
                Event::MouseWheel { x, y, .. } => {
                    if relative_mouse_mode { input_manager.add_scroll((x, y)); }
                }
                Event::Quit { .. } => break 'mainloop,
                Event::TextInput { text, .. } => text_input.push(text),
                _ => (),
            }
        }

        let mouse_state = sdl2::mouse::MouseState::new(&event_pump);
        let mouse_pos: (i16, i16) = (mouse_state.x() as _, mouse_state.y() as _);
        if relative_mouse_mode {
            input_manager.set_mouse_position((mouse_state.x(), mouse_state.y()));
        }

        let (w, h) = context.window().drawable_size();
        #[cfg(feature = "imgui")]
        if relative_mouse_mode {
            imgui_context.prepare_unfocused([w as _, h as _], time.duration());
        } else {
            imgui_context.prepare(
                [w as _, h as _],
                Some([mouse_pos.0.into(), mouse_pos.1.into()]),
                Some([mouse_state.left(), mouse_state.right()]),
                &key_changes,
                &text_input,
                time.duration(),
            );
        }

        application.tick(&time, &input_manager);

        #[cfg(feature = "imgui")]
        imgui_context.render(|ui| application.gui(ui));

        context.window().gl_swap_window();
    }

    Ok(())
}

fn insert_mod_keys(key_changes: &mut HashMap<Key, bool>, keymod: Mod) {
    key_changes.insert(Key::MOD_CONTROL, keymod.contains(Mod::LCTRLMOD));
    key_changes.insert(Key::MOD_SHIFT, keymod.contains(Mod::LSHIFTMOD));
    key_changes.insert(Key::MOD_ALT, keymod.contains(Mod::LALTMOD));
}
