use std::collections::{HashMap, HashSet};

use bitvec::{bitvec, order::Lsb0, vec::BitVec};
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{self, line_ending, space1},
    multi::{fold_many1, many1, separated_list1},
    sequence::delimited,
};
use rayon::prelude::*;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, machines) = machines(input).unwrap();

    let sum = machines
        .par_iter()
        .enumerate()
        .map(|(id, machine)| {
            info!(machine_id=?id, "starting");
            let mut set = HashSet::<Vec<usize>>::new();
            set.insert(machine.state.clone());
            let mut i = 0;
            loop {
                if i % 1000 == 0 {
                    info!(?i, ?id);
                }
                set = set
                    .into_iter()
                    .flat_map(|state| {
                        machine.buttons.iter().map(
                            move |button| {
                                push_button(
                                    state.clone(),
                                    &button,
                                )
                            },
                        )
                    })
                    .filter(|state| {
                        state
                            .iter()
                            .zip(machine.jolt.iter())
                            .all(|(a, b)| a <= b)
                    })
                    .collect();
                i += 1;
                if set.contains(&machine.jolt) {
                    break;
                }
            }
            info!(machine_id=?id, "ending");

            i
        })
        .sum::<usize>();
    Ok(sum.to_string())
}

#[derive(Debug)]
struct Machine {
    state: Vec<usize>,
    buttons: Vec<Vec<usize>>,
    jolt: Vec<usize>,
}

// use memoize::memoize;

// #[memoize]
fn push_button(
    mut state: Vec<usize>,
    button: &[usize],
) -> Vec<usize> {
    for bit in button {
        state[*bit] += 1;
    }
    state
}

// use bitvec::prelude::*;

// let mut bv: BitVec = BitVec::new();
// bv.push(false);
// bv.push(true);
fn machines(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(line_ending, machine).parse(input)
}
fn machine(input: &str) -> IResult<&str, Machine> {
    let (input, _goal_set) = goal(input)?;
    let (input, _) = space1(input)?;
    let (input, buttons) =
        separated_list1(space1, button).parse(input)?;
    let (input, _) = space1(input)?;
    let (input, jolt) = joltage(input)?;
    Ok((
        input,
        Machine {
            state: vec![0; jolt.len()],
            buttons,
            jolt,
        },
    ))
}
// [.##.]
fn goal(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        complete::char('['),
        fold_many1(
            alt((complete::char('.'), complete::char('#'))),
            Vec::new,
            |mut acc: Vec<_>, item| {
                acc.push(match item {
                    '.' => 0,
                    '#' => 1,
                    _ => {
                        panic!("invalid!");
                    }
                });
                acc
            },
        ),
        complete::char(']'),
    )
    .parse(input)
}
fn button(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        complete::char('('),
        separated_list1(
            complete::char(','),
            complete::usize,
        ),
        complete::char(')'),
    )
    .parse(input)
}
fn joltage(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        complete::char('{'),
        separated_list1(
            complete::char(','),
            complete::usize,
        ),
        complete::char('}'),
    )
    .parse(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        assert_eq!("33", process(input)?);
        Ok(())
    }
}
