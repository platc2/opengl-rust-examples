use std::time::{Duration, Instant, SystemTime};

#[derive(Debug)]
pub struct Time<T> {
    start: T,
    last: T,
    duration: Duration,
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

impl<T: TimeNow + Copy> Time<T> {
    #[must_use]
    pub fn new() -> Self {
        let now = T::time_now();
        Self { start: now, last: now, duration: Duration::default() }
    }

    pub fn update(&mut self) {
        let end = T::time_now();
        self.duration = end.duration_since(self.last)
            .expect("Has time been running backwards?");
        self.last = end;
    }

    #[must_use]
    pub fn get_duration(&self) -> Duration {
        let end = T::time_now();
        end.duration_since(self.start)
            .expect("Has time been running backwards?")
    }

    #[must_use]
    pub fn duration(&self) -> Duration { self.duration }

    #[must_use]
    pub fn fps(&self) -> f32 { 1. / self.duration.as_secs_f32() }
}
