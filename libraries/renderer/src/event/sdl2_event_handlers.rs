use super::EventHandler;
use super::closure_event_handler::ClosureEventHandler;
use sdl2::event::Event;
use sdl2::keyboard::{Mod, Scancode};
use sdl2::mouse::MouseButton;

pub struct Sdl2EventHandlers {
    handlers: Vec<Box<dyn EventHandler<Event, ()>>>,
}

impl EventHandler<Event, ()> for Sdl2EventHandlers {
    fn handle_event(&mut self, event: &Event) {
        for handler in &mut self.handlers {
            handler.handle_event(event);
        }
    }
}

struct MouseMotionEvent {
    x: i32,
    y: i32,
    delta_x: i32,
    delta_y: i32,
}

impl Sdl2EventHandlers {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Box<dyn EventHandler<Event, ()>>) {
        self.handlers.push(handler);
    }

    pub fn add_closure_handler<F: FnMut(&Event) + 'static>(&mut self, closure: F) {
        let closure_handler = ClosureEventHandler::new(Box::new(closure));
        self.add_handler(Box::new(closure_handler));
    }

    pub fn add_quit_handler<F: FnMut() + 'static>(&mut self, mut closure: F) {
        self.add_closure_handler(move |event| {
            if let &Event::Quit { .. } = event {
                closure();
            }
        });
    }

    pub fn add_all_keydown_handler<F: FnMut(Scancode, Mod) + 'static>(&mut self, mut closure: F) {
        self.add_closure_handler(move |event| {
            if let &Event::KeyDown {
                scancode: Some(scancode),
                keymod,
                ..
            } = event
            {
                closure(scancode, keymod);
            }
        });
    }

    pub fn add_all_keyup_handler<F: FnMut(Scancode, Mod) + 'static>(&mut self, mut closure: F) {
        self.add_closure_handler(move |event| {
            if let &Event::KeyUp {
                scancode: Some(scancode),
                keymod,
                ..
            } = event
            {
                closure(scancode, keymod);
            }
        });
    }

    pub fn add_keydown_handler<F: FnMut(Mod) + 'static>(&mut self, scancode: Scancode, mut closure: F) {
        self.add_all_keydown_handler(move |sc, keymod| {
            if sc == scancode {
                closure(keymod);
            }
        });
    }

    pub fn add_keyup_handler<F: FnMut(Mod) + 'static>(&mut self, scancode: Scancode, mut closure: F) {
        self.add_all_keyup_handler(move |sc, keymod| {
            if sc == scancode {
                closure(keymod);
            }
        });
    }

    pub fn add_all_mouse_button_down_handler<F: FnMut(MouseButton) + 'static>(
        &mut self,
        mut closure: F,
    ) {
        self.add_closure_handler(move |event| {
            if let &Event::MouseButtonDown { mouse_btn, .. } = event {
                closure(mouse_btn);
            }
        });
    }

    pub fn add_all_mouse_button_up_handler<F: FnMut(MouseButton) + 'static>(&mut self, mut closure: F) {
        self.add_closure_handler(move |event| {
            if let &Event::MouseButtonUp { mouse_btn, .. } = event {
                closure(mouse_btn);
            }
        });
    }

    pub fn add_mouse_button_down_handler<F: FnMut() + 'static>(
        &mut self,
        mouse_btn: MouseButton,
        mut closure: F,
    ) {
        self.add_all_mouse_button_down_handler(move |mb| {
            if mb == mouse_btn {
                closure();
            }
        });
    }

    pub fn add_mouse_button_up_handler<F: FnMut() + 'static>(
        &mut self,
        mouse_btn: MouseButton,
        mut closure: F,
    ) {
        self.add_all_mouse_button_up_handler(move |mb| {
            if mb == mouse_btn {
                closure();
            }
        });
    }

    pub fn add_mouse_motion_handler<F: FnMut(MouseMotionEvent) + 'static>(&mut self, mut closure: F) {
        self.add_closure_handler(move |event| {
            if let &Event::MouseMotion {
                xrel, yrel, x, y, ..
            } = event
            {
                closure(MouseMotionEvent {
                    x,
                    y,
                    delta_x: xrel,
                    delta_y: yrel,
                });
            }
        });
    }

    pub fn add_text_input_handler<F: FnMut(String) + 'static>(&mut self, mut closure: F) {
        self.add_closure_handler(move |event| {
            if let Event::TextInput { text, .. } = event {
                closure(text.to_owned());
            }
        });
    }
}
