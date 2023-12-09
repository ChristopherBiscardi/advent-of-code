use nom::{
    bytes::complete::is_not,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

use crate::custom_error::AocError;

fn nums(input: &str) -> IResult<&str, u64> {
    is_not("0123456789")
        .precedes(separated_list1(space1, digit1).map(
            |list| {
                list.join("")
                    .parse::<u64>()
                    .expect("a valid number")
            },
        ))
        .parse(input)
}
fn parse_times(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(nums, line_ending, nums).parse(input)
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_, (time, record_distance)) =
        parse_times(input).expect("a valid parse");

    let min_time_held = (0..=time)
        .find(|time_held| {
            time_held * (time - time_held) > record_distance
        })
        .unwrap();
    let max_time_held = time - min_time_held;
    let result = max_time_held - min_time_held + 1;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}
