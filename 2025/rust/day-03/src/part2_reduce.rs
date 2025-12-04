#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let result = input
        .lines()
        .map(|bank| {
            let mut batteries: String =
                String::with_capacity(12);

            let mut current_index = 0;
            for i in 0..12 {
                let (index, first_max) = &bank
                    [current_index..(bank.len() - 11 + i)]
                    .chars()
                    .enumerate()
                    .reduce(|acc, next| {
                        if next.1 > acc.1 {
                            next
                        } else {
                            acc
                        }
                    })
                    .unwrap();

                batteries.push(*first_max);
                current_index = current_index + index + 1;
            }

            batteries.parse::<u64>().unwrap()
        })
        .sum::<u64>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        // let input = "897777777789777";
        assert_eq!("3121910778619", process(input)?);
        Ok(())
    }
}
