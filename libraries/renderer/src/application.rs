use std::rc::Rc;
use anyhow::{anyhow, Result};
#[cfg(feature = "imgui")]
use imgui::Ui;
use sdl2::event::Event;
use sdl2::keyboard::{Mod, Scancode};
use sdl2::mouse::MouseButton;

#[cfg(feature = "imgui")]
use crate::imgui_impl::Imgui;
use crate::input_manager::{InputManager, Key, SdlInputManager};
use crate::renderer_context::RendererContext;
use crate::time::Time;

#[cfg(feature = "imgui")]
pub trait View {
    fn name(&self) -> &str;

    fn show(&mut self, ui: &Ui);
}

pub trait Application {
    #[allow(unused)]
    fn init(&mut self, context: &mut RendererContext) {}

    #[allow(unused)]
    fn tick(&mut self, time: &Time<std::time::Instant>, input_manager: &dyn InputManager) {}

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

trait EventHandler<E, R> {
    fn handle_event(&mut self, event: &E) -> R;
}

struct ClosureEventHandler<E, R, F: FnMut(&E) -> R> {
    closure: F,
    _phantom_event: std::marker::PhantomData<E>,
    _phantom_result: std::marker::PhantomData<R>,
}

impl<E, R, F: FnMut(&E) -> R> EventHandler<E, R> for ClosureEventHandler<E, R, F> {
    fn handle_event(&mut self, event: &E) -> R {
        (self.closure)(event)
    }
}

impl<E, R, F: FnMut(&E) -> R> ClosureEventHandler<E, R, F> {
    fn new(closure: F) -> Self {
        Self { closure, _phantom_event: Default::default(), _phantom_result: Default::default() }
    }
}

struct Sdl2EventHandler {
    handlers: Vec<Box<dyn EventHandler<Event, ()>>>,
}

impl EventHandler<Event, ()> for Sdl2EventHandler {
    fn handle_event(&mut self, event: &Event) {
        for handler in &mut self.handlers {
            handler.handle_event(event);
        }
    }
}

impl Sdl2EventHandler {
    fn new() -> Self {
        Self {
            handlers: Vec::new()
        }
    }

    fn add_handler(&mut self, handler: Box<dyn EventHandler<Event, ()>>) {
        self.handlers.push(handler);
    }

    fn add_closure_handler<F: FnMut(&Event) + 'static>(&mut self, closure: F) {
        let closure_handler = ClosureEventHandler::new(Box::new(closure));
        self.handlers.push(Box::new(closure_handler));
    }

    fn add_quit_handler<F: FnMut() + 'static>(&mut self, mut closure: F) {
        self.add_closure_handler(move |event| {
            if let &Event::Quit { .. } = event {
                closure();
            }
        });
    }

    fn add_all_keydown_handler<F: FnMut(Scancode, Mod) + 'static>(&mut self, mut closure: F) {
        self.add_closure_handler(move |event| {
            if let &Event::KeyDown { scancode: Some(scancode), keymod, .. } = event {
                closure(scancode, keymod);
            }
        });
    }

    fn add_all_keyup_handler<F: FnMut(Scancode, Mod) + 'static>(&mut self, mut closure: F) {
        self.add_closure_handler(move |event| {
            if let &Event::KeyUp { scancode: Some(scancode), keymod, .. } = event {
                closure(scancode, keymod);
            }
        });
    }

    fn add_keydown_handler<F: FnMut(Mod) + 'static>(&mut self, scancode: Scancode, mut closure: F) {
        self.add_all_keydown_handler(move |sc, keymod| {
            if sc == scancode {
                closure(keymod);
            }
        });
    }

    fn add_keyup_handler<F: FnMut(Mod) + 'static>(&mut self, scancode: Scancode, mut closure: F) {
        self.add_all_keyup_handler(move |sc, keymod| {
            if sc == scancode {
                closure(keymod);
            }
        });
    }
}

enum ApplicationState {
    Menu,
    Playing
}

pub fn main_loop<T: Application>(context: RendererContext, mut application: T) -> Result<()> {
    let mut time: Time<std::time::Instant> = Time::default();
    let mut event_pump = context.sdl().event_pump().map_err(|e| anyhow!(e))?;
    #[cfg(feature = "imgui")]
    let mut imgui_context = Imgui::init();
    let mut input_manager = SdlInputManager::default();
    let mut relative_mouse_mode = false;

    // Initialise application state
    let mut application_state = ApplicationState::Playing;
    let quit_requested = Rc::new(std::cell::RefCell::new(false));

    let mut play_event_handler = Sdl2EventHandler::new();
    play_event_handler.add_quit_handler({
        let quit_requested = quit_requested.clone();
        move || *quit_requested.borrow_mut() = true
    });
    let mut menu_event_handler = Sdl2EventHandler::new();
    menu_event_handler.add_quit_handler({
        let quit_requested = quit_requested.clone();
        move || *quit_requested.borrow_mut() = true
    });

    let mut view_states = std::collections::HashMap::new();
    for view in application.views() {
        view_states.insert(view.name().to_owned(), false);
    }

    while !application.quit() && !*quit_requested.borrow() {
        time.update();
        if relative_mouse_mode {
            input_manager.update();
        }

        let mut key_changes = std::collections::HashMap::new();
        let mut text_input: Vec<String> = Vec::new();

        for event in event_pump.poll_iter() {

            match application_state {
                ApplicationState::Playing => play_event_handler.handle_event(&event),
                ApplicationState::Menu => menu_event_handler.handle_event(&event),
            }

            // No further processing needed

/*
            match event {
                Event::KeyDown {
                    scancode: Some(scancode),
                    keymod,
                    ..
                } => {
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
                Event::KeyUp {
                    scancode: Some(scancode),
                    keymod,
                    ..
                } => {
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
                    if mouse_btn == MouseButton::Left
                        && !relative_mouse_mode
                        && !imgui_context.want_capture_mouse()
                    {
                        relative_mouse_mode = true;
                        context.sdl().mouse().set_relative_mouse_mode(true);
                    }
                }
                Event::MouseWheel { x, y, .. } => {
                    if relative_mouse_mode {
                        input_manager.add_scroll((x, y));
                    }
                }
                Event::Quit { .. } => break 'mainloop,
                Event::TextInput { text, .. } => text_input.push(text),
                _ => (),
            }
*/
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
        imgui_context.render(|ui| {
            application.gui(ui);

            ui.main_menu_bar(|| {
                ui.menu("Views", || {
                    for (view_name, view_state) in view_states.iter_mut() {
                        if ui.menu_item_config(view_name).selected(*view_state).build() {
                            *view_state = !*view_state;
                        }
                    }
                });
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
    key_changes.insert(Key::MOD_CONTROL, keymod.contains(Mod::LCTRLMOD));
    key_changes.insert(Key::MOD_SHIFT, keymod.contains(Mod::LSHIFTMOD));
    key_changes.insert(Key::MOD_ALT, keymod.contains(Mod::LALTMOD));
}
