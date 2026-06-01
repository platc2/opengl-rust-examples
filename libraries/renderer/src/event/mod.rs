pub use domain_event::DomainEvent;
pub use event_handler::EventHandler;
pub use sdl2_event_handlers::Sdl2EventHandlers;

use monoid::Monoid;

pub fn handle_events<Event, Result, Handler: EventHandler<Event, Result>>(
    handler: &mut Handler,
    events: Vec<Event>,
) -> Vec<Result> {
    events
        .into_iter()
        .map(|event| handler.handle_event(event))
        .collect()
}

pub fn handle_events_combine<Event, Result: Monoid, Handler: EventHandler<Event, Result>>(
    handler: &mut Handler,
    events: Vec<Event>,
) -> Result {
    handle_events(handler, events)
        .into_iter()
        .fold(Result::empty(), |acc, event| acc.combine(&event))
}

pub(crate) mod closure_event_handler;
mod domain_event;
mod event_handler;
mod monoid;
mod sdl2_event_handlers;
