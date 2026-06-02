use super::closure_event_handler::ClosureEventHandler;
use super::EventHandler;
use super::Monoid;
use sdl2::event::Event;
use sdl2::keyboard::{Mod, Scancode};
use sdl2::mouse::MouseButton;

pub struct Sdl2EventHandlers<R: Monoid = ()> {
    handlers: Vec<Box<dyn EventHandler<Event, R>>>,
}

impl<R: Monoid> EventHandler<Event, R> for Sdl2EventHandlers<R> {
    fn handle_event(&mut self, event: Event) -> R {
        self.handlers.iter_mut().fold(R::empty(), |acc, handler| {
            let result = handler.handle_event(event.clone());
            acc.combine(&result)
        })
    }
}

pub struct MouseMotionEvent {
    pub x: i32,
    pub y: i32,
    pub delta_x: i32,
    pub delta_y: i32,
}

pub struct MouseWheelEvent {
    pub x: f32,
    pub y: f32,
}

impl<R: Monoid + 'static> Sdl2EventHandlers<R> {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Box<dyn EventHandler<Event, R>>) {
        self.handlers.push(handler);
    }

    pub fn add_closure_handler<F: FnMut(&Event) -> R + 'static>(&mut self, closure: F) {
        let closure_handler = ClosureEventHandler::new(Box::new(closure));
        self.add_handler(Box::new(closure_handler));
    }

    pub fn add_quit_handler<F: FnMut() -> R + 'static>(&mut self, mut closure: F) {
        self.add_closure_handler(move |event| {
            if let &Event::Quit { .. } = event {
                closure()
            } else {
                R::empty()
            }
        });
    }

    pub fn add_all_keydown_handler<F: FnMut(Scancode, Mod) -> R + 'static>(
        &mut self,
        mut closure: F,
    ) {
        self.add_closure_handler(move |event| {
            if let &Event::KeyDown {
                scancode: Some(scancode),
                keymod,
                ..
            } = event
            {
                closure(scancode, keymod)
            } else {
                R::empty()
            }
        });
    }

    pub fn add_all_keyup_handler<F: FnMut(Scancode, Mod) -> R + 'static>(
        &mut self,
        mut closure: F,
    ) {
        self.add_closure_handler(move |event| {
            if let &Event::KeyUp {
                scancode: Some(scancode),
                keymod,
                ..
            } = event
            {
                closure(scancode, keymod)
            } else {
                R::empty()
            }
        });
    }

    pub fn add_all_mouse_button_down_handler<F: FnMut(MouseButton) -> R + 'static>(
        &mut self,
        mut closure: F,
    ) {
        self.add_closure_handler(move |event| {
            if let &Event::MouseButtonDown { mouse_btn, .. } = event {
                closure(mouse_btn)
            } else {
                R::empty()
            }
        });
    }

    pub fn add_all_mouse_button_up_handler<F: FnMut(MouseButton) -> R + 'static>(
        &mut self,
        mut closure: F,
    ) {
        self.add_closure_handler(move |event| {
            if let &Event::MouseButtonUp { mouse_btn, .. } = event {
                closure(mouse_btn)
            } else {
                R::empty()
            }
        });
    }

    pub fn add_mouse_motion_handler<F: FnMut(MouseMotionEvent) -> R + 'static>(
        &mut self,
        mut closure: F,
    ) {
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
                })
            } else {
                R::empty()
            }
        });
    }

    pub fn add_mouse_wheel_handler<F: FnMut(MouseWheelEvent) -> R + 'static>(
        &mut self,
        mut closure: F,
    ) {
        self.add_closure_handler(move |event| {
            if let &Event::MouseWheel {
                precise_x,
                precise_y,
                ..
            } = event
            {
                closure(MouseWheelEvent {
                    x: precise_x,
                    y: precise_y,
                })
            } else {
                R::empty()
            }
        });
    }

    pub fn add_text_input_handler<F: FnMut(String) -> R + 'static>(&mut self, mut closure: F) {
        self.add_closure_handler(move |event| {
            if let Event::TextInput { text, .. } = event {
                closure(text.to_owned())
            } else {
                R::empty()
            }
        });
    }
}
