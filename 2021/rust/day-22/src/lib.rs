use core::fmt;
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    iter::{self, Sum},
    ops::Add,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{
        self, alpha1, anychar, newline, one_of, u32,
    },
    combinator::opt,
    multi::{
        length_value, many0, many1, many_m_n,
        separated_list1,
    },
    sequence::{
        pair, preceded, separated_pair, terminated,
    },
    IResult,
};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
#[derive(Debug)]
enum Command {
    On((Point, Point)),
    Off((Point, Point)),
}
fn command(input: &str) -> IResult<&str, Command> {
    let (input, action) =
        alt((tag("on"), tag("off")))(input)?;
    let (input, _) = tag(" x=")(input)?;
    let (input, (low_x, high_x)) = separated_pair(
        complete::i32,
        tag(".."),
        complete::i32,
    )(input)?;
    let (input, _) = tag(",y=")(input)?;

    let (input, (low_y, high_y)) = separated_pair(
        complete::i32,
        tag(".."),
        complete::i32,
    )(input)?;
    let (input, _) = tag(",z=")(input)?;
    let (input, (low_z, high_z)) = separated_pair(
        complete::i32,
        tag(".."),
        complete::i32,
    )(input)?;

    let cmd = match action {
        "on" => Command::On((
            Point {
                x: low_x,
                y: low_y,
                z: low_z,
            },
            Point {
                x: high_x,
                y: high_y,
                z: high_z,
            },
        )),
        "off" => Command::Off((
            Point {
                x: low_x,
                y: low_y,
                z: low_z,
            },
            Point {
                x: high_x,
                y: high_y,
                z: high_z,
            },
        )),
        _ => {
            panic!("action")
        }
    };
    Ok((input, cmd))
}
fn puzzle_input(
    input: &str,
) -> IResult<&str, Vec<Command>> {
    let (input, commands) =
        separated_list1(newline, command)(input)?;
    Ok((input, commands))
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Bulb {
    On,
    Off,
}
fn tick(
    grid: &mut BTreeMap<Point, Bulb>,
    command: &Command,
) {
    let (low, high, op) = match command {
        Command::On((point_low, point_high)) => {
            (point_low, point_high, Bulb::On)
        }
        Command::Off((point_low, point_high)) => {
            (point_low, point_high, Bulb::Off)
        }
    };

    for point in (low.x..=high.x)
        .cartesian_product(low.y..=high.y)
        .cartesian_product(low.z..=high.z)
        .map(|((x, y), z)| Point { x, y, z })
    {
        grid.entry(point)
            .and_modify(|value| *value = op.clone())
            .or_insert(op.clone());
    }
}
pub fn process_part1(input: &str) -> usize {
    let (_, commands) =
        puzzle_input(input).expect("input to be valid");
    let starting_grid = BTreeMap::new();
    let valid_range = -50..=50;
    let final_grid = commands
        .iter()
        .filter(|cmd| {
            let (a, b) = match cmd {
                Command::On((a, b)) => (a, b),
                Command::Off((a, b)) => (a, b),
            };
            vec![a.x, a.y, a.z, b.x, b.y, b.z]
                .iter()
                .all(|val| valid_range.contains(val))
        })
        .fold(starting_grid, |mut grid, command| {
            tick(&mut grid, &command);
            // println!("\n\nstep");
            // for (point, bulb) in grid.iter() {
            //     println!(
            //         "({},{},{}, {:?})",
            //         point.x, point.y, point.z, bulb
            //     );
            // }
            grid
        });
    final_grid
        .iter()
        .filter(|(_, &v)| v == Bulb::On)
        .count()
}

pub fn process_part2(input: &str) -> usize {
    let (_, commands) =
        puzzle_input(input).expect("input to be valid");
    let starting_grid = BTreeMap::new();
    // let valid_range = -50..=50;
    let final_grid = commands
        .iter()
        // .filter(|cmd| {
        //     let (a, b) = match cmd {
        //         Command::On((a, b)) => (a, b),
        //         Command::Off((a, b)) => (a, b),
        //     };
        //     vec![a.x, a.y, a.z, b.x, b.y, b.z]
        //         .iter()
        //         .all(|val| valid_range.contains(val))
        // })
        .fold(starting_grid, |mut grid, command| {
            tick(&mut grid, &command);
            grid
        });
    final_grid
        .iter()
        .filter(|(_, &v)| v == Bulb::On)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &'static str =
        "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
    const INPUT: &'static str =
        include_str!("./test-input.txt");

    #[test]
    fn part1_test_small_input() {
        assert_eq!(39, process_part1(INPUT_1));
    }

    // #[test]
    // fn part1_test_demo_data() {
    //     assert_eq!(739785, process_part1(INPUT_1));
    // }

    // #[test]
    // fn part2_test_demo_data() {
    //     assert_eq!(3351, process_part2(INPUT));
    // }
}
