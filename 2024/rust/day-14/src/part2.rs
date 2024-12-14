use glam::IVec2;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
use std::{collections::HashSet, fmt, io::Write};
use std::{fmt::Write as _, fs::File};

const MAP_SIZE: IVec2 = if cfg!(test) {
    IVec2::new(11, 7)
} else {
    IVec2::new(101, 103)
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, mut robots) = parse(input).map_err(|e| {
        miette::miette!("parse failed {}", e)
    })?;

    // debug_grid(&robots);
    let mut i = 0;
    let last_step = loop {
        for robot in robots.iter_mut() {
            robot.position = (robot.position
                + robot.velocity)
                .rem_euclid(MAP_SIZE);
        }
        i += 1;
        if tree_test(&robots) {
            break i;
        }
    };

    // let mut file =
    //     File::create("output.txt").map_err(|e| {
    //         miette::miette!(
    //             "failed to create output.txt {:?}",
    //             e
    //         )
    //     })?;
    // let output = debug_grid(&robots).map_err(|e| {
    //     miette::miette!(
    //         "debug failed to write to output, {:?}",
    //         e
    //     )
    // })?;
    // file.write_all(output.as_bytes()).map_err(|e| {
    //     miette::miette!("failed to write to file {:?}", e)
    // })?;

    Ok(last_step.to_string())
}

fn tree_test(robots: &[Robot]) -> bool {
    robots
        .iter()
        .map(|Robot { position, .. }| position)
        .all_unique()
    // let mut map: HashSet<&IVec2> = HashSet::default();
    // for Robot { position, .. } in robots {
    //     match map.contains(position) {
    //         true => {
    //             return false;
    //         }
    //         false => {
    //             map.insert(position);
    //         }
    //     }
    // }
    // true
}

#[allow(dead_code)]
fn debug_grid(
    robots: &[Robot],
) -> Result<String, fmt::Error> {
    let mut output = String::new();
    writeln!(&mut output, "")?;
    for y in 0..MAP_SIZE.y {
        for x in 0..MAP_SIZE.x {
            match robots
                .iter()
                .filter(|Robot { position, .. }| {
                    position.x == x && position.y == y
                })
                .count()
            {
                0 => {
                    write!(&mut output, ".")?;
                }
                n => {
                    write!(&mut output, "{}", n)?;
                }
            }
        }
        writeln!(&mut output)?;
    }
    Ok(output)
}

#[derive(Debug)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

fn parse_ivec2(input: &str) -> IResult<&str, IVec2> {
    let (input, (x, y)) = separated_pair(
        complete::i32,
        tag(","),
        complete::i32,
    )(input)?;
    Ok((input, IVec2::new(x, y)))
}

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(
        line_ending,
        separated_pair(
            preceded(tag("p="), parse_ivec2),
            space1,
            preceded(tag("v="), parse_ivec2),
        )
        .map(|(position, velocity)| Robot {
            position,
            velocity,
        }),
    )(input)
}
