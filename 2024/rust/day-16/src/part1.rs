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
use std::collections::{HashMap, HashSet};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, Map { start, end, walls }) =
        all_consuming(parse)(Span::new(input))
            .map_err(|e| miette!("parse failed {}", e))?;

    let result = dijkstra(
        &(start, IVec2::X),
        |(position, direction)| {
            let next_pos = position + direction;
            if walls.contains(&next_pos) {
                vec![
                    ((*position, direction.perp()), 1000),
                    ((*position, -direction.perp()), 1000),
                ]
            } else {
                vec![
                    ((next_pos, *direction), 1),
                    ((*position, direction.perp()), 1000),
                    ((*position, -direction.perp()), 1000),
                ]
            }
        },
        |&(pos, _)| pos == end,
    )
    .expect("a valid aoc result");

    Ok(result.1.to_string())
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

    use rstest::rstest;

    #[rstest]
    #[case(
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        "7036"
    )]
    #[case(
        "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
        "11048"
    )]
    fn test_process(
        #[case] input: &str,
        #[case] result: &str,
    ) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
