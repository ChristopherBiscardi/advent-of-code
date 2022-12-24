use std::{
    collections::HashMap, fmt::Display, fs::File, io::Write,
};

use glam::{IVec2, IVec3};
use itertools::Itertools;
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
use petgraph::{
    algo::dijkstra,
    dot::{Config, Dot},
    prelude::DiGraphMap,
};
use tracing::*;

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Wall,
    Blizzard(Vec<Direction>),
    Space,
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Debug)]
struct Field(HashMap<IVec2, Cell>);

impl Field {
    fn get_inner_dimensions(&self) -> IVec2 {
        let x_max =
            self.0.iter().map(|(v, _)| v.x).max().unwrap();
        let y_max =
            self.0.iter().map(|(v, _)| v.y).max().unwrap();
        IVec2::new(x_max - 1, y_max - 1)
    }
    fn move_blizzard(
        &self,
        new_field: &mut Field,
        position: &IVec2,
        direction: &Direction,
    ) {
        let movement = match direction {
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
            Direction::Up => IVec2::new(0, -1),
            Direction::Down => IVec2::new(0, 1),
        };
        let desired_position = *position + movement;
        match self.0.get(&(desired_position)) {
            Some(Cell::Wall) => {
                // dbg!(position, desired_position);
                // find exterior wall
                let wall_position = self
                    .0
                    .iter()
                    .find(|(ivec, cell)| {
                        let is_wall = cell == &&Cell::Wall;
                        let is_in_row_or_column =
                            match direction {
                                Direction::Left => {
                                    ivec.x > position.x
                                        && ivec.y
                                            == position.y
                                }
                                Direction::Right => {
                                    ivec.x < position.x
                                        && ivec.y
                                            == position.y
                                }
                                Direction::Up => {
                                    ivec.y > position.y
                                        && ivec.x
                                            == position.x
                                }
                                Direction::Down => {
                                    ivec.y < position.y
                                        && ivec.x
                                            == position.x
                                }
                            };
                        is_wall && is_in_row_or_column
                    })
                    .unwrap()
                    .0;
                let left_of_wall =
                    *wall_position + movement;
                // insert left of rightmost wall
                // dbg!(left_of_wall);
                new_field
                    .0
                    .entry(left_of_wall)
                    .and_modify(|cell| {
                        // dbg!(&cell);
                        if let Cell::Blizzard(directions) =
                            cell
                        {
                            directions.push(*direction);
                        };
                    })
                    .or_insert(Cell::Blizzard(vec![
                        *direction,
                    ]));
            }
            Some(_) => {
                // dbg!("here");
                new_field
                    .0
                    .entry(desired_position)
                    .and_modify(|cell| {
                        if let Cell::Blizzard(directions) =
                            cell
                        {
                            directions.push(*direction);
                        };
                    })
                    .or_insert(Cell::Blizzard(vec![
                        *direction,
                    ]));
            }
            None => {
                panic!("shouldn't be none")
            }
        }
    }
    fn step(&self) -> Self {
        let mut new_field: Field = Field(HashMap::new());
        let blizzards =
            self.0.iter().filter_map(|(pos, cell)| {
                match cell {
                    Cell::Wall => None,
                    Cell::Blizzard(direction) => {
                        Some((pos, direction))
                    }
                    Cell::Space => None,
                }
            });
        for (position, directions) in blizzards {
            for one in directions {
                self.move_blizzard(
                    &mut new_field,
                    position,
                    one,
                );
            }
        }
        // copy walls
        self.0.iter().for_each(|(pos, cell)| match cell {
            Cell::Wall => {
                new_field.0.insert(*pos, cell.clone());
            }
            _ => (),
        });
        let total_size =
            self.get_inner_dimensions() + IVec2::new(2, 2);
        // dbg!(total_size);
        for (y, x) in (0..total_size.y)
            .cartesian_product(0..total_size.x)
        {
            let pos = IVec2::new(x, y);
            if new_field.0.get(&pos).is_none() {
                new_field.0.insert(pos, Cell::Space);
            }
        }
        new_field
    }
}

impl Display for Field {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let x_max =
            self.0.iter().map(|(v, _)| v.x).max().unwrap();
        let value = self
            .0
            .iter()
            .sorted_by(|a, b| {
                (a.0.y, a.0.x).cmp(&(b.0.y, b.0.x))
            })
            .map(|(_vec, cell)| match cell {
                Cell::Wall => "#".to_string(),
                Cell::Blizzard(directions) => {
                    match directions.len() {
                        1 => match directions[0] {
                            Direction::Left => {
                                "<".to_string()
                            }
                            Direction::Right => {
                                ">".to_string()
                            }
                            Direction::Up => {
                                "^".to_string()
                            }
                            Direction::Down => {
                                "v".to_string()
                            }
                        },
                        n => n.to_string(),
                    }
                }

                Cell::Space => ".".to_string(),
            })
            .chunks(x_max as usize + 1)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
            .join("\n");
        write!(f, "{}", value)
    }
}

