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

#[derive(Debug, Clone)]
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

    dbg!(map.len());
    debug_assert_eq!(input, "");

    let starting_nodes: Vec<&str> = map
        .keys()
        .filter(|key| key.ends_with("A"))
        .cloned()
        .collect();

    let results = starting_nodes
        .iter()
        .map(|node| {
            let mut visited_nodes = vec![*node];
            let mut current_node = *node;
            // find_cycle
            instructions
                .clone()
                .iter()
                .cycle()
                .enumerate()
                .position(|(index, instruction)| {
                    let options =
                        map.get(current_node).expect(
                            "always exist at a valid node",
                        );
                    let next_node = match instruction {
                        Direction::Left => options.0,
                        Direction::Right => options.1,
                    };
                    current_node = next_node;
                    next_node.ends_with("Z")
                })
                // .find_map(|(index, instruction)| {
                //     let options =
                //         map.get(current_node).expect(
                //             "always exist at a valid node",
                //         );
                //     let next_node = match instruction {
                //         Direction::Left => options.0,
                //         Direction::Right => options.1,
                //     };
                //     if visited_nodes.contains(&next_node) {
                //         let offset = visited_nodes
                //             .iter()
                //             .position(|node| {
                //                 node == &next_node
                //             })
                //             .unwrap();
                //         let cycle_length =
                //             visited_nodes.len() - offset;
                //         // dbg!(offset);
                //         // dbg!(index);
                //         Some((offset - 1, cycle_length))
                //     } else {
                //         current_node = next_node;
                //         visited_nodes.push(next_node);
                //         None
                //     }
                // })
                .expect("should find a cycle")
                + 1
        })
        .collect::<Vec<usize>>();

    // let cycle_lengths: Vec<usize> = results
    //     .iter()
    //     .map(|(_, cycle_length)| *cycle_length)
    //     .collect();

    // let cycle_lengths: usize = results
    // .iter()
    // .map(|(offset, cycle_length)| *)
    // .collect();
    dbg!(&results);
    let min_cycle = lcm(&results);
    dbg!(min_cycle);
    todo!()
    // Ok(.to_string())
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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
