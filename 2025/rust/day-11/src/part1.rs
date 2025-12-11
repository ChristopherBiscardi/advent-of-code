use std::collections::BTreeMap;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    combinator::{iterator, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};
use pathfinding::prelude::count_paths;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, devices) = parse(input).unwrap();
    let n = count_paths(
        "you",
        |device| devices[device].iter().map(|&d| d),
        |&c| c == "out",
    );

    Ok(n.to_string())
}

fn parse(
    input: &str,
) -> IResult<&str, BTreeMap<&str, Vec<&str>>> {
    let mut it = iterator(
        input,
        terminated(
            separated_pair(
                alpha1,
                tag(": "),
                separated_list1(space1, alpha1),
            ),
            opt(line_ending),
        ),
    );
    let parsed = it.by_ref().collect::<BTreeMap<_, _>>();
    let res: IResult<_, _> = it.finish();
    res.map(|(input, _)| (input, parsed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!("5", process(input)?);
        Ok(())
    }
}
