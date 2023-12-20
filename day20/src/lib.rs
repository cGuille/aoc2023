mod module;
mod parser;

pub use parser::parse;

use std::collections::{HashMap, VecDeque};

use module::{Module, ModuleName, ModuleType, Pulse};

#[derive(Debug, PartialEq)]
pub struct SendPulseRequest {
    origin: ModuleName,
    pulse: Pulse,
    destinations: Vec<ModuleName>,
}

type ModuleRegistry = HashMap<ModuleName, Module<Box<dyn ModuleType>>>;

pub struct ModuleSystem {
    modules: ModuleRegistry,
    stats: ModuleSystemStats,
}

impl ModuleSystem {
    pub fn run(&mut self) -> Result<ModuleSystemStats, ModuleNotFoundError> {
        let initial_request = SendPulseRequest {
            origin: ModuleName::from("button"),
            pulse: Pulse::Low,
            destinations: vec![ModuleName::from("broadcaster")],
        };

        let mut pending = VecDeque::from([initial_request]);

        while let Some(request) = pending.pop_front() {
            pending.extend(self.handle(request)?.into_iter());
        }

        Ok(self.stats)
    }

    fn handle(
        &mut self,
        request: SendPulseRequest,
    ) -> Result<Vec<SendPulseRequest>, ModuleNotFoundError> {
        log::trace!("Handling {request:?}");
        let mut reactions = Vec::new();

        for destination in request.destinations.iter() {
            let dest_module = self.modules.get_mut(destination);
            log::trace!("Destination module lookup for {destination:?}: {dest_module:?}");

            match request.pulse {
                Pulse::Low => self.stats.low_pulse_count += 1,
                Pulse::High => self.stats.high_pulse_count += 1,
            };

            log::debug!(
                "Sending pulse {} → {:?} → {} [{:?}]",
                request.origin,
                request.pulse,
                destination,
                self.stats
            );

            if dest_module.is_none() {
                continue;
            }

            let dest_module = dest_module.unwrap();

            if let Some(reaction) = dest_module.send_pulse(request.pulse, &request.origin) {
                log::trace!("Reaction is: {reaction:?}");
                reactions.push(reaction);
            } else {
                log::trace!("No reaction");
            }
        }

        Ok(reactions)
    }
}

impl From<ModuleRegistry> for ModuleSystem {
    fn from(value: ModuleRegistry) -> Self {
        Self {
            modules: value,
            stats: ModuleSystemStats::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ModuleSystemStats {
    pub low_pulse_count: usize,
    pub high_pulse_count: usize,
}

#[derive(Debug)]
pub struct ModuleNotFoundError(ModuleName);
impl std::error::Error for ModuleNotFoundError {}
impl std::fmt::Display for ModuleNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "module not found: {}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::module::{Broadcaster, Conjunction, FlipFlop, Output};

    use super::*;

    #[test]
    fn test_part1_example_1() {
        let mut modules = ModuleRegistry::new();

        modules.insert(
            ModuleName::from("broadcaster"),
            Module::new(
                ModuleName::from("broadcaster"),
                Box::new(Broadcaster::new()),
                vec![ModuleName::from("button")],
                vec![
                    ModuleName::from("a"),
                    ModuleName::from("b"),
                    ModuleName::from("c"),
                ],
            ),
        );

        modules.insert(
            ModuleName::from("a"),
            Module::new(
                ModuleName::from("a"),
                Box::new(FlipFlop::new()),
                vec![ModuleName::from("broadcaster"), ModuleName::from("inv")],
                vec![ModuleName::from("b")],
            ),
        );

        modules.insert(
            ModuleName::from("b"),
            Module::new(
                ModuleName::from("b"),
                Box::new(FlipFlop::new()),
                vec![ModuleName::from("broadcaster"), ModuleName::from("a")],
                vec![ModuleName::from("c")],
            ),
        );

        modules.insert(
            ModuleName::from("c"),
            Module::new(
                ModuleName::from("c"),
                Box::new(FlipFlop::new()),
                vec![ModuleName::from("broadcaster"), ModuleName::from("b")],
                vec![ModuleName::from("inv")],
            ),
        );

        modules.insert(
            ModuleName::from("inv"),
            Module::new(
                ModuleName::from("inv"),
                Box::new(Conjunction::new()),
                vec![ModuleName::from("c")],
                vec![ModuleName::from("a")],
            ),
        );

        let mut system = ModuleSystem::from(modules);

        let mut stats = system.run().unwrap();

        assert_eq!(stats.low_pulse_count, 8);
        assert_eq!(stats.high_pulse_count, 4);

        for _ in 1..1000 {
            stats = system.run().unwrap()
        }

        assert_eq!(stats.low_pulse_count, 8000);
        assert_eq!(stats.high_pulse_count, 4000);
    }

    #[test]
    fn test_part1_example_2() {
        let mut modules = ModuleRegistry::new();

        modules.insert(
            ModuleName::from("broadcaster"),
            Module::new(
                ModuleName::from("broadcaster"),
                Box::new(Broadcaster::new()),
                vec![ModuleName::from("button")],
                vec![ModuleName::from("a")],
            ),
        );

        modules.insert(
            ModuleName::from("a"),
            Module::new(
                ModuleName::from("a"),
                Box::new(FlipFlop::new()),
                vec![ModuleName::from("broadcaster")],
                vec![ModuleName::from("inv"), ModuleName::from("con")],
            ),
        );

        modules.insert(
            ModuleName::from("inv"),
            Module::new(
                ModuleName::from("inv"),
                Box::new(Conjunction::new()),
                vec![ModuleName::from("a")],
                vec![ModuleName::from("b")],
            ),
        );

        modules.insert(
            ModuleName::from("b"),
            Module::new(
                ModuleName::from("b"),
                Box::new(FlipFlop::new()),
                vec![ModuleName::from("inv")],
                vec![ModuleName::from("con")],
            ),
        );

        modules.insert(
            ModuleName::from("con"),
            Module::new(
                ModuleName::from("con"),
                Box::new(Conjunction::new()),
                vec![ModuleName::from("b")],
                vec![ModuleName::from("output")],
            ),
        );

        modules.insert(
            ModuleName::from("output"),
            Module::new(
                ModuleName::from("output"),
                Box::new(Output::new()),
                vec![ModuleName::from("con")],
                vec![],
            ),
        );

        let mut system = ModuleSystem::from(modules);

        for _ in 1..1000 {
            system.run().unwrap();
        }

        let stats = system.run().unwrap();

        assert_eq!(stats.low_pulse_count, 4250);
        assert_eq!(stats.high_pulse_count, 2750);
    }
}
