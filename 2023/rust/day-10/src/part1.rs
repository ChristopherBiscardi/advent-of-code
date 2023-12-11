use std::collections::{HashMap};

use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0},
    combinator::all_consuming,
    multi::many1,
    sequence::terminated,
    IResult, Parser,
};
use nom_locate::LocatedSpan;
use tracing::info;

use crate::custom_error::AocError;

#[derive(Debug, Eq, PartialEq)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    StartingPosition,
    Ground,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct PipeInfo<'a> {
    span: SpanIVec2<'a>,
    pipe_type: PipeType,
}

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

fn with_xy(span: Span) -> SpanIVec2 {
    // column/location are 1-indexed
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}

fn parse_grid(
    input: Span,
) -> IResult<Span, HashMap<IVec2, PipeType>> {
    let (input, output) =
        all_consuming(many1(terminated(
            alt((
                tag("|").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type: PipeType::Vertical,
                    }
                }),
                tag("-").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type: PipeType::Horizontal,
                    }
                }),
                tag("L").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type: PipeType::NorthEast,
                    }
                }),
                tag("J").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type: PipeType::NorthWest,
                    }
                }),
                tag("7").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type: PipeType::SouthWest,
                    }
                }),
                tag("F").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type: PipeType::SouthEast,
                    }
                }),
                tag("S").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type:
                            PipeType::StartingPosition,
                    }
                }),
                tag(".").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type: PipeType::Ground,
                    }
                }),
            )),
            multispace0,
        )))(input)?;

    Ok((
        input,
        output
            .into_iter()
            .filter_map(|pipe_info| {
                (pipe_info.pipe_type != PipeType::Ground)
                    .then_some((
                        pipe_info.span.extra,
                        pipe_info.pipe_type,
                    ))
            })
            .collect(),
    ))
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    info!(input);
    let (_input, grid) = parse_grid(Span::new(input))
        .expect("should parse a valid grid");

    let start_position = grid
        .iter()
        .find_map(|(key, value)| {
            (value == &PipeType::StartingPosition)
                .then_some(key)
        })
        .expect("should have a starting position");

    let north = *start_position + IVec2::new(0, -1);
    let north_position = grid
        .get(&north)
        .is_some_and(|pipe_type| match pipe_type {
            PipeType::Vertical
            | PipeType::SouthWest
            | PipeType::SouthEast => true,
            _ => false,
        })
        .then_some((Direction::South, north));
    let south = *start_position + IVec2::new(0, 1);
    let south_position = grid
        .get(&south)
        .is_some_and(|pipe_type| match pipe_type {
            PipeType::Vertical
            | PipeType::NorthWest
            | PipeType::NorthEast => true,
            _ => false,
        })
        .then_some((Direction::North, south));
    let east = *start_position + IVec2::new(1, 0);
    let east_position = grid
        .get(&east)
        .is_some_and(|pipe_type| match pipe_type {
            PipeType::Horizontal
            | PipeType::NorthWest
            | PipeType::SouthWest => true,
            _ => false,
        })
        .then_some((Direction::West, east));
    let west = *start_position + IVec2::new(-1, 0);
    let west_position = grid
        .get(&west)
        .is_some_and(|pipe_type| match pipe_type {
            PipeType::Horizontal
            | PipeType::NorthEast
            | PipeType::SouthEast => true,
            _ => false,
        })
        .then_some((Direction::East, west));

    let mut iters = vec![
        north_position,
        south_position,
        east_position,
        west_position,
    ]
    .into_iter()
    .flatten()
    .map(|tuple| {
        std::iter::successors(
            Some(tuple),
            |(from_direction, current_position)| {
                let pipe_type = grid
                .get(current_position)
                .expect("should not be asking for a grid position that doesn't exist");
                info!(?from_direction, ?current_position, ?pipe_type);

               let direction_to_go = match (from_direction, pipe_type) {
                    (Direction::North, PipeType::Vertical) => Direction::South,
                    (Direction::North, PipeType::NorthEast) => Direction::East,
                    (Direction::North, PipeType::NorthWest) => Direction::West,
                    (Direction::South, PipeType::Vertical) => Direction::North,
                    (Direction::South, PipeType::SouthEast) => Direction::East,
                    (Direction::South, PipeType::SouthWest) => Direction::West,
                    (Direction::East, PipeType::Horizontal) => Direction::West,
                    (Direction::East, PipeType::NorthEast) => Direction::North,
                    (Direction::East, PipeType::SouthEast) => Direction::South,
                    (Direction::West, PipeType::Horizontal) => Direction::East,
                    (Direction::West, PipeType::NorthWest) => Direction::North,
                    (Direction::West, PipeType::SouthWest) => Direction::South,
                    value => {unreachable!("should not land on Ground or loop around again, {:?}", value);}
                };
                Some(match direction_to_go {
                    Direction::North => (Direction::South, *current_position + IVec2::new(0, -1)),
                    Direction::South => (Direction::North, *current_position + IVec2::new(0, 1)),
                    Direction::East => (Direction::West, *current_position + IVec2::new(1, 0)),
                    Direction::West => (Direction::East, *current_position + IVec2::new(-1, 0)),
                })
            },
        )
    });

    let path_a = iters.next().expect("path a should_exist");
    let path_b = iters.next().expect("path b should exist");
    let final_position = path_a
        .zip(path_b)
        .position(|(a, b)| a.1 == b.1)
        .expect("should meet in the middle");

    // + 1 for the initial starting position
    Ok((final_position + 1).to_string())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        ".....
.S-7.
.|.|.
.L-J.
.....",
        "4"
    )]
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        "8"
    )]
    #[test_log::test]
    fn test_process(
        #[case] input: &str,
        #[case] output: &str,
    ) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }
}
