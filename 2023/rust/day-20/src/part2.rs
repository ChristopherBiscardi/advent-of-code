use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Deref,
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

                let new_signal = memory
                    .values()
                    .all(|s| s == &Signal::High)
                    .then_some(Signal::Low)
                    .unwrap_or(Signal::High);
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
    let (input, mut machines) =
        parse(input).expect("should parse");

    let final_node = "rx";
    let penultimate_node = machines
        .iter()
        .find_map(|(id, machine)| {
            machine
                .output
                .contains(&final_node)
                .then_some(*id)
        })
        .unwrap();
    let mut penultimate_nodes = machines
        .iter()
        .filter_map(|(id, machine)| {
            machine
                .output
                .contains(&penultimate_node)
                .then_some(*id)
        })
        .collect::<Vec<&str>>();

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

    let mut lcms: Vec<usize> = vec![];
    for i in 0.. {
        if lcms.len() == 4 {
            break;
        }
        let mut inbox =
            VecDeque::<(From, To, Signal)>::from([(
                String::from("button"),
                String::from("broadcaster"),
                Signal::Low,
            )]);
        while let Some((from, id, signal)) =
            inbox.pop_front()
        {
            if penultimate_nodes.contains(&id.as_str())
                && signal == Signal::Low
            {
                let index = penultimate_nodes
                    .iter()
                    .position(|x| x == &id)
                    .unwrap();
                penultimate_nodes.remove(index);
                // dbg!(&penultimate_nodes, &id, i);
                lcms.push(i + 1);
            }
            info!(id, ?signal);
            let output = machines
                .get_mut(id.as_str())
                .map(|m| m.process(from.clone(), &signal))
                .unwrap_or(vec![]);

            info!(?output, "{id} ->");
            inbox.extend(output);
        }
    }

    Ok(lcm(&lcms).to_string())
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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
