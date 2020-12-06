#![feature(bool_to_option)]

use itertools::Itertools;
use std::collections::HashMap;

pub fn process_part1(input: &str) -> usize {
    input
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(_, lines)| {
            let s = lines.collect::<String>();
            if s.is_empty() {
                return None;
            }
            let count = s.chars().unique().count();
            Some(count)
        })
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    input
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(_, line_group)| {
            // count occurrences of chars
            let line_maps = line_group
                .filter_map(|line| {
                    let map =
                        line.chars()
                            .fold(HashMap::new(), |mut map: HashMap<char, usize>, c| {
                                map.entry(c).and_modify(|v| *v += 1).or_insert(1);
                                map
                            });
                    if map.len() == 0 {
                        None
                    } else {
                        Some(map)
                    }
                })
                .collect::<Vec<_>>();

            let num_people = line_maps.len();
            let s = line_maps
                .into_iter()
                .fold(HashMap::new(), |mut acc: HashMap<char, usize>, map| {
                    for (c, num_ocurrences) in map.iter() {
                        acc.entry(*c).and_modify(|v| *v += 1).or_insert(1);
                    }

                    acc
                })
                .into_iter()
                .filter_map(|(k, v)| (v == num_people).then(|| 1));

            let count = s.count();
            Some(count)
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
