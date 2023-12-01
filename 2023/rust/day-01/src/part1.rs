use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars();

            let first = it
                .find_map(|character| {
                    character.to_digit(10)
                })
                .expect("should be a number");

            let last = it
                .rfind(|character| character.is_digit(10))
                .map(|character| {
                    character.to_digit(10).unwrap()
                })
                // if we don't find a number, then we're
                // re-using the first number
                .unwrap_or(first);

            first * 10 + last
        })
        .sum::<u32>();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
