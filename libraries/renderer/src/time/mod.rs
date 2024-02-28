use core::time::Duration;

pub use duration_since::*;
pub use now::*;

mod now;
mod duration_since;

#[derive(Debug)]
pub struct Time<T> {
    start: T,
    last: T,
    duration: Duration,
}

impl<T: Now + Copy> Default for Time<T> {
    fn default() -> Self {
        let now = T::now();
        Self { start: now, last: now, duration: Duration::default() }
    }
}

impl<T: Now + DurationSince + Copy> Time<T> {
    /// # Panics
    /// - if subsequent invocations of `T::now()` aren't considered "later" than earlier
    pub fn update(&mut self) {
        let end = T::now();
        self.duration = end.duration_since(self.last)
            .expect("Has time been running backwards?");
        self.last = end;
    }

    /// # Panics
    /// - if subsequent invocations of `T::now()` aren't considered "later" than earlier
    pub fn duration_since_start(&self) -> Duration {
        let end = T::now();
        end.duration_since(self.start)
            .expect("Has time been running backwards?")
    }

    pub const fn duration(&self) -> Duration { self.duration }

    #[must_use]
    pub fn fps(&self) -> f32 { 1. / self.duration.as_secs_f32() }
}
