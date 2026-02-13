use crate::event;
#[cfg(feature = "imgui")]
use crate::imgui_impl::Imgui;
use crate::input_manager::{InputManager, Key, SdlInputManager};
use crate::renderer_context::RendererContext;
use crate::time::Time;
use anyhow::{anyhow, Result};
use event::EventHandler;
#[cfg(feature = "imgui")]
use imgui::Ui;
use sdl2::keyboard::{Mod, Scancode};
use sdl2::mouse::MouseButton;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[cfg(feature = "imgui")]
pub trait View {
    fn name(&self) -> &str;

    fn show(&mut self, ui: &Ui);
}

pub trait Application {
    #[allow(unused)]
    fn init(&mut self, context: &mut RendererContext) {}

    #[allow(unused)]
    fn tick(&mut self, time: &Time<std::time::Instant>, input_manager: Ref<dyn InputManager>) {}

    #[cfg(feature = "imgui")]
    fn gui(&mut self, #[allow(unused)] ui: &Ui) {}

    fn views(&mut self) -> &mut [Box<dyn View>] {
        &mut []
    }

    fn quit(&self) -> bool {
        false
    }
}

trait App {
    fn new() -> Self;
}

pub fn start<T: App>() -> Result<()> {
    let _ = T::new();

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ApplicationState {
    Menu,
    Playing,
}

enum DomainEvent {
    QuitRequested,
    KeyDown(Scancode, Mod),
    KeyUp(Scancode, Mod),
    MouseMotion { x: i16, y: i16, delta_x: i16, delta_y: i16 },
    MouseButtonDown(MouseButton),
    MouseButtonUp(MouseButton),
    TextInput(String)
}

// FIXME
#[allow(clippy::too_many_lines)]
pub fn main_loop<T: Application>(context: RendererContext, mut application: T) -> Result<()> {
    let mut time: Time<std::time::Instant> = Time::default();
    let mut event_pump = context.sdl().event_pump().map_err(|e| anyhow!(e))?;
    #[cfg(feature = "imgui")]
    let mut quit = false;
    let imgui_context = Rc::new(RefCell::new(Imgui::init()));
    let input_manager = Rc::new(RefCell::new(SdlInputManager::default()));

    // Initialise application state
    let application_state = Rc::new(RefCell::new(ApplicationState::Playing));
    let quit_requested = Rc::new(RefCell::new(false));

    // Play state handler
    let mut play_event_handler = event::Sdl2EventHandlers::new();
    play_event_handler.add_quit_handler({
        let quit_requested = quit_requested.clone();
        move || *quit_requested.borrow_mut() = true
    });
    play_event_handler.add_keydown_handler(Scancode::Escape, {
        let application_state = application_state.clone();
        move |_| *application_state.borrow_mut() = ApplicationState::Menu
    });
    play_event_handler.add_all_keydown_handler({
        let input_manager = input_manager.clone();
        move |scancode, keymod| {
            input_manager.borrow_mut().set_key_down(scancode);
        }
    });
    play_event_handler.add_all_keyup_handler({
        let input_manager = input_manager.clone();
        move |scancode, keymod| {
            input_manager.borrow_mut().set_key_up(scancode);
        }
    });
    play_event_handler.add_mouse_motion_handler({
        let input_manager = input_manager.clone();
        move |evt| {
            input_manager
                .borrow_mut()
                .set_mouse_position((evt.x, evt.y));
            input_manager
                .borrow_mut()
                .add_mouse_movement((evt.delta_x, evt.delta_y));
        }
    });

    // Menu state handler
    let menu_key_changes = Rc::new(RefCell::new(std::collections::HashMap::<Key, bool>::new()));
    let menu_text_input = Rc::new(RefCell::new(Vec::new()));
    let mut menu_event_handler = event::Sdl2EventHandlers::new();
    menu_event_handler.add_quit_handler({
        let quit_requested = quit_requested.clone();
        move || *quit_requested.borrow_mut() = true
    });
    menu_event_handler.add_keydown_handler(Scancode::Escape, {
        let quit_requested = quit_requested.clone();
        let imgui_context = imgui_context.clone();
        move |_| {
            if (!imgui_context.borrow().want_capture_keyboard()) {
                *quit_requested.borrow_mut() = true;
            }
        }
    });
    menu_event_handler.add_mouse_button_down_handler(MouseButton::Left, {
        let application_state = application_state.clone();
        let imgui_context = imgui_context.clone();
        move || {
            if !imgui_context.borrow().want_capture_mouse() {
                *application_state.borrow_mut() = ApplicationState::Playing;
            }
        }
    });
    menu_event_handler.add_all_keydown_handler({
        let menu_key_changes = menu_key_changes.clone();
        move |scancode, keymod| {
            if let Ok(k) = scancode.try_into() {
                menu_key_changes.borrow_mut().insert(k, true);
                insert_mod_keys(&mut menu_key_changes.borrow_mut(), keymod);
            }
        }
    });
    menu_event_handler.add_all_keyup_handler({
        let menu_key_changes = menu_key_changes.clone();
        move |scancode, keymod| {
            if let Ok(k) = scancode.try_into() {
                menu_key_changes.borrow_mut().insert(k, false);
                insert_mod_keys(&mut menu_key_changes.borrow_mut(), keymod);
            }
        }
    });
    menu_event_handler.add_text_input_handler({
        let menu_text_input = menu_text_input.clone();
        move |text| menu_text_input.borrow_mut().push(text)
    });

    let mut view_states = std::collections::HashMap::new();
    for view in application.views() {
        view_states.insert(view.name().to_owned(), false);
    }

    while !application.quit() && !quit {
        time.update();

        let application_state = *application_state.borrow();
        if application_state == ApplicationState::Playing {
            input_manager.borrow_mut().update();
        }

        imgui_context.borrow().want_capture_mouse();

        menu_key_changes.borrow_mut().clear();
        menu_text_input.borrow_mut().clear();

        for event in event_pump.poll_iter() {
            match application_state {
                ApplicationState::Playing => play_event_handler.handle_event(&event),
                ApplicationState::Menu => menu_event_handler.handle_event(&event),
            }
        }

        match application_state {
            ApplicationState::Playing => context.sdl().mouse().set_relative_mouse_mode(true),
            ApplicationState::Menu => context.sdl().mouse().set_relative_mouse_mode(false),
        }

        let mouse_state = sdl2::mouse::MouseState::new(&event_pump);
        let mouse_pos: (i16, i16) = (mouse_state.x() as _, mouse_state.y() as _);
        if application_state == ApplicationState::Playing {
            input_manager
                .borrow_mut()
                .set_mouse_position((mouse_state.x(), mouse_state.y()));
        }

        let (w, h) = context.window().drawable_size();
        #[cfg(feature = "imgui")]
        match application_state {
            ApplicationState::Playing => imgui_context
                .borrow_mut()
                .prepare_unfocused([w as _, h as _], time.duration()),
            ApplicationState::Menu => imgui_context.borrow_mut().prepare(
                [w as _, h as _],
                Some([mouse_pos.0.into(), mouse_pos.1.into()]),
                Some([mouse_state.left(), mouse_state.right()]),
                &menu_key_changes.borrow(),
                &menu_text_input.borrow(),
                time.duration(),
            ),
        }

        application.tick(&time, input_manager.borrow());

        #[cfg(feature = "imgui")]
        imgui_context.borrow_mut().render(|ui| {
            application.gui(ui);

            if *quit_requested.borrow() {
                ui.open_popup("Quit Application");
            }

            ui.modal_popup_config("Quit Application")
                .always_auto_resize(true)
                .movable(false)
                .build(|| {
                    ui.text("Are you sure you want to close the application?");
                    if ui.button("Yes") {
                        quit = true;
                        ui.close_current_popup();
                    }

                    ui.same_line();

                    if ui.button("No") {
                        *quit_requested.borrow_mut() = false;
                        ui.close_current_popup();
                    }
                });

            ui.main_menu_bar(|| {
                ui.menu("File", || {
                    if ui.menu_item("Exit") {
                        *quit_requested.borrow_mut() = true;
                    }
                });

                ui.menu("Views", || {
                    for (view_name, view_state) in view_states.iter_mut() {
                        if ui.menu_item_config(view_name).selected(*view_state).build() {
                            *view_state = !*view_state;
                        }
                    }
                });

                let status_text = format!("State: {application_state:?}");
                let text_size = ui.calc_text_size(&status_text);
                let available = ui.content_region_avail()[0];
                ui.same_line();
                ui.dummy([available - text_size[0], 0.0]);
                ui.same_line();
                ui.text_disabled(status_text);
            });

            for view in application.views() {
                if *view_states.entry(view.name().to_owned()).or_insert(false) {
                    view.show(ui);
                }
            }
        });

        context.window().gl_swap_window();
    }

    Ok(())
}

fn insert_mod_keys(key_changes: &mut std::collections::HashMap<Key, bool>, keymod: Mod) {
    key_changes.insert(
        Key::MOD_CONTROL,
        keymod.contains(Mod::LCTRLMOD) || keymod.contains(Mod::RCTRLMOD),
    );
    key_changes.insert(
        Key::MOD_SHIFT,
        keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD),
    );
    key_changes.insert(
        Key::MOD_ALT,
        keymod.contains(Mod::LALTMOD) || keymod.contains(Mod::RALTMOD),
    );
}
