use crate::event::{handle_events, handle_events_combine, DomainEvent, EventHandler};
use crate::imgui_impl::Imgui;
use crate::input::{InputManager, Key, SdlInputManager};
use crate::renderer_context::RendererContext;
use crate::time::Time;
use anyhow::Result;
use imgui::Ui;
pub(crate) use sdl2::mouse::MouseButton;
use std::cell::RefCell;
use std::rc::Rc;

pub trait View {
    fn name(&self) -> &str;

    fn show(&mut self, ui: &Ui);
}

pub trait Application {
    #[allow(unused)]
    fn init(&mut self, context: &mut RendererContext) {}

    #[allow(unused)]
    fn tick(&mut self, time: &Time<std::time::Instant>, input_manager: &dyn InputManager) {}

    fn gui(&mut self, #[allow(unused)] ui: &Ui) {}

    fn views(&mut self) -> &mut [Box<dyn View>] {
        &mut []
    }

    fn quit(&self) -> bool {
        false
    }
}

pub trait App {
    fn new() -> anyhow::Result<Self>
    where
        Self: std::marker::Sized;
}

pub(crate) struct AppContext {
    pub time: Time<std::time::Instant>,
    pub input_manager: Rc<RefCell<SdlInputManager>>,
    application_state: ApplicationState,
    quit_requested: bool,
    pub view_states: std::collections::HashMap<String, bool>,
}

impl AppContext {
    pub fn new<T: Application>(application: &mut T) -> Self {
        let mut view_states = std::collections::HashMap::new();
        for view in application.views() {
            view_states.insert(view.name().to_owned(), false);
        }

        Self {
            time: Time::default(),
            input_manager: Rc::new(RefCell::new(SdlInputManager::default())),
            application_state: ApplicationState::Playing,
            quit_requested: false,
            view_states,
        }
    }

    pub const fn set_application_state(&mut self, state: ApplicationState) {
        self.application_state = state;
    }

    #[must_use]
    pub const fn application_state(&self) -> ApplicationState {
        self.application_state
    }

    pub const fn reset_quit(&mut self) {
        self.quit_requested = false;
    }

    pub const fn request_quit(&mut self) {
        self.quit_requested = true;
    }

    #[must_use]
    pub const fn is_quit_requested(&self) -> bool {
        self.quit_requested
    }

