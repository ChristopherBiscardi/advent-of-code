use std::{
    fmt::Write as _,
    fs::File,
    io::Write as _,
    ops::{Range, RangeInclusive},
};

use glam::I64Vec2;
use itertools::{Itertools, MinMaxResult};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, hex_digit1, line_ending, space1,
    },
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};
use tracing::{info, span, Level};

use crate::custom_error::AocError;

#[derive(Debug)]
struct DigInstruction<'a> {
    direction: I64Vec2,
    count: i64,
    color: &'a str,
}

#[tracing::instrument(skip(input))]
fn dig_instruction(
    input: &str,
) -> IResult<&str, DigInstruction> {
    let (input, direction) = alt((
        complete::char('R').map(|_| I64Vec2::X),
        complete::char('L').map(|_| I64Vec2::NEG_X),
        complete::char('U').map(|_| I64Vec2::Y),
        complete::char('D').map(|_| I64Vec2::NEG_Y),
    ))(input)?;

    let (input, count) =
        delimited(space1, complete::i64, space1)(input)?;

    let (input, hex) = delimited(
        tag("(#"),
        hex_digit1,
        complete::char(')'),
    )(input)?;

    Ok((
        input,
        DigInstruction {
            direction,
            count,
            color: hex,
        },
    ))
}

#[tracing::instrument(skip(input))]
fn instructions(
    input: &str,
) -> IResult<&str, Vec<DigInstruction>> {
    separated_list1(line_ending, dig_instruction)(input)
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_, digs) =
        instructions(input).expect("should parse");

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
        assert_eq!("62", process(input)?);
        Ok(())
    }
}
