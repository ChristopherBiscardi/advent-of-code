use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, alpha1, alphanumeric1, line_ending,
        multispace1,
    },
    combinator::eof,
    multi::{fold_many1, many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn parser(
    input: &str,
) -> IResult<
    &str,
    (
        Vec<Direction>,
        BTreeMap<&str, (&str, &str)>,
    ),
> {
    let (input, instructions) = many1(alt((
        complete::char('R').map(|_| Direction::Right),
        complete::char('L').map(|_| Direction::Left),
    )))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, map) = fold_many1(
        terminated(
            separated_pair(
                alphanumeric1,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(
                        alphanumeric1,
                        tag(", "),
                        alphanumeric1,
                    ),
                    complete::char(')'),
                ),
            ),
            alt((line_ending, eof)),
        ),
        BTreeMap::new,
        |mut acc: BTreeMap<&str, (&str, &str)>,
         (key, value)| {
            acc.insert(key, value);
            acc
        },
    )(input)?;

    Ok((input, (instructions, map)))
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (input, (instructions, map)) =
        parser(input).expect("should validly parse");

    debug_assert_eq!(input, "");

    let mut current_nodes: Vec<&str> = map
        .keys()
        .filter(|key| key.ends_with("A"))
        .cloned()
        .collect();

    let Some(step_count) =
        instructions.iter().cycle().enumerate().find_map(
            |(index, instruction)| {
                if index % 1000000 == 0 {
                    println!("{}", index);
                }
                let next_nodes: Vec<&str> = current_nodes
                    .iter()
                    .map(|current_node| {
                        let options =
                        map.get(*current_node).expect(
                            "always exist at a valid node",
                        );
                        let next_node = match instruction {
                            Direction::Left => options.0,
                            Direction::Right => options.1,
                        };
                        next_node
                    })
                    .collect();

                if next_nodes
                    .iter()
                    .all(|node| node.ends_with("Z"))
                {
                    Some(index + 1)
                } else {
                    current_nodes = next_nodes;
                    None
                }
            },
        )
    else {
        panic!("infinite iterator can't produce None")
    };

    Ok(step_count.to_string())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        "6"
    )]
    fn test_process(
        #[case] input: &str,
        #[case] expected: &str,
    ) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
