#![feature(bool_to_option)]

use itertools::Itertools;
use nom::{bytes::complete::tag, character, IResult};
use std::io::{Error, ErrorKind};

pub fn process_part1(input: &str) -> usize {
    input
        .lines()
        .group_by(|line| line.len() > 0)
        .into_iter()
        .filter_map(|(key, passport_group)| {
            let mut raw_passport = passport_group.collect::<String>();
            if raw_passport.len() == 0 {
                return None;
            }
            (raw_passport.contains("byr:")
                && raw_passport.contains("iyr:")
                && raw_passport.contains("eyr:")
                && raw_passport.contains("hgt:")
                && raw_passport.contains("hcl:")
                && raw_passport.contains("ecl:")
                && raw_passport.contains("pid:"))
            .then_some(true)
        })
        .count()
}

pub fn process_part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input_one() {
        assert_eq!(
            process_part1(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in",
            ),
            2
        )
    }
    #[test]
    fn test_input_two() {
        assert_eq!(
            process_part2(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
            ),
            336
        )
    }
}
