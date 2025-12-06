use itertools::{Itertools, izip, multizip};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, line_ending, space0, space1,
    },
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, (nums, ops)) = parse.parse(input).unwrap();
    let result = ops
        .iter()
        .enumerate()
        .map(|(index, op)| {
            let it = (0..nums.len()).into_iter().map(
                |inner_index| nums[inner_index][index],
            );
            match *op {
                "*" => it.product(),
                "+" => it.sum::<u64>(),
                _ => {
                    panic!("");
                }
            }
        })
        .sum::<u64>();
    Ok(result.to_string())
}

fn parse(
    input: &str,
) -> IResult<&str, (Vec<Vec<u64>>, Vec<&str>)> {
    separated_pair(
        separated_list1(
            line_ending,
            delimited(
                space0,
                separated_list1(space1, complete::u64),
                space0,
            ),
        ),
        line_ending,
        separated_list1(space1, alt((tag("*"), tag("+")))),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";
        assert_eq!("4277556", process(input)?);
        Ok(())
    }
}
