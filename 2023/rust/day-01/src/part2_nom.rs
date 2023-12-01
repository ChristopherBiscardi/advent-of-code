use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{iterator, value},
    // multi::many1,
    IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let output =
        input.lines().map(process_line).sum::<u32>();

    Ok(output.to_string())
}

fn numbers(input: &str) -> IResult<&str, Option<u32>> {
    let res: IResult<&str, u32> = alt((
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input);

    let (input, digit) = anychar(input)?;

    match res {
        Ok((_, digit)) => Ok((input, Some(digit))),
        Err(_) => Ok((input, digit.to_digit(10))),
    }
}

fn parser(input: &str) -> IResult<&str, Vec<u32>> {
    // can do this more simply than iterator, but it costs
    // some microseconds it
    // let (input, output) = many1(numbers)(input)?;
    let mut it = iterator(input, numbers);

    let output = it.filter_map(|v| v).collect();
    let (input, _) = it.finish()?;

    Ok((input, output))
}

#[tracing::instrument]
fn process_line(line: &str) -> u32 {
    let result = parser(line).unwrap();

    let mut it = result.1.iter();
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    /// this test case is from the real input
    /// it tests two overlapping numbers
    /// where the second number should succeed
    #[case("fivezg8jmf6hrxnhgxxttwoneg", 51)]
    fn line_test(
        #[case] line: &str,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, process_line(line))
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
