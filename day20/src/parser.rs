use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, multispace1},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

use crate::{
    module::{Broadcaster, Conjunction, FlipFlop, Module, ModuleName, ModuleType},
    ModuleRegistry, ModuleSystem,
};

pub fn parse(input: &str) -> ModuleSystem {
    let definitions: Vec<(Box<dyn ModuleType>, ModuleName, Vec<ModuleName>)> = input
        .lines()
        .map(parse_module_definition)
        .map(Result::unwrap)
        .map(|(_remain, definition)| definition)
        .collect();

    let mut inputs: HashMap<ModuleName, Vec<ModuleName>> = HashMap::new();

    for (_mod_type, name, destinations) in definitions.iter() {
        for destination in destinations {
            inputs
                .entry(destination.clone())
                .or_insert_with(Vec::new)
                .push(name.clone());
        }
    }

    let modules: ModuleRegistry = definitions
        .into_iter()
        .map(|(mod_type, name, destinations)| {
            let mod_inputs = inputs.remove(&name).unwrap_or(vec![]);
            let module = Module::new(name.clone(), mod_type, mod_inputs, destinations);

            (name, module)
        })
        .collect();

    ModuleSystem::from(modules)
}

fn parse_module_definition(
    input: &str,
) -> IResult<&str, (Box<dyn ModuleType>, ModuleName, Vec<ModuleName>)> {
    let (remain, ((module_type, name), destinations)) = separated_pair(
        parse_module,
        delimited(multispace1, tag("->"), multispace1),
        parse_destinations,
    )(input)?;

    Ok((remain, (module_type, name, destinations)))
}

fn parse_module(input: &str) -> IResult<&str, (Box<dyn ModuleType>, ModuleName)> {
    alt((parse_broadcaster, parse_flip_flop, parse_conjunction))(input)
}

fn parse_destinations(input: &str) -> IResult<&str, Vec<ModuleName>> {
    separated_list0(terminated(tag(","), multispace0), parse_module_name)(input)
}

fn parse_module_name(input: &str) -> IResult<&str, ModuleName> {
    alphanumeric1(input).map(|(remain, word)| (remain, word.to_string()))
}

fn parse_broadcaster(input: &str) -> IResult<&str, (Box<dyn ModuleType>, ModuleName)> {
    tag("broadcaster")(input).map(|(remain, token)| {
        let mod_type: Box<dyn ModuleType> = Box::new(Broadcaster::new());
        let name = token.to_string();

        (remain, (mod_type, name))
    })
}

fn parse_flip_flop(input: &str) -> IResult<&str, (Box<dyn ModuleType>, ModuleName)> {
    preceded(tag("%"), parse_module_name)(input).map(|(remain, name)| {
        let mod_type: Box<dyn ModuleType> = Box::new(FlipFlop::new());

        (remain, (mod_type, name.clone()))
    })
}

fn parse_conjunction(input: &str) -> IResult<&str, (Box<dyn ModuleType>, ModuleName)> {
    preceded(tag("&"), parse_module_name)(input).map(|(remain, name)| {
        let mod_type: Box<dyn ModuleType> = Box::new(Conjunction::new());

        (remain, (mod_type, name.clone()))
    })
}
