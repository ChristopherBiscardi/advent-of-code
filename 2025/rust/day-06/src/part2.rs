use itertools::{Itertools, Position, izip, multizip};
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
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    // let (_input, (nums, ops)) = parse.parse(input).unwrap();
    let mut ops = vec![];
    let mut lines_iterators = vec![];
    for (pos, line) in input.lines().with_position() {
        if let Position::Last = pos {
            let (_input, mut output) =
                operations(line).unwrap();
            output.reverse();
            ops = output;
        }
        lines_iterators.push(line.chars().rev());
    }
    let result = ops
        .iter()
        .map(|op| {
            let mut output = match *op {
                "*" => 1,
                "+" => 0,
                _ => {
                    panic!("");
                }
            };
            loop {
                let result: u64 = lines_iterators
                    .iter_mut()
                    .rev()
                    .filter_map(|line| {
                        line.next()
                            .and_then(|c| c.to_digit(10))
                    })
                    .enumerate()
                    .map(|(places, digit)| {
                        digit as u64
                            * 10u64.pow(places as u32)
                    })
                    .sum();
                if result == 0 {
                    break;
                }
                match *op {
                    "*" => {
                        output *= result;
                    }
                    "+" => {
                        output += result;
                    }
                    _ => {
                        panic!("");
                    }
                }
            }
            output
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
        operations,
    )
    .parse(input)
}

fn operations(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(space1, alt((tag("*"), tag("+"))))
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";
        assert_eq!("3263827", process(input)?);
        Ok(())
    }
}
