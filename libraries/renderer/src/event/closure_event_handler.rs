use super::EventHandler;

pub struct ClosureEventHandler<E, R, F: FnMut(&E) -> R> {
    closure: F,
    _phantom_event: std::marker::PhantomData<E>,
    _phantom_result: std::marker::PhantomData<R>,
}

impl<E, R, F: FnMut(&E) -> R> EventHandler<E, R> for ClosureEventHandler<E, R, F> {
    fn handle_event(&mut self, event: E) -> R {
        (self.closure)(&event)
    }
}

impl<E, R, F: FnMut(&E) -> R> ClosureEventHandler<E, R, F> {
    pub fn new(closure: F) -> Self {
        Self {
            closure,
            _phantom_event: Default::default(),
            _phantom_result: Default::default(),
        }
    }
}