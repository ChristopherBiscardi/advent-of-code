use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::opt,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};
use rangemap::RangeInclusiveSet;
use std::ops::RangeInclusive;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, ranges) = ranges.parse(input).unwrap();

    let mut range_set = RangeInclusiveSet::new();
    for range in ranges {
        range_set.insert(range);
    }
    let result = range_set
        .iter()
        .map(|range| range.end() + 1 - range.start())
        .sum::<u64>();

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
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
