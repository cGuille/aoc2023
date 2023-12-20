use super::{ModuleName, ModuleType, Pulse};

#[derive(Debug)]
pub struct Broadcaster;

impl Broadcaster {
    pub fn new() -> Self {
        Self
    }
}

impl ModuleType for Broadcaster {
    fn handle_pulse(&mut self, pulse: Pulse, _from: &ModuleName) -> Option<Pulse> {
        Some(pulse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broadcaster() {
        let sender = ModuleName::from("sender");
        let inputs = vec![sender.clone()];

        let mut sut = Broadcaster::new();

        sut.connect(&inputs);

        assert_eq!(sut.handle_pulse(Pulse::Low, &sender), Some(Pulse::Low));
        assert_eq!(sut.handle_pulse(Pulse::High, &sender), Some(Pulse::High));
    }
}
