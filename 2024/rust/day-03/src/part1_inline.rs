use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::fold_many1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, result) = parse(input)
        .map_err(|e| miette!("parse failed {}", e))?;

    Ok(result.to_string())
}

fn instruction(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(
            complete::u32,
            tag(","),
            complete::u32,
        ),
        tag(")"),
    )(input)?;
    Ok((input, pair.0 * pair.1))
}
fn parse(input: &str) -> IResult<&str, u32> {
    fold_many1(
        alt((instruction, value(0, anychar))),
        || 0,
        |mut acc: u32, item| {
            acc += item;
            acc
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
