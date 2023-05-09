use super::time::Timer;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct SearchInfo {
    pub nodes: u128,
    pub stop: bool,
    pub timer: Timer,
}

impl SearchInfo {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: 0,
            stop: false,
            timer: Timer::empty(),
        }
    }
}
