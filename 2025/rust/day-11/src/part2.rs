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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Node<'a> {
    label: &'a str,
    fft: bool,
    dac: bool,
}
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, mut devices) = parse(input).unwrap();
    devices.insert("out", vec![]);

    let n = count_paths(
        Node {
            label: "svr",
            fft: false,
            dac: false,
        },
        |&device| {
            devices[device.label].iter().map(
                move |&next_label| Node {
                    label: next_label,
                    fft: device.fft || next_label == "fft",
                    dac: device.dac || next_label == "dac",
                },
            )
        },
        |&c| {
            c == Node {
                label: "out",
                fft: true,
                dac: true,
            }
        },
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
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
