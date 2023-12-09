use itertools::{Itertools, Position};
use tracing::{debug, info};

use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let result = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let mut start_numbers: Vec<i64> = vec![];
            loop {
                if nums.iter().all(|num| num == &0) {
                    break;
                }
                nums = nums
                    .iter()
                    .tuple_windows::<(&i64, &i64)>()
                    .with_position()
                    .map(|(position, (left, right))| {
                        match position {
                            Position::First
                            | Position::Only => {
                                start_numbers.push(*left);
                            }
                            _ => {}
                        };
                        right - left
                    })
                    .collect::<Vec<i64>>();
            }

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
