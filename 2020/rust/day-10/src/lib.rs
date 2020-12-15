#![feature(non_ascii_idents)]
#![feature(bool_to_option)]

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
use rayon::prelude::*;
use std::convert::TryInto;

type Span<'a> = LocatedSpan<&'a str>;

fn num(input: Span) -> IResult<Span, isize> {
    let (input, num_str) = digit1(input)?;
    let num = num_str.parse::<isize>().unwrap();

    Ok((input, num))
}

fn nums(input: Span) -> IResult<Span, Vec<isize>> {
    let (input, vs) = separated_list1(char('\n'), num)(input)?;
    Ok((input, vs))
}
pub fn process_part1(input: &str, preamble_length: usize) -> isize {
    // let nums = nums(Span::new(input)).ok().unwrap().1;
    let nums = input
        .lines()
        .map(|num_str| num_str.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    // err is an err, but we want that value here
    // to detect the infinite loop
    find_part1_num(&nums, preamble_length)
}

fn find_part1_num(nums: &[isize], preamble_length: usize) -> isize {
    *nums
        .windows(preamble_length + 1)
        .find_map(|num_group| {
            let (cur_num, preamble_nums) = num_group.split_last().unwrap();
            preamble_nums
                .iter()
                .all(|preamble_num| !preamble_nums.contains(&(cur_num - preamble_num)))
                .then(|| cur_num)
        })
        .expect("no num found")
}

pub fn process_part2(input: &str, preamble_length: usize) -> isize {
    let nums = input
        .lines()
        .map(|num_str| num_str.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let part_1_num = find_part1_num(&nums, preamble_length);
    for window_size in 2.. {
        let result = nums
            .windows(window_size)
            .find(|v| v.iter().sum::<isize>() == part_1_num)
            .and_then(|window| {
                window.iter().max().map(|max| {
                    let min = window.iter().min().unwrap();
                    min + max
                })
            });
        match result {
            None => continue,
            Some(v) => return v,
        }
    }
    panic!("SHOULDN'T REACH THIS; AOC INPUT IS VALID AND YOU ARE TOO.");
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
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
                5
            ),
            127
        );
    }
    #[test]
    fn test_input_process_two() {
        assert_eq!(
            process_part2(
                "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
                5
            ),
            62
        );
    }
}
