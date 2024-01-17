pub struct Time {
    start: std::time::Instant,
    last: std::time::Instant,
    delta_time: std::time::Duration,
}

impl Time {
    pub fn new() -> Self {
        let now = Self::now();
        Self { start: now, last: now, delta_time: std::time::Duration::default() }
    }

    pub fn update(&mut self) {
        let end = Self::now();
        self.delta_time = end - self.last;
        self.last = end;
    }

    pub fn get_duration(&self) -> std::time::Duration {
        let end = Self::now();
        end - self.start
    }

    pub fn delta_time(&self) -> std::time::Duration { self.delta_time }

    pub fn fps(&self) -> f32 { 1. / self.delta_time.as_secs_f32() }

    fn now() -> std::time::Instant { std::time::Instant::now() }
}
