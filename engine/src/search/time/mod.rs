pub mod timeman;

use std::time::Instant;

#[derive(Clone)]
pub struct Timer {
    pub start: Option<Instant>,
    pub stop_time: Option<u32>,
}

impl Timer {
    #[must_use]
    pub fn new(stop_time: Option<u32>) -> Self {
        Self {
            start: Some(Instant::now()),
            stop_time,
        }
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self {
            start: None,
            stop_time: None,
        }
    }
}
