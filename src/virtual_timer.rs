#[derive(Debug)]
pub struct VirtualTimer {
    time_elapsed: u32,
}

impl VirtualTimer {
    pub fn new() -> Self {
        Self { time_elapsed: 0 }
    }

    pub fn absolute_time(&self) -> u32 {
        self.time_elapsed
    }

    pub fn add_time(&mut self, time: u32) {
        self.time_elapsed += time;
    }
}
