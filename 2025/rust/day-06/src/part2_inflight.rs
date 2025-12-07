use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, space1},
    combinator::verify,
    multi::{many1, separated_list1},
    sequence::pair,
};
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (mut lines_iterators, mut ops)) =
        parse(input.as_bytes()).unwrap();
    ops.reverse();

    let result = ops
        .iter()
        .map(|op| {
            let mut output = match *op {
                // "*" => 1,
                // "+" => 0,
                [42] => 1,
                [43] => 0,
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
                            .and_then(|c| c.checked_sub(48))
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
                    [42] => {
                        output *= result;
                    }
                    [43] => {
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

// split into
pub fn parse(
    input: &[u8],
) -> IResult<
    &[u8],
    (
        Vec<impl Iterator<Item = &u8>>,
        Vec<&[u8]>,
    ),
> {
    pair(nums, operations).parse(input)
}

// fn parse(input: &[u8]) -> IResult<&[u8]> {}
fn nums(
    input: &[u8],
) -> IResult<&[u8], Vec<impl Iterator<Item = &u8>>> {
    many1(line).parse(input)
}

fn line(
    input: &[u8],
) -> IResult<&[u8], impl Iterator<Item = &u8>> {
    // this recognize would work, but using u8 is faster
    // and we don't actually need to do a full parse to
    // know we aren't trying to parse the operator line
    // recognize(preceded(
    //     space0,
    //     many1(terminated(complete::u64, space0)),
    // ))
    verify(
        take_until("\n")
            .and(line_ending)
            .map(|(line, _)| line),
        |v: &[u8]| ![42, 43].contains(&v[0]),
    )
    .parse(input)
    .map(|(input, output)| (input, output.iter().rev()))
}

fn operations(input: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
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
