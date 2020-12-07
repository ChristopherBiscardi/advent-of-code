#![feature(bool_to_option)]
#![feature(iterator_fold_self)]

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn process_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|lines| {
            lines
                .chars()
                .filter(|v| *v != '\n')
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(|lines| {
            lines
                .split('\n')
                .map(|line| line.chars().collect::<HashSet<char>>())
                .fold_first(|acc, v| acc.intersection(&v).cloned().collect())
                .map(|m| m.len())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input_process_one() {
        assert_eq!(
            process_part1(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            ),
            11
        );
    }
    #[test]
    fn test_input_process_two() {
        assert_eq!(
            process_part2(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            ),
            6
        );
    }
}
