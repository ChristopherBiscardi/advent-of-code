use itertools::Itertools;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const OPERATORS: [char; 2] = ['*', '+'];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, equations) = parse(input)
        .map_err(|e| miette!("parse failed {}", e))?;
    let result: u64 = equations
        .iter()
        .filter_map(|(test, numbers)| {
            let operator_count = numbers.len() - 1;
            (0..operator_count)
                .map(|_| OPERATORS)
                .multi_cartesian_product()
                .any(|seq| {
                    let mut s = seq.iter();
                    let result = numbers
                        .iter()
                        .copied()
                        .reduce(|acc, next_number| match s
                            .next()
                            .unwrap()
                        {
                            '*' => acc * next_number,
                            '+' => acc + next_number,
                            _ => panic!("invalid operator"),
                        })
                        .unwrap();
                    *test == result
                })
                .then_some(test)
        })
        .sum();

    Ok(result.to_string())
}

fn parse(
    input: &str,
) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(space1, complete::u64),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
