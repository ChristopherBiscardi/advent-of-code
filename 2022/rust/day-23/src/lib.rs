use glam::IVec2;
use itertools::Itertools;
use itertools::MinMaxResult::*;
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
use std::collections::{HashMap, HashSet};
use tracing::*;

fn map(input: &str) -> IResult<&str, HashSet<IVec2>> {
    let mut it = iterator(
        input,
        terminated(
            many1(one_of(".#")),
            alt((line_ending, eof)),
        ),
    );
    let elves = it
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter().enumerate().filter_map(
                move |(x, c)| match c {
                    '.' => None,
                    '#' => {
                        Some(IVec2::new(x as i32, y as i32))
                    }
                    _ => panic!("unknown char"),
                },
            )
        })
        .collect::<HashSet<IVec2>>();

    let res: IResult<_, _> = it.finish();
    Ok((res.unwrap().0, elves))
}

#[instrument(skip(input))]
pub fn process_part1(input: &str) -> String {
    let (_, mut field) = map(input).unwrap();
    let checks = vec![
        [
            IVec2::new(-1, -1),
            IVec2::new(0, -1),
            IVec2::new(1, -1),
        ],
        [
            IVec2::new(-1, 1),
            IVec2::new(0, 1),
            IVec2::new(1, 1),
        ],
        [
            IVec2::new(-1, -1),
            IVec2::new(-1, 0),
            IVec2::new(-1, 1),
        ],
        [
            IVec2::new(1, -1),
            IVec2::new(1, 0),
            IVec2::new(1, 1),
        ],
    ];
    let checks_iter = checks.iter().cycle();
    // println!("\nInitial State");
    // print_field(&field);

    for i in 0..10 {
        let local_checks =
            checks_iter.clone().skip(i).take(4);
        // for check in local_checks.clone() {
        //     println!("check {:?}", check);
        // }

        let mut proposed_moves: HashMap<IVec2, Vec<IVec2>> =
            HashMap::new();

        for elf in field.iter() {
            // check for all empty around elf
            if local_checks
                .clone()
                .flat_map(|v| {
                    v.iter().map(|vec| *vec + *elf)
                })
                .unique()
                .all(|value| field.get(&value).is_none())
            {
                proposed_moves
                    .entry(*elf)
                    // .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
                continue;
            };
            // check for a possible move in a direction
            let possible_move =
                local_checks.clone().find_map(|checks| {
                    let output = checks
                        .iter()
                        .all(|position| {
                            field
                                .get(&(*position + *elf))
                                .is_none()
                        })
                        .then_some(checks[1] + *elf);
                    // dbg!(output);
                    output
                });
            if let Some(r#move) = possible_move {
                proposed_moves
                    .entry(r#move)
                    .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
            } else {
                proposed_moves
                    .entry(*elf)
                    // .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
            }
        }
        // proposed_moves.iter().for_each(|(key, value)| {
        //     println!("{}{:?}", key, value);
        // });

        field = proposed_moves
            .into_iter()
            .flat_map(
                |(desired_position, elves_to_move)| {
                    if elves_to_move.len() == 1 {
                        vec![desired_position]
                    } else {
                        elves_to_move
                    }
                },
            )
            .collect::<HashSet<IVec2>>();
        // println!("Round {}", i + 1);
        // print_field(&field);
    }
    let minmax_x = field.iter().map(|v| v.x).minmax();
    let minmax_y = field.iter().map(|v| v.y).minmax();
    let (MinMax(x1,x2), MinMax(y1,y2)) = (minmax_x,minmax_y) else {
        panic!("");
    };

    let min_box_size = (x2 - x1 + 1) * (y2 - y1 + 1);
    (min_box_size as usize - field.len()).to_string()
}

fn print_field(field: &HashSet<IVec2>) {
    let minmax_x = field.iter().map(|v| v.x).minmax();
    let minmax_y = field.iter().map(|v| v.y).minmax();
    let (MinMax(x1,x2), MinMax(y1,y2)) = (minmax_x,minmax_y) else {
        panic!("");
    };
    let output = (y1..=y2)
        .cartesian_product(x1..=x2)
        .map(
            |(y, x)| match field.get(&IVec2 { x, y }) {
                Some(_) => "#",
                None => ".",
            },
        )
        .chunks((x2 - x1 + 1) as usize)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n");
    println!("{}", output);
}

#[instrument(skip(input))]
pub fn process_part2(input: &str) -> String {
    let (_, mut field) = map(input).unwrap();
    let checks = vec![
        [
            IVec2::new(-1, -1),
            IVec2::new(0, -1),
            IVec2::new(1, -1),
        ],
        [
            IVec2::new(-1, 1),
            IVec2::new(0, 1),
            IVec2::new(1, 1),
        ],
        [
            IVec2::new(-1, -1),
            IVec2::new(-1, 0),
            IVec2::new(-1, 1),
        ],
        [
            IVec2::new(1, -1),
            IVec2::new(1, 0),
            IVec2::new(1, 1),
        ],
    ];
    let checks_iter = checks.iter().cycle();
    // println!("\nInitial State");
    // print_field(&field);

    let mut rounds = 0;

    for i in 0.. {
        let local_checks =
            checks_iter.clone().skip(i).take(4);
        // for check in local_checks.clone() {
        //     println!("check {:?}", check);
        // }

        let mut proposed_moves: HashMap<IVec2, Vec<IVec2>> =
            HashMap::new();

        for elf in field.iter() {
            // check for all empty around elf
            if local_checks
                .clone()
                .flat_map(|v| {
                    v.iter().map(|vec| *vec + *elf)
                })
                .unique()
                .all(|value| field.get(&value).is_none())
            {
                proposed_moves
                    .entry(*elf)
                    // .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
                continue;
            };
            // check for a possible move in a direction
            let possible_move =
                local_checks.clone().find_map(|checks| {
                    let output = checks
                        .iter()
                        .all(|position| {
                            field
                                .get(&(*position + *elf))
                                .is_none()
                        })
                        .then_some(checks[1] + *elf);
                    // dbg!(output);
                    output
                });
            if let Some(r#move) = possible_move {
                proposed_moves
                    .entry(r#move)
                    .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
            } else {
                proposed_moves
                    .entry(*elf)
                    // .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
            }
        }
        // proposed_moves.iter().for_each(|(key, value)| {
        //     println!("{}{:?}", key, value);
        // });

        let new_field = proposed_moves
            .into_iter()
            .flat_map(
                |(desired_position, elves_to_move)| {
                    if elves_to_move.len() == 1 {
                        vec![desired_position]
                    } else {
                        elves_to_move
                    }
                },
            )
            .collect::<HashSet<IVec2>>();
        if field == new_field {
            rounds = i;
            break;
        } else {
            field = new_field
        }
        // println!("Round {}", i + 1);
        // print_field(&field);
    }
    // let minmax_x = field.iter().map(|v| v.x).minmax();
    // let minmax_y = field.iter().map(|v| v.y).minmax();
    // let (MinMax(x1,x2), MinMax(y1,y2)) = (minmax_x,minmax_y) else {
    //     panic!("");
    // };

    // let min_box_size = (x2 - x1 + 1) * (y2 - y1 + 1);
    // (min_box_size as usize - field.len()).to_string()
    (rounds + 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    #[ignore]
    fn part1_test_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(
            process_part1(
                ".....
..##.
..#..
.....
..##.
....."
            ),
            "110"
        );
    }
    #[test]
    #[ignore]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part1(INPUT), "110");
    }

    #[test]
    fn part2_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part2(INPUT), "20");
    }
}
