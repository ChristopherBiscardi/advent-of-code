use num_traits::Euclid;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(
    input: &str,
    blinks: u64,
) -> miette::Result<String> {
    let nums = input
        .split_ascii_whitespace()
        .map(|num| {
            num.parse::<u64>()
                .expect("numbers to be valid in aoc")
        })
        .collect::<Vec<u64>>();

    // stone_number, stone_count
    let mut cache: HashMap<u64, u64> = HashMap::default();

    for num in nums {
        cache
            .entry(num)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
    }

    for _ in 0..blinks {
        let mut new_cache: HashMap<u64, u64> =
            HashMap::default();

        for (num, count) in cache.into_iter() {
            match num {
                0 => {
                    new_cache
                        .entry(1)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(count);
                }
                n if (n.checked_ilog10().unwrap_or(0)
                    + 1)
                    % 2
                    == 0 =>
                {
                    let num_digits =
                        n.checked_ilog10().unwrap_or(0) + 1;
                    let (left, right) = n.div_rem_euclid(
                        &10u64.pow(num_digits / 2),
                    );

                    new_cache
                        .entry(left)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(count);
                    new_cache
                        .entry(right)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(count);
                }
                n => {
                    new_cache
                        .entry(n * 2024)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(count);
                }
            }
        }
        cache = new_cache;
    }

    Ok(cache.values().sum::<u64>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input, 25)?);
        Ok(())
    }
}
