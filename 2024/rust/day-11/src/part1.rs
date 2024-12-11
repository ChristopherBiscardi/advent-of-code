use either::Either;
use num_traits::Euclid;

#[tracing::instrument]
pub fn process(
    input: &str,
    blinks: u64,
) -> miette::Result<String> {
    let mut nums = input
        .split_ascii_whitespace()
        .map(|num| {
            num.parse::<u64>()
                .expect("numbers to be valid in aoc")
        })
        .collect::<Vec<u64>>();

    let mut all_iterations =
        std::iter::from_fn(move || {
            let next_nums: Vec<u64> = nums
                .iter()
                .flat_map(|num| match num {
                    0 => {
                        Either::<[u64; 1], [u64; 2]>::Left(
                            [1],
                        )
                        .into_iter()
                    }
                    n if (n
                        .checked_ilog10()
                        .unwrap_or(0)
                        + 1)
                        % 2
                        == 0 =>
                    {
                        let num_digits =
                            n.checked_ilog10().unwrap_or(0)
                                + 1;
                        let (left, right) = n
                            .div_rem_euclid(
                                &10u64.pow(num_digits / 2),
                            );

                        let return_value = Either::<
                            [u64; 1],
                            [u64; 2],
                        >::Right(
                            [
                            left, right,
                        ]
                        );

                        return_value.into_iter()
                    }
                    n => {
                        let return_value = Either::<
                            [u64; 1],
                            [u64; 2],
                        >::Left(
                            [
                            n * 2024,
                        ]
                        );

                        return_value.into_iter()
                    }
                })
                .collect();

            nums = next_nums.clone();
            Some(next_nums)
        });

    let result =
        all_iterations.nth(blinks as usize).unwrap().len();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input, 25 - 1)?);
        Ok(())
    }
}
