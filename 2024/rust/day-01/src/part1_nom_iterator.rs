use miette::miette;
use nom::{
    character::complete::{self, newline, space1},
    combinator::{iterator, opt},
    sequence::{separated_pair, terminated},
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (mut left, mut right)) = parse(input)
        .map_err(|e| miette!("parse failed {}", e))?;

    left.sort();
    right.sort();

    let result: i32 = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum();

    Ok(result.to_string())
}

fn parse(
    input: &str,
) -> IResult<&str, (Vec<i32>, Vec<i32>)> {
    let mut it = iterator(
        input,
        terminated(
            separated_pair(
                complete::i32,
                space1,
                complete::i32,
            ),
            opt(newline),
        ),
    );

    let parsed = it.collect::<(Vec<i32>, Vec<i32>)>();
    let res: IResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
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
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
