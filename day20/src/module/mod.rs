mod broadcaster;
mod conjunction;
mod flipflop;
mod output;

use std::fmt::Debug;

pub use broadcaster::Broadcaster;
pub use conjunction::Conjunction;
pub use flipflop::FlipFlop;
pub use output::Output;

use crate::SendPulseRequest;

pub type ModuleName = String;

#[derive(Debug)]
pub struct Module<T> {
    name: ModuleName,
    inputs: Vec<ModuleName>,
    outputs: Vec<ModuleName>,
    module_type: T,
}

pub trait ModuleType: Debug {
    fn connect(&mut self, _inputs: &Vec<ModuleName>) {}
    fn handle_pulse(&mut self, pulse: Pulse, from: &ModuleName) -> Option<Pulse>;
}

impl<M: ModuleType + ?Sized> ModuleType for Box<M> {
    fn connect(&mut self, inputs: &Vec<ModuleName>) {
        (**self).connect(inputs)
    }

    fn handle_pulse(&mut self, pulse: Pulse, from: &ModuleName) -> Option<Pulse> {
        (**self).handle_pulse(pulse, from)
    }
}

impl<T: ModuleType> Module<T> {
    pub fn new(
        name: ModuleName,
        module_type: T,
        inputs: Vec<ModuleName>,
        outputs: Vec<ModuleName>,
    ) -> Self {
        let mut module = Self {
            name,
            inputs,
            outputs,
            module_type,
        };

        module.module_type.connect(&module.inputs);

        module
    }

    pub fn send_pulse(&mut self, pulse: Pulse, from: &ModuleName) -> Option<SendPulseRequest> {
        self.module_type
            .handle_pulse(pulse, from)
            .map(|pulse_to_send| SendPulseRequest {
                origin: self.name.clone(),
                pulse: pulse_to_send,
                destinations: self.outputs.clone(),
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn is_high(&self) -> bool {
        match self {
            Self::High => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct FakeModType {
        fake_result: Option<Pulse>,
    }

    impl ModuleType for FakeModType {
        fn handle_pulse(&mut self, _pulse: Pulse, _from: &ModuleName) -> Option<Pulse> {
            self.fake_result
        }
    }

    #[test]
    fn test_module() {
        let sender = ModuleName::from("sender");
        let receiver1 = ModuleName::from("receiver1");
        let receiver2 = ModuleName::from("receiver2");
        let inputs = vec![sender.clone()];
        let outputs = vec![receiver1.clone(), receiver2.clone()];

        let fake_mod_type = Box::new(FakeModType { fake_result: None });
        let sut_name = ModuleName::from("sut");
        let mut sut = Module::new(sut_name.clone(), fake_mod_type, inputs, outputs);

        let reaction = sut.send_pulse(Pulse::High, &sender);
        assert!(reaction.is_none());

        sut.module_type.fake_result = Some(Pulse::High);

        let req = sut.send_pulse(Pulse::Low, &sender).unwrap();
        assert_eq!(req.origin, sut_name);
        assert_eq!(req.pulse, Pulse::High);
        assert_eq!(req.destinations.len(), 2);
        assert_eq!(req.destinations[0], receiver1);
        assert_eq!(req.destinations[1], receiver2);

        sut.module_type.fake_result = Some(Pulse::Low);

        let req = sut.send_pulse(Pulse::Low, &sender).unwrap();
        assert_eq!(req.origin, sut_name);
        assert_eq!(req.pulse, Pulse::Low);
        assert_eq!(req.destinations.len(), 2);
        assert_eq!(req.destinations[0], receiver1);
        assert_eq!(req.destinations[1], receiver2);
    }
}
