use std::collections::{HashMap, HashSet};

use crate::custom_error::AocError;

use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    combinator::opt,
    multi::many1,
    sequence::delimited,
    IResult, Parser,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum MirrorType {
    Horizontal,
    Vertical,
    Backslash,
    ForwardSlash,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn step(&self, position: &IVec2) -> IVec2 {
        *position
            + match self {
                Direction::North => IVec2::new(0, -1),
                Direction::South => IVec2::new(0, 1),
                Direction::East => IVec2::new(1, 0),
                Direction::West => IVec2::new(-1, 0),
            }
    }
}

#[derive(Debug)]
struct Mirror<'a> {
    r#type: MirrorType,
    span: SpanIVec2<'a>,
}

fn with_xy(span: Span) -> SpanIVec2 {
    // column/location are 1-indexed
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}
fn parse_grid(input: Span) -> IResult<Span, Vec<Mirror>> {
    let (input, grid) = many1(delimited(
        opt(is_a(".\n")),
        alt((
            tag("/").map(|span| {
                let located = with_xy(span);
                Mirror {
                    r#type: MirrorType::ForwardSlash,
                    span: located,
                }
            }),
            tag(r#"\"#).map(|span| {
                let located = with_xy(span);
                Mirror {
                    r#type: MirrorType::Backslash,
                    span: located,
                }
            }),
            tag("-").map(|span| {
                let located = with_xy(span);
                Mirror {
                    r#type: MirrorType::Horizontal,
                    span: located,
                }
            }),
            tag("|").map(|span| {
                let located = with_xy(span);
                Mirror {
                    r#type: MirrorType::Vertical,
                    span: located,
                }
            }),
        )),
        opt(is_a(".\n")),
    ))(input)?;
    Ok((input, grid))
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let row_count = input.lines().count() as i32;
    let column_count =
        input.lines().next().unwrap().len() as i32;

    let (_input, grid) = parse_grid(Span::new(input))
        .expect("a valid parse");
    let grid_map = grid
        .iter()
        .map(|mirror| (mirror.span.extra, mirror.r#type))
        .collect::<HashMap<IVec2, MirrorType>>();
    let mut energized_cells: HashSet<IVec2> =
        HashSet::from([IVec2::new(-1, 0)]);
    let mut all_movement =
        vec![(Direction::East, IVec2::new(-1, 0))];
    let mut active_beams: Vec<(Direction, IVec2)> =
        vec![(Direction::East, IVec2::new(-1, 0))];

    loop {
        // dbg!("loop 1");

        if active_beams.is_empty() {
            break;
        }
        let mut next_beams: Vec<(Direction, IVec2)> =
            vec![];
        for (direction, current_position) in
            active_beams.iter()
        {
            // dbg!("loop2");
            let mut current_position = *current_position;
            loop {
                let new_position =
                    direction.step(&current_position);
                // todo match
                match march(
                    direction,
                    grid_map.get(&new_position),
                ) {
                    March::Empty => {
                        // dbg!("empty");
                        energized_cells
                            .insert(new_position);

                        current_position = new_position;
                        if !(0..column_count)
                            .contains(&new_position.x)
                            || !(0..row_count)
                                .contains(&new_position.y)
                        {
                            break;
                        }
                    }
                    March::Split(d1, d2) => {
                        // dbg!("split");
                        if all_movement
                            .iter()
                            .find(|item| {
                                item == &&(d1, new_position)
                            })
                            .is_none()
                        {
                            next_beams
                                .push((d1, new_position));
                            energized_cells
                                .insert(new_position);
                            all_movement
                                .push((d1, new_position));
                        }
                        if all_movement
                            .iter()
                            .find(|item| {
                                item == &&(d2, new_position)
                            })
                            .is_none()
                        {
                            next_beams
                                .push((d2, new_position));
                            energized_cells
                                .insert(new_position);
                            all_movement
                                .push((d2, new_position));
                        }

                        break;
                    }
                    March::Dir(d) => {
                        // dbg!(&d, new_position);
                        if all_movement
                            .iter()
                            .find(|item| {
                                item == &&(d, new_position)
                            })
                            .is_none()
                        {
                            next_beams
                                .push((d, new_position));
                            energized_cells
                                .insert(new_position);
                            all_movement
                                .push((d, new_position));
                        }
                        break;
                    }
                }
            }
        }
        active_beams = next_beams;
    }

    // print_grid(
    //     &energized_cells,
    //     &IVec2::new(column_count, row_count),
    // );
    // println!("{:?}", &energized_cells);
    Ok(energized_cells
        .iter()
        .filter(|position| {
            (0..column_count).contains(&position.x)
                && (0..row_count).contains(&position.y)
        })
        .count()
        .to_string())
}

enum March {
    Empty,
    Split(Direction, Direction),
    Dir(Direction),
}
fn march(
    direction: &Direction,
    mirror_type: Option<&MirrorType>,
) -> March {
    use Direction as D;
    use MirrorType as MT;
    match (direction, mirror_type) {
        (D::North, Some(MT::Backslash)) => {
            March::Dir(D::West)
        }
        (D::South, Some(MT::Backslash)) => {
            March::Dir(D::East)
        }
        (D::East, Some(MT::Backslash)) => {
            March::Dir(D::South)
        }
        (D::West, Some(MT::Backslash)) => {
            March::Dir(D::North)
        }

        (D::North, Some(MT::ForwardSlash)) => {
            March::Dir(D::East)
        }
        (D::South, Some(MT::ForwardSlash)) => {
            March::Dir(D::West)
        }
        (D::East, Some(MT::ForwardSlash)) => {
            March::Dir(D::North)
        }
        (D::West, Some(MT::ForwardSlash)) => {
            March::Dir(D::South)
        }

        (D::North, Some(MT::Horizontal))
        | (D::South, Some(MT::Horizontal)) => {
            March::Split(D::West, D::East)
        }

        (D::East, Some(MT::Vertical))
        | (D::West, Some(MT::Vertical)) => {
            March::Split(D::North, D::South)
        }

        (_, None)
        | (D::East, Some(MT::Horizontal))
        | (D::West, Some(MT::Horizontal))
        | (D::North, Some(MT::Vertical))
        | (D::South, Some(MT::Vertical)) => March::Empty,
    }
}

fn print_grid(map: &HashSet<IVec2>, boundaries: &IVec2) {
    for y in 0..boundaries.y {
        for x in 0..boundaries.x {
            match map.get(&IVec2::new(x, y)) {
                Some(_) => {
                    print!("#");
                }
                None => print!("."),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!("46", process(input)?);
        Ok(())
    }
}
