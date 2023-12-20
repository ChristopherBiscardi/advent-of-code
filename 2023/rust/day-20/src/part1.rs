use std::{
    collections::{HashMap, HashSet, VecDeque},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    IResult,
};
use tracing::info;

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Signal {
    High,
    Low,
}

#[derive(Debug)]
enum Status {
    On,
    Off,
}
#[derive(Debug)]
enum MachineType<'a> {
    Broadcast,
    FlipFlop { status: Status },
    Conjunction { memory: HashMap<&'a str, Signal> },
}
#[derive(Debug)]
struct Machine<'a> {
    machine_type: MachineType<'a>,
    id: &'a str,
    output: Vec<&'a str>,
}

impl<'a> Machine<'a> {
    #[tracing::instrument]
    fn process(
        &mut self,
        sending_machine_id: String,
        signal: &Signal,
    ) -> Vec<(From, To, Signal)> {
        match &mut self.machine_type {
            MachineType::Broadcast => self
                .output
                .iter()
                .map(|&id| {
                    (
                        self.id.to_string(),
                        id.to_string(),
                        *signal,
                    )
                })
                .collect::<Vec<(From, To, Signal)>>(),
            MachineType::FlipFlop { ref mut status } => {
                match (signal, &status) {
                    (Signal::High, _) => vec![],
                    (Signal::Low, Status::On) => {
                        *status = Status::Off;
                        self.output
                            .iter()
                            .map(|&id| (self.id.to_string(), id.to_string(), Signal::Low))
                            .collect::<Vec<(From, To, Signal)>>()
                    }
                    (Signal::Low, Status::Off) => {
                        *status = Status::On;
                        self.output
                            .iter()
                            .map(|&id| (self.id.to_string(), id.to_string(), Signal::High))
                            .collect::<Vec<(From, To, Signal)>>()
                    }
                }
            }
            MachineType::Conjunction { memory } => {
                info!(?memory);

                *memory
                    .get_mut(sending_machine_id.as_str())
                    .unwrap() = *signal;

                let new_signal = if memory
                    .values()
                    .all(|s| s == &Signal::High) { Signal::Low } else { Signal::High };
                self.output
                    .iter()
                    .map(|id| {
                        (
                            self.id.to_string(),
                            id.to_string(),
                            new_signal,
                        )
                    })
                    .collect::<Vec<(From, To, Signal)>>()
            }
        }
    }
}

fn broadcast(input: &str) -> IResult<&str, Machine> {
    let (input, _) = tag("broadcaster -> ")(input)?;
    let (input, outputs) =
        separated_list1(tag(", "), alpha1)(input)?;
    Ok((
        input,
        Machine {
            machine_type: MachineType::Broadcast,
            id: "broadcaster",
            output: outputs,
        },
    ))
}
fn flip_flop(input: &str) -> IResult<&str, Machine> {
    let (input, _) = tag("%")(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, outputs) =
        separated_list1(tag(", "), alpha1)(input)?;
    Ok((
        input,
        Machine {
            machine_type: MachineType::FlipFlop {
                status: Status::Off,
            },
            id: name,
            output: outputs,
        },
    ))
}
fn conjunction(input: &str) -> IResult<&str, Machine> {
    let (input, _) = tag("&")(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, outputs) =
        separated_list1(tag(", "), alpha1)(input)?;
    Ok((
        input,
        Machine {
            machine_type: MachineType::Conjunction {
                memory: HashMap::new(),
            },
            id: name,
            output: outputs,
        },
    ))
}
fn parse(
    input: &str,
) -> IResult<&str, HashMap<&str, Machine>> {
    let (input, machines) = separated_list1(
        line_ending,
        alt((broadcast, flip_flop, conjunction)),
    )(input)?;
    Ok((
        input,
        machines
            .into_iter()
            .map(|machine| (machine.id, machine))
            .collect(),
    ))
}

// type From<'a> = &'a str;
// type To<'a> = &'a str;
type From = String;
type To = String;

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_input, mut machines) =
        parse(input).expect("should parse");
    {
        let (a, b) = machines.iter().fold(
            (vec![], vec![]),
            |mut acc: (Vec<&str>, Vec<&str>),
             (id, machine)| {
                acc.0.push(*id);
                acc.1.extend(&machine.output);
                acc
            },
        );
        let mut t = HashSet::<&str>::new();
        for i in a.iter() {
            t.insert(i);
        }
        let t2 = HashSet::<&str>::new();
        for i in b.iter() {
            t.insert(i);
        }

        info!(dif = ?t2.difference(&t));
    }
    let conjunctions = machines
        .iter()
        .filter_map(|(id, machine)| {
            match &machine.machine_type {
                MachineType::Broadcast => None,
                MachineType::FlipFlop { .. } => None,
                MachineType::Conjunction { .. } => {
                    Some(*id)
                }
            }
        })
        .collect::<Vec<&str>>();
    let inputs = machines.iter().fold(
        HashMap::<&str, Vec<&str>>::new(),
        |mut acc, (id, machine)| {
            for c in conjunctions.iter() {
                if machine.output.contains(c) {
                    acc.entry(c)
                        .and_modify(|item| {
                            item.push(id);
                        })
                        .or_insert(vec![id]);
                }
            }
            acc
        },
    );
    inputs.into_iter().for_each(
        |(conjunction, input_machines)| {
            machines.entry(conjunction).and_modify(
                |machine| {
                    let MachineType::Conjunction {
                        memory,
                        ..
                    } = &mut machine.machine_type
                    else {
                        unreachable!("has to exist");
                    };
                    *memory = input_machines
                        .into_iter()
                        .map(|id| (id, Signal::Low))
                        .collect();
                },
            );
        },
    );

    let button_pushes = 1000;
    let mut high_pulses = 0;
    let mut low_pulses = 0;
    for _ in 0..button_pushes {
        low_pulses += 1;
        let mut inbox =
            VecDeque::<(From, To, Signal)>::from([(
                String::from("button"),
                String::from("broadcaster"),
                Signal::Low,
            )]);
        while let Some((from, id, signal)) =
            inbox.pop_front()
        {
            info!(id, ?signal);
            let output = machines
                .get_mut(id.as_str())
                .map(|m| m.process(from.clone(), &signal))
                .unwrap_or(vec![]);
            for (_, _, signal) in output.iter() {
                match signal {
                    Signal::High => {
                        high_pulses += 1;
                    }
                    Signal::Low => {
                        low_pulses += 1;
                    }
                }
            }
            info!(?output, "{id} ->");
            inbox.extend(output);
        }
    }
    info!(high_pulses, low_pulses);
    Ok((high_pulses * low_pulses).to_string())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        "32000000"
    )]
    #[case(
        "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        "11687500"
    )]
    #[test_log::test]
    fn test_process(
        #[case] input: &str,
        #[case] expected: &str,
    ) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
