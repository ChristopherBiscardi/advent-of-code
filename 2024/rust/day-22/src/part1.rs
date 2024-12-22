use std::iter::successors;

use miette::IntoDiagnostic;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result: usize = input
        .lines()
        .map(|line| {
            process_secret(line).unwrap().nth(2000).unwrap()
        })
        .sum();
    Ok(result.to_string())
}

fn process_secret(
    secret: &str,
) -> miette::Result<impl Iterator<Item = usize>> {
    let secret =
        secret.parse::<usize>().into_diagnostic()?;

    Ok(successors(Some(secret), |secret| {
        // Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
        let value = secret * 64;
        let secret = prune(mix(*secret, value));
        // Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
        let value = secret / 32;
        let secret = prune(mix(secret, value));
        // Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
        let value = secret * 2048;
        let secret = prune(mix(secret, value));

        Some(secret)
    }))
}

// To mix a value into the secret number, calculate the bitwise XOR of the given value and the secret number. Then, the secret number becomes the result of that operation. (If the secret number is 42 and you were to mix 15 into the secret number, the secret number would become 37.)
fn mix(secret: usize, value: usize) -> usize {
    secret ^ value
}
// To prune the secret number, calculate the value of the secret number modulo 16777216. Then, the secret number becomes the result of that operation. (If the secret number is 100000000 and you were to prune the secret number, the secret number would become 16113920.)
fn prune(secret: usize) -> usize {
    secret.rem_euclid(16777216)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("1", "8685429")]
    #[case("10", "4700978")]
    #[case("100", "15273692")]
    #[case("2024", "8667524")]
    fn test_process(
        #[case] input: &str,
        #[case] output: &str,
    ) -> miette::Result<()> {
        assert_eq!(
            output,
            process_secret(input)?
                .nth(2000)
                .unwrap()
                .to_string()
        );
        Ok(())
    }

    #[test]
    fn test_mix() {
        assert_eq!(37, mix(42, 15));
    }
    #[test]
    fn test_prune() {
        assert_eq!(16113920, prune(100000000));
    }

    #[test]
    #[ignore]
    fn test_sequence() {
        let input = "123";
        let output: Vec<&str> = "15887950
16495136
527345
704524
1553684
12683156
11100544
12249484
7753432
5908254"
            .lines()
            .collect();
        // assert_eq!()
    }
}
