use std::ops::Not;

use itertools::Itertools;
use tracing::{debug, info};

use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let result = input
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let start_numbers =
                std::iter::successors(Some(nums), |nums| {
                    nums.iter()
                        .all(|num| num == &0)
                        .not()
                        .then_some(
                        nums.iter()
                            .tuple_windows::<(&i64, &i64)>()
                            .map(|(left, right)| {
                                right - left
                            })
                            .collect(),
                    )
                })
                .map(|v| *v.first().unwrap())
                .collect::<Vec<i64>>();

            debug!(?start_numbers);
            let result = start_numbers.iter().rev().fold(
                0,
                |acc, num| {
                    info!(acc, num, result = num - acc);
                    num - acc
                },
            );

            result
        })
        .sum::<i64>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
