use super::{ModuleName, ModuleType, Pulse};

#[derive(Debug)]
pub struct Output;

impl Output {
    pub fn new() -> Self {
        Self
    }
}

impl ModuleType for Output {
    fn handle_pulse(&mut self, _pulse: Pulse, _from: &ModuleName) -> Option<Pulse> {
        None
    }
}
