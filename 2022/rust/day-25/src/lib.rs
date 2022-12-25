#[cfg(test)]
use rstest_reuse::{self, *};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    character::complete::{
        alpha1, char, line_ending, one_of, u32,
    },
    combinator::{eof, iterator},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    *,
};
use radix_fmt::radix_5;
use std::{iter::Sum, str::FromStr};
use tracing::*;

fn snafu_digits(input: &str) -> IResult<&str, Vec<i8>> {
    many1(alt((
        char('2').map(|_| 2),
        char('1').map(|_| 1),
        char('0').map(|_| 0),
        char('-').map(|_| -1),
        char('=').map(|_| -2),
    )))(input)
}
fn snafu(input: &str) -> IResult<&str, Snafu> {
    let (input, digits) = snafu_digits(input)?;
    let decimal = digits.iter().rev().enumerate().fold(
        0,
        |acc, (index, item)| {
            let contribution =
                *item as i64 * 5i64.pow(index as u32);

            acc + contribution
        },
    );
    Ok((input, Snafu { decimal }))
}
#[derive(Debug, PartialEq)]
struct Snafu {
    decimal: i64,
}
impl Snafu {
    fn to_snafu_string(&self) -> String {
        // 124030
        // 2=-1=0
        let result =
            itertools::unfold(self.decimal, |num| {
                if num == &0 {
                    None
                } else {
                    match *num % 5 {
                        0 => {
                            *num /= 5;
                            Some('0')
                        }
                        1 => {
                            *num -= 1;
                            *num /= 5;
                            Some('1')
                        }
                        2 => {
                            *num -= 2;
                            *num /= 5;
                            Some('2')
                        }
                        3 => {
                            *num -= -2;
                            *num /= 5;
                            Some('=')
                        }
                        4 => {
                            *num -= -1;
                            *num /= 5;
                            Some('-')
                        }
                        _ => panic!("impossible"),
                    }
                }
            })
            .collect::<String>();
        result.chars().rev().collect::<String>()
    }
}
impl FromStr for Snafu {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, snafu) = snafu(s).map_err(|err| {
            dbg!(err);
            "unparsable"
        })?;
        Ok(snafu)
    }
}

// impl Sum<Snafu> for i64 {
//     fn sum<I: Iterator<Item = Snafu>>(iter: I) -> Self {
//         iter.map(|snafu| snafu.decimal).sum()
//     }
// }
impl Sum<Snafu> for Snafu {
    fn sum<I: Iterator<Item = Snafu>>(iter: I) -> Self {
        Snafu {
            decimal: iter.map(|snafu| snafu.decimal).sum(),
        }
    }
}

#[instrument(skip(input))]
pub fn process_part1(input: &str) -> String {
    let sum = input
        .split("\n")
        .map(|s| s.parse::<Snafu>().unwrap())
        .sum::<Snafu>();
    sum.to_snafu_string()
}

#[instrument(skip(input))]
pub fn process_part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    use rstest::rstest;
    use rstest_reuse::{self, *};

    #[template]
    #[rstest]
    #[case("1", 1)]
    #[case("2", 2)]
    #[case("1=", 3)]
    #[case("1-", 4)]
    #[case("10", 5)]
    #[case("11", 6)]
    #[case("12", 7)]
    #[case("2=", 8)]
    #[case("2-", 9)]
    #[case("20", 10)]
    #[case("1=0", 15)]
    #[case("1-0", 20)]
    #[case("1=11-2", 2022)]
    #[case("1-0---0", 12345)]
    #[case("1121-1110-1=0", 314159265)]
    #[case("1=-0-2", 1747)]
    #[case("12111", 906)]
    #[case("2=0=", 198)]
    #[case("21", 11)]
    #[case("2=01", 201)]
    #[case("111", 31)]
    #[case("20012", 1257)]
    #[case("112", 32)]
    #[case("1=-1=", 353)]
    #[case("1-12", 107)]
    #[case("12", 7)]
    #[case("1=", 3)]
    #[case("122", 37)]
    fn snafu_parse_test(
        #[case] input: &str,
        #[case] expected: i64,
    ) {
        assert_eq!(
            expected,
            input.parse::<Snafu>().unwrap().decimal
        )
    }

    #[apply(snafu_parse_test)]
    fn to_string(
        #[case] expected: &str,
        #[case] input: i64,
    ) {
        assert!(
            expected
                == Snafu { decimal: input }
                    .to_snafu_string()
        );
    }
    #[test]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part1(INPUT), "2=-1=0");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part2(INPUT), "54");
    }
}
