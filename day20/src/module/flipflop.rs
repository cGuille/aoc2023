use super::{ModuleName, ModuleType, Pulse};

#[derive(Debug, PartialEq)]
pub enum FlipFlop {
    On,
    Off,
}

use FlipFlop::*;

impl FlipFlop {
    pub fn new() -> Self {
        Off
    }

    fn switch(&mut self) {
        match self {
            On => *self = Off,
            Off => *self = On,
        };
    }
}

impl ModuleType for FlipFlop {
    fn handle_pulse(&mut self, pulse: Pulse, _from: &ModuleName) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.switch();

                Some(match self {
                    On => Pulse::High,
                    Off => Pulse::Low,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_flop() {
        let sender = ModuleName::from("sender");
        let inputs = vec![sender.clone()];

        let mut sut = FlipFlop::new();

        sut.connect(&inputs);

        assert_eq!(sut.handle_pulse(Pulse::High, &sender), None);
        assert_eq!(sut, Off);

        assert_eq!(sut.handle_pulse(Pulse::High, &sender), None);
        assert_eq!(sut, Off);

        assert_eq!(sut.handle_pulse(Pulse::Low, &sender), Some(Pulse::High));
        assert_eq!(sut, On);

        assert_eq!(sut.handle_pulse(Pulse::High, &sender), None);
        assert_eq!(sut, On);

        assert_eq!(sut.handle_pulse(Pulse::Low, &sender), Some(Pulse::Low));
        assert_eq!(sut, Off);
    }
}