fn field(input: &str) -> IResult<&str, Field> {
    let mut it = iterator(
        input,
        terminated(
            many1(alt((
                char('.').map(|_| Cell::Space),
                char('#').map(|_| Cell::Wall),
                char('v').map(|_| {
                    Cell::Blizzard(vec![Direction::Down])
                }),
                char('^').map(|_| {
                    Cell::Blizzard(vec![Direction::Up])
                }),
                char('<').map(|_| {
                    Cell::Blizzard(vec![Direction::Left])
                }),
                char('>').map(|_| {
                    Cell::Blizzard(vec![Direction::Right])
                }),
            ))),
            alt((line_ending, eof)),
        ),
    );
    let cells = it
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter().enumerate().map(
                move |(x, cell)| {
                    (IVec2::new(x as i32, y as i32), cell)
                },
            )
        })
        .collect::<HashMap<IVec2, Cell>>();

    let res: IResult<_, _> = it.finish();
    Ok((res.unwrap().0, Field(cells)))
}

#[instrument(skip(input))]
pub fn process_part1(input: &str) -> String {
    let (_, mut field) = field(input).unwrap();
    // println!("{}", field);
    let field_size = field.get_inner_dimensions();
    let step_cycle_number = [
        (field_size.x..).step_by(field_size.x as usize),
        (field_size.y..).step_by(field_size.y as usize),
    ]
    .into_iter()
    .kmerge()
    .tuple_windows()
    .find(|(a, b)| a == b)
    .unwrap()
    .0;
    // dbg!(step_cycle_number);
    // println!(" ");
    let end_position = field
        .0
        .iter()
        .filter_map(|(pos, cell)| match cell {
            Cell::Wall => None,
            Cell::Blizzard(_) => None,
            Cell::Space => Some(pos),
        })
        .max_by(
            |IVec2 { x: x1, y: y1 },
             IVec2 { x: x2, y: y2 }| {
                (y1, x1).cmp(&(y2, x2))
            },
        )
        .unwrap()
        .clone();
    let end_edge_cell =
        (end_position.x, end_position.y, i32::MAX);
    // TODO: calculate (1,0,0) starting position
    let mut edges: Vec<((i32, i32, i32), (i32, i32, i32))> =
        vec![];
    for i in 0..(step_cycle_number) {
        let next_field = field.step();
        let origin_spaces =
            field.0.iter().filter_map(|(pos, cell)| {
                match cell {
                    Cell::Wall => None,
                    Cell::Blizzard(_) => None,
                    Cell::Space => Some(pos),
                }
            });
        // if let Some(position) =
        //     origin_spaces.clone().find(|position| {
        //         position
        //             == &&IVec2::new(
        //                 end_edge_cell.0,
        //                 end_edge_cell.1 - 1,
        //             )
        //     })
        // {
        //     edges.push((
        //         (position.x, position.y, i),
        //         end_edge_cell,
        //     ));
        // };
        for origin_position in origin_spaces {
            // println!("{:?}", origin_position);

            let possible_next_positions = vec![
                (-1, 0),
                (0, 0),
                (1, 0),
                (0, -1),
                (0, 1),
            ]
            .into_iter()
            .filter_map(|(y, x)| {
                let next_position =
                    IVec2::new(x, y) + *origin_position;
                if let Some(Cell::Space) =
                    next_field.0.get(&next_position)
                {
                    Some(next_position)
                } else {
                    None
                }
            });

            for pos in possible_next_positions {
                edges.push((
                    (
                        origin_position.x,
                        origin_position.y,
                        i,
                    ),
                    (
                        pos.x,
                        pos.y,
                        if step_cycle_number == i + 1 {
                            // dbg!("step_cycle_number");
                            0
                        } else {
                            i + 1
                        },
                    ),
                ))
            }
        }
        field = next_field;
    }
    // construct graph
    let graph =
        DiGraphMap::<(i32, i32, i32), ()>::from_edges(
            edges,
        );

    // let dot =
    //     Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    // // println!(
    // //     "{:?}",
    // //     Dot::with_config(&graph, &[Config::EdgeNoLabel])
    // // );
    // let mut file = File::create("graph.dot").unwrap();
    // file.write_all(format!("{:?}", dot).as_bytes())
    //     .unwrap();
    // connect final cycle to first cycle
    let result = dijkstra(
        &graph,
        (1, 0, 0),
        None, // Some(end_edge_cell),
        |_| 1,
    );
    result
        .iter()
        .filter_map(|(end, value)| {
            if end.0 == end_position.x
                && end.1 == end_position.y
            {
                Some(value)
            } else {
                None
            }
        })
        .min()
        .unwrap()
        .to_string()
}

