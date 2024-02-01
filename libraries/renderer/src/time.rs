use core::time::Duration;
use std::time::{Instant, SystemTime};

#[derive(Debug)]
pub struct Time<T> {
    start: T,
    last: T,
    duration: Duration,
}

pub trait Now {
    fn now() -> Self;
}

impl Now for std::time::Instant {
    fn now() -> Self {
        Self::now()
    }
}

pub trait TimeNow {
    fn time_now() -> Self;

    fn duration_since(&self, earlier: Self) -> Option<Duration>;
}

impl TimeNow for Instant {
    fn time_now() -> Self { Self::now() }

    fn duration_since(&self, earlier: Self) -> Option<Duration> {
        self.checked_duration_since(earlier)
    }
}

impl TimeNow for SystemTime {
    fn time_now() -> Self { Self::now() }

    fn duration_since(&self, earlier: Self) -> Option<Duration> {
        self.duration_since(earlier)
            .ok()
    }
}

impl<T: TimeNow + Copy> Default for Time<T> {
    fn default() -> Self {
        let now = T::time_now();
        Self { start: now, last: now, duration: Duration::default() }
    }
}

impl<T: TimeNow + Copy> Time<T> {
    pub fn update(&mut self) {
        let end = T::time_now();
        self.duration = end.duration_since(self.last)
            .expect("Has time been running backwards?");
        self.last = end;
    }

    pub fn duration_since_start(&self) -> Duration {
        let end = T::time_now();
        end.duration_since(self.start)
            .expect("Has time been running backwards?")
    }

    pub const fn duration(&self) -> Duration { self.duration }

    #[must_use]
    pub fn fps(&self) -> f32 { 1. / self.duration.as_secs_f32() }
}
