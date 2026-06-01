pub trait EventHandler<E, R> {
    fn handle_event(&mut self, event: E) -> R;
}
