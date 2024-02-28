use std::time::{Instant, SystemTime};

pub trait Now {
    fn now() -> Self;
}

impl Now for Instant {
    fn now() -> Self {
        Self::now()
    }
}

impl Now for SystemTime {
    fn now() -> Self { Self::now() }
}
