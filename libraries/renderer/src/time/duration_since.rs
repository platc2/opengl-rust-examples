use std::time::{Duration, Instant, SystemTime};

pub trait DurationSince {
    fn duration_since(&self, earlier: Self) -> Option<Duration>;
}

impl DurationSince for Instant {
    fn duration_since(&self, earlier: Self) -> Option<Duration> {
        self.checked_duration_since(earlier)
    }
}

impl DurationSince for SystemTime {
    fn duration_since(&self, earlier: Self) -> Option<Duration> {
        self.duration_since(earlier)
            .ok()
    }
}
