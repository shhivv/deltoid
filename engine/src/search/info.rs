#[derive(Clone)]
pub struct SearchInfo {
    pub nodes: u128,
}

impl SearchInfo {
    pub fn new() -> Self {
        Self { nodes: 0 }
    }
}
