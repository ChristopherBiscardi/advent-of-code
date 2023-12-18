use std::{
    fmt::Write as _, fs::File, io::Write as _,
    ops::RangeInclusive,
};

use glam::I64Vec2;
use itertools::Itertools;
use nom::{
    bytes::complete::{take, take_until},
    character::complete::{self},
    multi::many1,
    sequence::terminated,
    IResult,
};
use tracing::{info, span, Level};

use crate::custom_error::AocError;

#[derive(Debug)]
struct DigInstruction {
    direction: I64Vec2,
    count: i64,
    // color: &'a str,
}

// 0 means R, 1 means D, 2 means L, and 3 means U.
#[tracing::instrument(skip(input))]
fn dig_instruction(
    input: &str,
) -> IResult<&str, DigInstruction> {
    // let (input, direction) = alt((
    //     complete::char('R').map(|_| I64Vec2::X),
    //     complete::char('L').map(|_| I64Vec2::NEG_X),
    //     complete::char('U').map(|_| I64Vec2::Y),
    //     complete::char('D').map(|_| I64Vec2::NEG_Y),
    // ))(input)?;

    let (input, _) = terminated(
        take_until("#"),
        complete::char('#'),
    )(input)?;
    let (input, hex) = take(5usize)(input)?;
    let (input, direction) = take(1usize)(input)?;

    let count =
        i64::from_str_radix(hex, 16).expect("should parse");
    let direction = match i64::from_str_radix(direction, 16)
        .expect("should parse")
    {
        0 => I64Vec2::X,
        1 => I64Vec2::NEG_Y,
        2 => I64Vec2::NEG_X,
        3 => I64Vec2::Y,
        _ => unreachable!("advent of code yay!"),
    };

    Ok((
        input,
        DigInstruction { direction, count },
    ))
}

#[tracing::instrument(skip(input))]
fn instructions(
    input: &str,
) -> IResult<&str, Vec<DigInstruction>> {
    many1(dig_instruction)(input)
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_, digs) =
        instructions(input).expect("should parse");
    // dbg!(&digs);
    let vertices = digs
        .iter()
        .scan(I64Vec2::new(0, 0), |state, next| {
            *state += next.direction * next.count;
            Some(*state)
        })
        .collect::<Vec<I64Vec2>>();

    let perimeter_length = vertices
        .iter()
        .tuple_windows()
        .map(|(one, two)| {
            let distance = (*two - *one).abs();
            distance.x + distance.y
        })
        .sum::<i64>()
        + {
            let one = vertices.iter().last().unwrap();
            let two = vertices.iter().next().unwrap();
            let distance = (*two - *one).abs();
            distance.x + distance.y
        };
    let area = ((vertices
        .iter()
        .tuple_windows()
        .map(|(one, two)| one.x * two.y - one.y * two.x)
        .sum::<i64>()
        + perimeter_length)
        / 2)
    .abs()
        + 1;

    Ok((area + perimeter_length).to_string())
}

#[allow(dead_code)]
fn write_grid(
    map: &[I64Vec2],
    y_bound: RangeInclusive<i64>,
    x_bound: RangeInclusive<i64>,
) {
    let mut s = String::new();

    for y in y_bound.rev() {
        for x in x_bound.clone() {
            match map.contains(&I64Vec2::new(x, y)) {
                true => {
                    write!(&mut s, "#").unwrap();
                }
                false => {
                    write!(&mut s, ".").unwrap();
                }
            }
        }
        writeln!(&mut s).unwrap();
    }

    let mut file = File::create("grid.txt").unwrap();
    file.write_all(s.as_bytes()).unwrap();
}

#[allow(dead_code)]
fn print_grid(
    map: &[I64Vec2],
    y_bound: RangeInclusive<i64>,
    x_bound: RangeInclusive<i64>,
) {
    for y in y_bound.rev() {
        for x in x_bound.clone() {
            match map.contains(&I64Vec2::new(x, y)) {
                true => {
                    print!("#");
                }
                false => print!("."),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!("952408144115", process(input)?);
        Ok(())
    }
}
