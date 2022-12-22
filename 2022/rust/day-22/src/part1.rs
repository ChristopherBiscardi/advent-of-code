use core::fmt::Debug;
use glam::{IVec2, UVec2};
use itertools::Itertools;
use itertools::Position::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    character::complete::{
        alpha1, char, line_ending, one_of, u32,
    },
    combinator::{eof, iterator},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    *,
};
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    fs::File,
    io::Write,
};
use tracing::*;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
enum Turn {
    Left,
    Right,
}
#[derive(Debug)]
enum Move {
    Paces(u32),
    Turn(Turn),
}
impl Direction {
    fn turn(self, turn: &Turn) -> Direction {
        use Direction::*;
        match (self, turn) {
            (Up, Turn::Left) => Left,
            (Up, Turn::Right) => Right,
            (Down, Turn::Left) => Right,
            (Down, Turn::Right) => Left,
            (Left, Turn::Left) => Down,
            (Left, Turn::Right) => Up,
            (Right, Turn::Left) => Up,
            (Right, Turn::Right) => Down,
        }
    }
}
fn moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(alt((
        u32.map(|num| Move::Paces(num)),
        alt((
            char('L').map(|_| Move::Turn(Turn::Left)),
            char('R').map(|_| Move::Turn(Turn::Right)),
        )),
    )))(input)
}
#[derive(Debug)]
enum Cell {
    Space,
    Wall,
}

#[derive(Debug)]
struct Field(HashMap<UVec2, Cell>);

impl Field {
    fn get_row(
        &self,
        target_y: u32,
    ) -> Vec<(&UVec2, &Cell)> {
        self.0
            .iter()
            .filter(|(UVec2 { y, .. }, _cell)| {
                y == &target_y
            })
            .sorted_by(|(vec_a, _), (vec_b, _)| {
                vec_a.x.cmp(&vec_b.x)
            })
            .collect()
    }
    fn get_column(
        &self,
        target_x: u32,
    ) -> Vec<(&UVec2, &Cell)> {
        self.0
            .iter()
            .filter(|(UVec2 { x, .. }, _cell)| {
                x == &target_x
            })
            .sorted_by(|(vec_a, _), (vec_b, _)| {
                vec_a.y.cmp(&vec_b.y)
            })
            .collect()
    }
}

impl Display for Field {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let field_string = self
            .0
            .iter()
            .sorted_by(|(UVec2 { x: x1, y: y1 }, _), (UVec2 { x: x2, y: y2 }, _)| {
                (y1, x1).cmp(&(y2, x2))
            })
            .group_by(|(UVec2 { x: _, y }, _)| *y)
            .into_iter()
            .map(|(_y, xs)| {
                let mut padding = "".to_string();
                let line: String = xs
                    .into_iter()
                    .with_position()
                    .map(|position| {
                        let cell = match position {
                            First((UVec2 { x, y: _ }, cell)) => {
                                padding = " ".repeat(*x as usize);
                                cell
                            }
                            Middle((_, cell)) => cell,
                            Last((_, cell)) => cell,
                            Only((_, _cell)) => {
                                todo!()
                            }
                        };
                        match cell {
                            Cell::Space => ".",
                            Cell::Wall => "#",
                        }
                    })
                    .collect();
                format!("{padding}{line}")
            })
            .join("\n");

        write!(f, "{}", field_string)
    }
}

fn map(input: &str) -> IResult<&str, Field> {
    let mut it = iterator(
        input,
        terminated(many1(one_of(" .#")), line_ending),
    );

    let parsed: HashMap<UVec2, Cell> = it
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter().enumerate().filter_map(
                move |(x, cell)| match cell {
                    ' ' => None,
                    '.' => Some((
                        UVec2::new(x as u32, y as u32),
                        Cell::Space,
                    )),
                    '#' => Some((
                        UVec2::new(x as u32, y as u32),
                        Cell::Wall,
                    )),
                    _ => panic!("invalid character"),
                },
            )
        })
        .collect();

    let res: IResult<_, _> = it.finish();
    res.map(|(input, _)| (input, Field(parsed)))
}
fn map_and_moves(
    input: &str,
) -> IResult<&str, (Field, Vec<Move>)> {
    separated_pair(map, line_ending, moves)(input)
}

fn pace<'a>(
    current_position: &mut UVec2,
    paces_to_move: u32,
    positions: impl Iterator<Item = &'a (&'a UVec2, &'a Cell)>
        + Clone,
) {
    let current_index = positions
        .clone()
        .position(|(vec, _)| vec == &current_position)
        .unwrap();
    let mut it = positions.cycle();
    it.nth(current_index);

    for _ in 1..=paces_to_move {
        let next_cell = it.next().unwrap();
        if let Cell::Wall = next_cell.1 {
            break;
        } else {
            current_position.x = next_cell.0.x;
            current_position.y = next_cell.0.y;
        }
    }
}

#[instrument(skip(input))]
pub fn process_part1(input: &str) -> String {
    let (_, (field, moves)) = map_and_moves(input).unwrap();
    // dbg!(map, moves);
    let mut facing = Direction::Right;
    let starting_position = field
        .0
        .iter()
        .sorted_by(
            |(UVec2 { x: x1, y: y1 }, _),
             (UVec2 { x: x2, y: y2 }, _)| {
                (y1, x1).cmp(&(y2, x2))
            },
        )
        .next()
        .unwrap();
    let mut current_position = *starting_position.0;
    for m in moves {
        match m {
            Move::Paces(paces_to_move) => {
                match facing {
                    Direction::Up => {
                        // dbg!(current_position);
                        let x = current_position.x;
                        pace(
                            &mut current_position,
                            paces_to_move,
                            field
                                .get_column(x)
                                .iter()
                                .rev(),
                        );
                        // dbg!(current_position);
                    }
                    Direction::Down => {
                        let x = current_position.x;
                        pace(
                            &mut current_position,
                            paces_to_move,
                            field.get_column(x).iter(),
                        );
                    }
                    Direction::Left => {
                        let y = current_position.y;
                        pace(
                            &mut current_position,
                            paces_to_move,
                            field.get_row(y).iter().rev(),
                        );
                    }
                    Direction::Right => {
                        let y = current_position.y;
                        pace(
                            &mut current_position,
                            paces_to_move,
                            field.get_row(y).iter(),
                        );
                    }
                };
            }
            Move::Turn(turn) => {
                facing = facing.turn(&turn);
            }
        }
    }
    (1000 * (current_position.y + 1)
        + 4 * (current_position.x + 1)
        + match facing {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        })
    .to_string()
}
