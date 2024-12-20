use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{line_ending, one_of},
    combinator::{all_consuming, opt},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;
use pathfinding::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Write},
    ops::Not,
};
use tracing::info;

const DIRECTIONS: [IVec2; 4] =
    [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, Map { start, end, walls }) =
        all_consuming(parse)(Span::new(input))
            .map_err(|e| miette!("parse failed {}", e))?;

    let x_max =
        walls.iter().map(|pos| pos.x).max().unwrap();
    let y_max =
        walls.iter().map(|pos| pos.y).max().unwrap();
    // run first pathfind
    let first_run = dijkstra(
        &start,
        |position| {
            DIRECTIONS
                .iter()
                .filter_map(|direction| {
                    let next_pos = position + direction;
                    walls
                        .contains(&next_pos)
                        .not()
                        .then_some((next_pos, 1))
                })
                .collect::<Vec<_>>()
        },
        |&pos| pos == end,
    )
    .expect("a valid aoc result");

    dbg!(first_run.1);

    // run each pathfind with one missing wall
    // if that wall has at least two empty sides.
    let result = walls
        .iter()
        .filter(|wall| {
            DIRECTIONS
                .iter()
                .filter(|direction| {
                    let next_pos = **wall + **direction;
                    (0..x_max).contains(&next_pos.x)
                        && (0..y_max).contains(&next_pos.y)
                        && walls.contains(&next_pos).not()
                })
                .count()
                >= 2
        })
        .filter_map(|wall| {
            dijkstra(
                &start,
                |position| {
                    DIRECTIONS
                        .iter()
                        .filter_map(|direction| {
                            let next_pos =
                                position + direction;
                            (next_pos == *wall
                                || walls
                                    .contains(&next_pos)
                                    .not())
                            .then_some((next_pos, 1))
                        })
                        .collect::<Vec<_>>()
                },
                |&pos| pos == end,
            )
            .map(|(path, cost)| (path, cost, wall))
        })
        .map(|(_path, cost, _wall)| first_run.1 - cost)
        .filter(|cost| cost >= &100)
        .count();

    Ok(result.to_string())
}

pub type Span<'a> = LocatedSpan<&'a str>;

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let y = input.location_line();
    let x = input.get_column();
    let (input, token) = one_of(".#SE")(input)?;

    Ok((
        input,
        (
            IVec2::new(x as i32 - 1, y as i32 - 1),
            token,
        ),
    ))
}

pub struct Map {
    pub start: IVec2,
    pub end: IVec2,
    pub walls: HashSet<IVec2>,
}
pub fn parse(input: Span) -> IResult<Span, Map> {
    let (input, items) =
        separated_list1(line_ending, many1(token))(input)?;
    let (input, _) = opt(line_ending)(input)?;

    let (starting_position, _) = items
        .iter()
        .flatten()
        .find(|(_, value)| value == &'S')
        .cloned()
        .expect("should have a player");
    let (ending_position, _) = items
        .iter()
        .flatten()
        .find(|(_, value)| value == &'E')
        .cloned()
        .expect("should have a player");
    let walls = items
        .into_iter()
        .flatten()
        .filter_map(|(pos, value)| {
            (value == '#').then_some(pos)
        })
        .collect::<HashSet<IVec2>>();

    Ok((
        input,
        Map {
            start: starting_position,
            end: ending_position,
            walls: walls,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!("44", process(input)?);
        Ok(())
    }
}
