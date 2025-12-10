use std::collections::{HashMap, HashSet};

use bitvec::{bitvec, order::Lsb0, vec::BitVec};
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{self, line_ending, space1},
    multi::{fold_many1, many1, separated_list1},
    sequence::delimited,
};
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, machines) = machines(input).unwrap();

    let sum = machines
        .iter()
        .map(|machine| {
            let mut set = HashSet::<BitVec<u8>>::new();
            set.insert(machine.state.clone());
            let mut i = 0;
            loop {
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
                    .collect();
                i += 1;
                if set.contains(&machine.goal) {
                    break;
                }
            }
            i
        })
        .sum::<usize>();
    Ok(sum.to_string())
}

#[derive(Debug)]
struct Machine {
    state: BitVec<u8>,
    goal: BitVec<u8>,
    buttons: Vec<Vec<usize>>,
    jolt: Vec<u32>,
}

fn push_button(
    mut state: BitVec<u8>,
    button: &[usize],
) -> BitVec<u8> {
    for bit in button {
        let mut val =
            state.get_mut(*bit).expect("bit to exist");
        val.set(!*val);
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
    let (input, goal_set) = goal(input)?;
    let (input, _) = space1(input)?;
    let (input, buttons) =
        separated_list1(space1, button).parse(input)?;
    let (input, _) = space1(input)?;
    let (input, jolt) = joltage(input)?;
    Ok((
        input,
        Machine {
            state: bitvec![u8, Lsb0; 0; goal_set.len()],
            goal: goal_set,
            buttons,
            jolt,
        },
    ))
}
// [.##.]
fn goal(input: &str) -> IResult<&str, BitVec<u8>> {
    delimited(
        complete::char('['),
        fold_many1(
            alt((complete::char('.'), complete::char('#'))),
            || BitVec::<u8>::with_capacity(16),
            |mut acc: BitVec<_>, item| {
                acc.push(match item {
                    '.' => false,
                    '#' => true,
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
fn joltage(input: &str) -> IResult<&str, Vec<u32>> {
    delimited(
        complete::char('{'),
        separated_list1(complete::char(','), complete::u32),
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

        assert_eq!("7", process(input)?);
        Ok(())
    }
}
