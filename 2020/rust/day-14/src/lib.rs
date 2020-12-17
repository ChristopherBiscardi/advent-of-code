#![feature(non_ascii_idents)]
#![feature(bool_to_option)]

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1, newline},
    combinator::opt,
    multi::{fold_many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};
use rayon::prelude::*;
use std::{collections::HashMap, convert::TryInto};

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
enum Program<'a> {
    Mask(HashMap<usize, &'a str>),
    Set((usize, usize)),
}

fn limiter(stack: &[usize], limit: usize) {
    if stack.len() == limit {
        return stack;
    } else {
        limiter(vec![stack], limit)
    }
}
fn fmt_memory(mask: &HashMap<usize, &str>, num: usize) -> usize {
    let floating_bits = mask.iter().filter(|(k, v)| v == "X").collect::<Vec<_>>();
    floating_bits.len();
    let mut combos = vec![];
    let sources: [usize; 2] = [0, 1];
    for bit in &sources {
        for bit2 in &sources {
            for bit3 in &sources {
                for bit4 in &sources {
                    combos.push([bit, bit2, bit3, bit4])
                }
            }
        }
    }

    let n = format!("{:036b}", num)
        .chars()
        .enumerate()
        .map(|(i, num_v)| match mask.get(&i) {
            Some("X") => mask_v.to_string(),
            None => num_v.to_string(),
        })
        .collect::<String>();

    usize::from_str_radix(&n, 2).unwrap()
}
fn mask_bit(input: Span) -> IResult<Span, &str> {
    let (input, result) = alt((tag("X"), tag("1"), tag("0")))(input)?;
    Ok((input, *result))
}
fn mask(input: Span) -> IResult<Span, Program> {
    let (input, num_str) = tag("mask = ")(input)?;
    let (input, mask) = fold_many1(
        mask_bit,
        (HashMap::new(), 0),
        |(mut map, i): (HashMap<usize, &str>, usize), c| match c {
            "0" => (map, i + 1),
            v => {
                map.insert(i, v);
                (map, i + 1)
            }
        },
    )(input)?;

    Ok((input, Program::Mask(mask.0)))
}

fn mem_set(input: Span) -> IResult<Span, Program> {
    // mem[8] = 11
    let (input, _) = tag("mem[")(input)?;
    let (input, register) = digit1(input)?;
    let (input, _) = tag("] = ")(input)?;
    let (input, int) = digit1(input)?;

    Ok((
        input,
        Program::Set((
            register.parse::<usize>().unwrap(),
            int.parse::<usize>().unwrap(),
        )),
    ))
}

fn program(input: Span) -> IResult<Span, Vec<Program>> {
    let (input, vs) = separated_list1(char('\n'), alt((mask, mem_set)))(input)?;
    Ok((input, vs))
}
pub fn process_part1(input: &str, preamble_length: usize) -> usize {
    let (input, program) = program(Span::new(input)).unwrap();
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut cur_mask: HashMap<usize, &str> = HashMap::new();
    for action in program {
        match action {
            Program::Mask(mask) => {
                cur_mask = mask;
            }
            Program::Set((register, value)) => {
                memory.insert(register, fmt_memory(&cur_mask, value));
            }
        }
    }
    memory.iter().map(|(_, v)| v).sum()
}

pub fn process_part2(input: &str, preamble_length: usize) -> isize {
    todo!()
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
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0",
                5
            ),
            165
        );
    }
}
