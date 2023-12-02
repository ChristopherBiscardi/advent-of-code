use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, alpha1, digit1, line_ending,
    },
    combinator::{eof, iterator, recognize},
    multi::{fold_many1, many1, separated_list1},
    sequence::{
        delimited, preceded, separated_pair, terminated,
    },
    IResult,
};
use std::collections::BTreeMap;

use crate::custom_error::AocError;

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    amount: u32,
}

#[derive(Debug)]
struct Game<'a> {
    #[allow(dead_code)]
    id: &'a str,
    rounds: Vec<Vec<Cube<'a>>>,
}

impl<'a> Game<'a> {
    #[allow(dead_code)]
    fn valid_for_cube_set(
        &self,
        map: &BTreeMap<&str, u32>,
    ) -> Option<u32> {
        self.rounds
            .iter()
            .all(|round| {
                round.iter().all(|shown_cube| {
                    shown_cube.amount
                        <= *map
                            .get(shown_cube.color)
                            .expect("a valid cube")
                })
            })
            .then_some(
                self.id.parse::<u32>().expect(
                    "game id should a parsable u32",
                ),
            )
    }
    fn minimum_cube_set(&self) -> u32 {
        let map: BTreeMap<&str, u32> = BTreeMap::new();
        self.rounds
            .iter()
            .fold(map, |mut acc, round| {
                for cube in round.iter() {
                    acc.entry(cube.color)
                        .and_modify(|v| {
                            *v = (*v).max(cube.amount);
                        })
                        .or_insert(cube.amount);
                }
                acc
            })
            .values()
            .product()
    }
}

// 4 red
fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) =
        separated_pair(complete::u32, tag(" "), alpha1)(
            input,
        )?;
    let (input, _) = alt((
        tag(", "),
        tag("; "),
        recognize(line_ending),
        recognize(eof),
    ))(input)?;
    Ok((input, Cube { color, amount }))
}
// 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// output: product of max cube values
fn parse_power(input: &str) -> IResult<&str, u32> {
    let (input, cubes) = fold_many1(
        cube,
        BTreeMap::new,
        |mut acc: BTreeMap<&str, u32>, cube| {
            acc.entry(cube.color)
                .and_modify(|v| {
                    *v = (*v).max(cube.amount);
                })
                .or_insert(cube.amount);
            acc
        },
    )(input)?;
    Ok((input, cubes.values().product()))
}
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// output: product of max cube values
fn game(input: &str) -> IResult<&str, u32> {
    let (input, _id) =
        delimited(tag("Game "), digit1, tag(": "))(input)?;
    let (input, power) = parse_power(input)?;
    Ok((input, power))
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut it = iterator(input, game);

    let sum = it.sum::<u32>();
    let _ = it.finish().expect("should parse successfully");

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        48
    )]
    #[case("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560)]
    #[case("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 630)]
    #[case(
        "6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        36
    )]
    fn power_test(
        #[case] line: &str,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, parse_power(line).unwrap().1)
    }

    #[rstest]
    #[case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        48
    )]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560)]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 630)]
    #[case(
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        36
    )]
    fn game_test(
        #[case] line: &str,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, game(line).unwrap().1)
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
