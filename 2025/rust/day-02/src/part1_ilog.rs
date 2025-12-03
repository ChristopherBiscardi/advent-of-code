use miette::miette;
use nom::{
    IResult, Parser, bytes::complete::tag,
    character::complete, combinator::all_consuming,
    multi::separated_list1, sequence::separated_pair,
};
use std::ops::RangeInclusive;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, id_ranges) =
        all_consuming(ranges).parse(input).map_err(
            |e| miette!("failed to parse aoc input, {e}"),
        )?;
    let mut total = 0;
    for id in id_ranges.into_iter().flatten() {
        // a number from 0-5, which is half of the
        // number of digits in the number
        let places = (id.ilog10() + 1) / 2;
        // 10^n, which is 10, 100, 1000, etc
        let hundos = 10u64.pow(places);
        // 204204 == 204 === 204
        if id / hundos == id % hundos {
            total += id;
        }
    }
    Ok(total.to_string())
}

fn ranges(
    input: &str,
) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(
        tag(","),
        separated_pair(
            complete::u64,
            tag("-"),
            complete::u64,
        )
        .map(|(start, end)| start..=end),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }
}
