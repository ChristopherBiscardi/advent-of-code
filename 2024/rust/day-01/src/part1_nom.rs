use miette::miette;
use nom::{
    character::complete::{self, newline, space1},
    combinator::opt,
    multi::fold_many1,
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

// fn parse_raw(
//     input: &str,
// ) -> IResult<&str, Vec<(i32, i32)>> {
//     separated_list1(
//         newline,
//         separated_pair(
//             complete::i32,
//             space1,
//             complete::i32,
//         ),
//     )(input)
// }

fn parse(
    input: &str,
) -> IResult<&str, (Vec<i32>, Vec<i32>)> {
    fold_many1(
        terminated(
            separated_pair(
                complete::i32,
                space1,
                complete::i32,
            ),
            opt(newline),
        ),
        || (Vec::new(), Vec::new()),
        |mut acc: (Vec<i32>, Vec<i32>), (l, r)| {
            acc.0.push(l);
            acc.1.push(r);
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
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
