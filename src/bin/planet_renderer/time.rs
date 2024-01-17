pub struct Time {
    start: std::time::Instant,
    last: std::time::Instant,
    duration: std::time::Duration,
}

impl Time {
    #[must_use]
    pub fn new() -> Self {
        let now = Self::now();
        Self { start: now, last: now, duration: std::time::Duration::default() }
    }

    pub fn update(&mut self) {
        let end = Self::now();
        self.duration = end - self.last;
        self.last = end;
    }

    #[must_use]
    pub fn get_duration(&self) -> std::time::Duration {
        let end = Self::now();
        end - self.start
    }

    #[must_use]
    pub fn duration(&self) -> std::time::Duration { self.duration }

    #[must_use]
    pub fn fps(&self) -> f32 { 1. / self.duration.as_secs_f32() }

    #[must_use]
    fn now() -> std::time::Instant { std::time::Instant::now() }
}
