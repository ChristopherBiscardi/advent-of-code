use std::io::Write;
use std::{collections::BTreeMap, fs::File};

use itertools::Itertools;
use ndarray::{s, Array2, Axis, Zip};
use nom::{
    bytes::complete::tag,
    character::complete::{
        self, alpha1, anychar, newline, u32,
    },
    multi::{many1, separated_list1},
    sequence::{pair, separated_pair},
    IResult,
};
use std::fs;

fn puzzle_input(
    input: &str,
) -> IResult<&str, (&str, BTreeMap<(char, char), char>)> {
    let (input, initial_state) = alpha1(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, rules) = separated_list1(
        newline,
        separated_pair(
            pair(anychar, anychar),
            tag(" -> "),
            anychar,
        ),
    )(input)?;

    let ruleset = rules
        .into_iter()
        .collect::<BTreeMap<(char, char), char>>();

    Ok((input, (initial_state, ruleset)))
}
pub fn process_part1(input: &str) -> usize {
    let (_, (initial_state, ruleset)) =
        puzzle_input(input).unwrap();

    let mut state = initial_state.to_string();
    for _ in 0..10 {
        // dbg!(&state);
        let last = state.chars().last().unwrap().clone();
        state = state
            .chars()
            .tuple_windows()
            .map(|pair: (char, char)| {
                let new_char = ruleset.get(&pair).unwrap();
                format!("{}{}", pair.0, new_char)
            })
            .collect::<String>();
        state.push(last);
    }
    let groups = state
        .chars()
        .sorted()
        .group_by(|c| *c)
        .into_iter()
        .map(|(c, group)| (c, group.count()))
        .collect::<Vec<(char, usize)>>();
    let max = groups
        .iter()
        .max_by_key(|(_, count)| count)
        .unwrap();
    let min = groups
        .iter()
        .min_by_key(|(_, count)| count)
        .unwrap();
    max.1 - min.1
}

pub fn process_part2(input: &str) -> usize {
    let (_, (initial_state, ruleset)) =
        puzzle_input(input).unwrap();

    let mut state: BTreeMap<(char, char), usize> =
        BTreeMap::new();
    for tuple in initial_state.chars().tuple_windows() {
        state
            .entry(tuple)
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);
    }

    for _ in 0..40 {
        let mut new_state: BTreeMap<(char, char), usize> =
            BTreeMap::new();
        for (pair, pair_count) in state.iter() {
            let new_char = ruleset.get(&pair).unwrap();

            new_state
                .entry((pair.0, *new_char))
                .and_modify(|count| {
                    *count += pair_count;
                })
                .or_insert(*pair_count);
            new_state
                .entry((*new_char, pair.1))
                .and_modify(|count| {
                    *count += pair_count;
                })
                .or_insert(*pair_count);
        }
        state = new_state
    }

    let mut new_counts: BTreeMap<char, usize> =
        BTreeMap::new();

    for (c, count) in
        state.iter().map(|((a, b), count)| (a, count))
    {
        new_counts
            .entry(*c)
            .and_modify(|v| {
                *v += count;
            })
            .or_insert(*count);
    }
    new_counts
        .entry(initial_state.chars().last().unwrap())
        .and_modify(|v| {
            *v += 1;
        })
        .or_insert(1);
    // for (c, thing) in new_counts.iter() {
    //     dbg!(c, thing);
    // }

    let max = new_counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .unwrap();
    let min = new_counts
        .iter()
        .min_by_key(|(_, count)| *count)
        .unwrap();
    max.1 - min.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        include_str!("./test-input.txt");

    #[test]
    fn part1_test_demo_data() {
        assert_eq!(1588, process_part1(INPUT));
    }
    #[test]
    fn part2_test_demo_data() {
        assert_eq!(2188189693529, process_part2(INPUT));
    }
}