#[instrument(skip(input))]
pub fn process_part2(input: &str) -> String {
    let (_, mut field) = field(input).unwrap();
    // println!("{}", field);
    let field_size = field.get_inner_dimensions();
    let step_cycle_number = [
        (field_size.x..).step_by(field_size.x as usize),
        (field_size.y..).step_by(field_size.y as usize),
    ]
    .into_iter()
    .kmerge()
    .tuple_windows()
    .find(|(a, b)| a == b)
    .unwrap()
    .0;
    // dbg!(step_cycle_number);
    // println!(" ");
    let end_position = field
        .0
        .iter()
        .filter_map(|(pos, cell)| match cell {
            Cell::Wall => None,
            Cell::Blizzard(_) => None,
            Cell::Space => Some(pos),
        })
        .max_by(
            |IVec2 { x: x1, y: y1 },
             IVec2 { x: x2, y: y2 }| {
                (y1, x1).cmp(&(y2, x2))
            },
        )
        .unwrap()
        .clone();
    let end_edge_cell =
        (end_position.x, end_position.y, i32::MAX);
    // TODO: calculate (1,0,0) starting position
    let mut edges: Vec<((i32, i32, i32), (i32, i32, i32))> =
        vec![];
    for i in 0..(step_cycle_number) {
        let next_field = field.step();
        let origin_spaces =
            field.0.iter().filter_map(|(pos, cell)| {
                match cell {
                    Cell::Wall => None,
                    Cell::Blizzard(_) => None,
                    Cell::Space => Some(pos),
                }
            });
        // if let Some(position) =
        //     origin_spaces.clone().find(|position| {
        //         position
        //             == &&IVec2::new(
        //                 end_edge_cell.0,
        //                 end_edge_cell.1 - 1,
        //             )
        //     })
        // {
        //     edges.push((
        //         (position.x, position.y, i),
        //         end_edge_cell,
        //     ));
        // };
        for origin_position in origin_spaces {
            // println!("{:?}", origin_position);

            let possible_next_positions = vec![
                (-1, 0),
                (0, 0),
                (1, 0),
                (0, -1),
                (0, 1),
            ]
            .into_iter()
            .filter_map(|(y, x)| {
                let next_position =
                    IVec2::new(x, y) + *origin_position;
                if let Some(Cell::Space) =
                    next_field.0.get(&next_position)
                {
                    Some(next_position)
                } else {
                    None
                }
            });

            for pos in possible_next_positions {
                edges.push((
                    (
                        origin_position.x,
                        origin_position.y,
                        i,
                    ),
                    (
                        pos.x,
                        pos.y,
                        if step_cycle_number == i + 1 {
                            // dbg!("step_cycle_number");
                            0
                        } else {
                            i + 1
                        },
                    ),
                ))
            }
        }
        field = next_field;
    }
    // construct graph
    let graph =
        DiGraphMap::<(i32, i32, i32), ()>::from_edges(
            edges,
        );

    // let dot =
    //     Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    // // println!(
    // //     "{:?}",
    // //     Dot::with_config(&graph, &[Config::EdgeNoLabel])
    // // );
    // let mut file = File::create("graph.dot").unwrap();
    // file.write_all(format!("{:?}", dot).as_bytes())
    //     .unwrap();
    // connect final cycle to first cycle
    let result = dijkstra(
        &graph,
        (1, 0, 0),
        None, // Some(end_edge_cell),
        |_| 1,
    );
    let to_goal = result
        .iter()
        .filter(|(end, value)| {
            if end.0 == end_position.x
                && end.1 == end_position.y
            {
                true
            } else {
                false
            }
        })
        .min_by_key(|(_, value)| *value)
        .unwrap();
    dbg!(to_goal);

    let result = dijkstra(
        &graph,
        *to_goal.0,
        None, // Some(end_edge_cell),
        |_| 1,
    );
    let back_to_camp = result
        .iter()
        .filter(|(end, value)| {
            if end.0 == 1 && end.1 == 0 {
                true
            } else {
                false
            }
        })
        .min_by_key(|(_, value)| *value)
        .unwrap();
    dbg!(back_to_camp);
    let result = dijkstra(
        &graph,
        *back_to_camp.0,
        None, // Some(end_edge_cell),
        |_| 1,
    );
    let back_to_goal = result
        .iter()
        .filter(|(end, value)| {
            if end.0 == end_position.x
                && end.1 == end_position.y
            {
                true
            } else {
                false
            }
        })
        .min_by_key(|(_, value)| *value)
        .unwrap();
    dbg!(back_to_goal);
    (to_goal.1 + back_to_camp.1 + back_to_goal.1)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    #[ignore]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part1(INPUT), "18");
    }

    #[test]
    fn part2_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part2(INPUT), "54");
    }
}