    pub fn update(&mut self) {
        self.time.update();
        if self.application_state == ApplicationState::Playing {
            self.input_manager.borrow_mut().update();
        }
    }
}

struct DomainEventPreProcessor {
    app_context: Rc<RefCell<AppContext>>,
}

impl DomainEventPreProcessor {
    pub const fn new(app_context: Rc<RefCell<AppContext>>) -> Self {
        Self { app_context }
    }
}

impl EventHandler<DomainEvent, DomainEvent> for DomainEventPreProcessor {
    fn handle_event(&mut self, event: DomainEvent) -> DomainEvent {
        use ApplicationState::{Menu, Playing};
        use DomainEvent::{KeyUp, MouseButtonUp, QuitRequested, StateSwitch};
        let app_state = self.app_context.borrow().application_state;
        let in_menu = app_state == Menu;
        match event {
            KeyUp(Key::ESCAPE) if in_menu => QuitRequested,
            KeyUp(Key::ESCAPE) => StateSwitch(Menu),
            MouseButtonUp(MouseButton::Left) if in_menu => StateSwitch(Playing),
            _ => event,
        }
    }
}

struct DomainEventHandler {
    app_context: Rc<RefCell<AppContext>>,
    renderer_context: Rc<RefCell<RendererContext>>,
}

impl DomainEventHandler {
    pub const fn new(
        app_context: Rc<RefCell<AppContext>>,
        renderer_context: Rc<RefCell<RendererContext>>,
    ) -> Self {
        Self {
            app_context,
            renderer_context,
        }
    }
}

impl EventHandler<DomainEvent, ()> for DomainEventHandler {
    fn handle_event(&mut self, event: DomainEvent) {
        use ApplicationState::Playing;
        let app_state = self.app_context.borrow().application_state;
        let playing = app_state == Playing;
        match event {
            DomainEvent::QuitRequested => self.app_context.borrow_mut().request_quit(),
            DomainEvent::KeyDown(key) if playing => self
                .app_context
                .borrow_mut()
                .input_manager
                .borrow_mut()
                .set_key_down(key.sdl),
            DomainEvent::KeyUp(key) if playing => self
                .app_context
                .borrow_mut()
                .input_manager
                .borrow_mut()
                .set_key_up(key.sdl),
            DomainEvent::MouseMotion {
                x,
                y,
                delta_x,
                delta_y,
            } if playing => {
                self.app_context
                    .borrow_mut()
                    .input_manager
                    .borrow_mut()
                    .set_mouse_position((x, y));
                self.app_context
                    .borrow_mut()
                    .input_manager
                    .borrow_mut()
                    .add_mouse_movement((delta_x, delta_y));
            }
            DomainEvent::MouseWheel { x, y } if playing => {
                self.app_context
                    .borrow_mut()
                    .input_manager
                    .borrow_mut()
                    .add_scroll((x, y));
            }
            DomainEvent::StateSwitch(state) => {
                self.app_context.borrow_mut().set_application_state(state);
                self.renderer_context
                    .borrow_mut()
                    .set_relative_mouse_mode(matches!(state, Playing));
            }
            _ => {}
        }
    }
}

pub fn start<T: App + Application>(renderer_context: RendererContext) -> Result<()> {
    let mut application = T::new()?;
    let context = Rc::new(RefCell::new(renderer_context));

    application.init(&mut context.borrow_mut());

    let mut quit = false;
    let app_ctx = Rc::new(RefCell::new(AppContext::new(&mut application)));
    let imgui_context = Rc::new(RefCell::new(Imgui::init()));
    let mut domain_event_preprocessor = DomainEventPreProcessor::new(app_ctx.clone());
    let mut domain_event_handler = DomainEventHandler::new(app_ctx.clone(), context.clone());

    while !application.quit() && !quit {
        app_ctx.borrow_mut().update();

        imgui_context.borrow_mut().update(
            app_ctx.borrow().time.duration(),
            context.borrow().window_dimension(),
        );

        let domain_events = context.borrow_mut().events();

        let remaining = match app_ctx.borrow().application_state {
            ApplicationState::Playing => domain_events.clone(),
            ApplicationState::Menu => {
                handle_events_combine(&mut *imgui_context.borrow_mut(), domain_events)
            }
        };

        let events = handle_events(&mut domain_event_preprocessor, remaining);
        handle_events(&mut domain_event_handler, events);

        {
            let app_ctx = app_ctx.borrow();
            let input_manager = app_ctx.input_manager.borrow();
            application.tick(&app_ctx.time, &*input_manager);
        }

        imgui_context.borrow_mut().render(|ui| {
            application.gui(ui);

            if app_ctx.borrow().is_quit_requested() {
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
                        app_ctx.borrow_mut().reset_quit();
                        ui.close_current_popup();
                    }
                });

            ui.main_menu_bar(|| {
                ui.menu("File", || {
                    if ui.menu_item("Exit") {
                        app_ctx.borrow_mut().request_quit();
                    }
                });

                {
                    let view_states = &mut app_ctx.borrow_mut().view_states;
                    ui.menu_with_enabled("Views", !view_states.is_empty(), || {
                        for (view_name, view_state) in view_states.iter_mut() {
                            if ui.menu_item_config(view_name).selected(*view_state).build() {
                                *view_state = !*view_state;
                            }
                        }
                    });
                }

                let status_text = format!("State: {:?}", app_ctx.borrow().application_state());
                let text_size = ui.calc_text_size(&status_text);
                let available = ui.content_region_avail()[0];
                ui.same_line();
                ui.dummy([available - text_size[0], 0.0]);
                ui.same_line();
                ui.text_disabled(status_text);
            });

            for view in application.views() {
                if *app_ctx
                    .borrow_mut()
                    .view_states
                    .entry(view.name().to_owned())
                    .or_insert(false)
                {
                    let name = view.name().to_owned();
                    ui.window(name)
                        .save_settings(false)
                        .always_auto_resize(true)
                        .opened(
                            app_ctx
                                .borrow_mut()
                                .view_states
                                .get_mut(view.name())
                                .unwrap(),
                        )
                        .build(|| view.show(ui));
                }
            }
        });

        context.borrow_mut().swap_buffers();
    }

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ApplicationState {
    Menu,
    Playing,
}
