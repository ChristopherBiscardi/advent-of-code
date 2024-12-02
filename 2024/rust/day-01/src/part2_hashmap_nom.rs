use miette::miette;
use nom::{
    character::complete::{self, line_ending, space1},
    combinator::opt,
    multi::fold_many1,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (left, right)) = parse(input)
        .map_err(|e| miette!("parse failed {}", e))?;

    let result: u32 = left
        .iter()
        .map(|number| {
            number * right.get(number).unwrap_or(&0)
        })
        .sum();

    Ok(result.to_string())
}

fn parse(
    input: &str,
) -> IResult<&str, (Vec<u32>, HashMap<u32, u32>)> {
    fold_many1(
        terminated(
            separated_pair(
                complete::u32,
                space1,
                complete::u32,
            ),
            opt(line_ending),
        ),
        || (Vec::new(), HashMap::new()),
        |mut acc: (Vec<u32>, HashMap<u32, u32>), (l, r)| {
            acc.0.push(l);
            acc.1
                .entry(r)
                .and_modify(|v| {
                    *v += 1;
                })
                .or_insert(1);
            acc
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
