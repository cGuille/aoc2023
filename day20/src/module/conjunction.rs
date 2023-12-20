use std::collections::HashMap;

use super::{ModuleName, ModuleType, Pulse};

#[derive(Debug)]
pub struct Conjunction {
    last_received: HashMap<ModuleName, Pulse>,
}

impl Conjunction {
    pub fn new() -> Self {
        Self {
            last_received: HashMap::new(),
        }
    }
}

fn init_input(input: &ModuleName) -> (ModuleName, Pulse) {
    (input.clone(), Pulse::Low)
}

impl ModuleType for Conjunction {
    fn connect(&mut self, inputs: &Vec<ModuleName>) {
        self.last_received.extend(inputs.iter().map(init_input));
    }

    fn handle_pulse(&mut self, pulse: Pulse, from: &ModuleName) -> Option<Pulse> {
        self.last_received.insert(from.clone(), pulse);

        let all_high = self.last_received.values().all(Pulse::is_high);

        Some(if all_high { Pulse::Low } else { Pulse::High })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conjunction() {
        let sender1 = ModuleName::from("sender1");
        let sender2 = ModuleName::from("sender2");
        let inputs = vec![sender1.clone(), sender2.clone()];

        let mut sut = Conjunction::new();

        sut.connect(&inputs);

        assert_eq!(sut.handle_pulse(Pulse::High, &sender1), Some(Pulse::High));
        assert_eq!(sut.handle_pulse(Pulse::Low, &sender1), Some(Pulse::High));
        assert_eq!(sut.handle_pulse(Pulse::High, &sender1), Some(Pulse::High));

        assert_eq!(sut.handle_pulse(Pulse::High, &sender2), Some(Pulse::Low));

        assert_eq!(sut.handle_pulse(Pulse::Low, &sender1), Some(Pulse::High));
        assert_eq!(sut.handle_pulse(Pulse::High, &sender1), Some(Pulse::Low));
    }
}
