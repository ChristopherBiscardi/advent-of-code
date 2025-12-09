use glam::I64Vec2;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
};
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, red_tiles) = parse(input).unwrap();
    let max = red_tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            (a.x.abs_diff(b.x) + 1)
                * (a.y.abs_diff(b.y) + 1)
        })
        .max()
        .expect("");
    Ok(max.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<I64Vec2>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::i64,
            tag(","),
            complete::i64,
        )
        .map(|(x, y)| I64Vec2::new(x, y)),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!("50", process(input)?);
        Ok(())
    }
}
