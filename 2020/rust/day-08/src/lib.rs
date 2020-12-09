#![feature(non_ascii_idents)]
use rayon::prelude::*;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    combinator::opt,
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};
use std::convert::TryInto;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Clone)]
enum Instruction {
    NOOP(isize),
    ACCUMULATE(isize),
    JUMP(isize),
}
use Instruction::*;

fn to_num(sign: &str, num: isize) -> isize {
    match sign {
        "-" => (0 - num).try_into().unwrap(),
        "+" => num.try_into().unwrap(),
        _ => panic!("bad input b"),
    }
}
fn instruction(input: Span) -> IResult<Span, Instruction> {
    let (input, instruction) = alt((tag("nop"), tag("acc"), tag("jmp")))(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, sign) = alt((tag("-"), tag("+")))(input)?;
    let (input, num_str) = digit1(input)?;

    let num = num_str.parse::<isize>().unwrap();

    let ins = match *instruction {
        "nop" => NOOP(to_num(*sign, num)),
        "acc" => ACCUMULATE(to_num(*sign, num)),
        "jmp" => JUMP(to_num(*sign, num)),
        _ => panic!("bad input a"),
    };
    Ok((input, ins))
}

fn instructions(input: Span) -> IResult<Span, Vec<Instruction>> {
    let (input, vs) = separated_list1(char('\n'), instruction)(input)?;
    Ok((input, vs))
}
pub fn process_part1(input: &str) -> isize {
    let commands = instructions(Span::new(input)).ok().unwrap().1;
    // err is an err, but we want that value here
    // to detect the infinite loop
    execute(commands).unwrap_err()
}

fn execute(commands: Vec<Instruction>) -> Result<isize, isize> {
    let mut acc: isize = 0;
    let mut i: usize = 0;
    let mut ಠ_ಠ = vec![];
    loop {
        if ಠ_ಠ.contains(&i) {
            break Err(acc);
        } else {
            if i == commands.len() {
                break Ok(acc);
            } else {
                ಠ_ಠ.push(i);
            }
        };
        match &commands[i] {
            NOOP(_) => {
                i = i + 1;
            }
            ACCUMULATE(value) => {
                i = i + 1;
                acc = acc + value;
            }
            JUMP(value) => {
                if *value < 0 {
                    let v: usize = value.abs().try_into().expect("msg");
                    i = i - v;
                } else {
                    let v: usize = value.abs().try_into().expect("msg");
                    i = i + v;
                }
            }
        }
    }
}
pub fn process_part2(input: &str) -> isize {
    let commands = instructions(Span::new(input)).ok().unwrap().1;
    commands
        .par_iter()
        .enumerate()
        .filter_map(|(i, v)| match v {
            ACCUMULATE(v) => None,
            x => Some(i),
        })
        .map(|index| {
            let mut cmds = commands.clone();
            let replacement = match cmds[index] {
                JUMP(value) => NOOP(value),
                NOOP(value) => JUMP(value),
                _ => panic!("no instruction"),
            };
            cmds[index] = replacement;
            execute(cmds)
        })
        .find(|v| v.is_ok())
        .map(|v| v.ok())
        .flatten()
        .unwrap()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input_process_one() {
        assert_eq!(
            process_part1(
                "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            ),
            5
        );
    }
    #[test]
    fn test_input_process_two() {
        assert_eq!(
            process_part2(
                "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            ),
            8
        );
    }
}
