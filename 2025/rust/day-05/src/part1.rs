use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};
use std::ops::RangeInclusive;
use tracing::info;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (ranges, items)) =
        all_consuming(parse).parse(input).unwrap();

    let result = items
        .iter()
        .filter(|item| {
            ranges.iter().any(|range| range.contains(item))
        })
        .count();

    Ok(result.to_string())
}

fn parse(
    input: &str,
) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    terminated(
        separated_pair(
            ranges,
            line_ending.and(line_ending),
            separated_list1(line_ending, complete::u64),
        ),
        opt(line_ending),
    )
    .parse(input)
}

fn ranges(
    input: &str,
) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(line_ending, range).parse(input)
}

fn range(
    input: &str,
) -> IResult<&str, RangeInclusive<u64>> {
    separated_pair(complete::u64, tag("-"), complete::u64)
        .map(|(a, b)| a..=b)
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
