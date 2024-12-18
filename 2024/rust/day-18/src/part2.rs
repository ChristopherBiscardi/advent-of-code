use glam::IVec2;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use owo_colors::OwoColorize;
use pathfinding::prelude::*;
use std::{
    fmt::{self, Write},
    ops::Not,
};

const GRID_SIZE: IVec2 = if cfg!(test) {
    IVec2::splat(6)
} else {
    IVec2::splat(70)
};

const DIRECTIONS: [IVec2; 4] =
    [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, falling_bytes) =
        parse(input).map_err(|e| {
            miette::miette!("parse failed {}", e)
        })?;

    let mut last_visited = vec![];
    let mut n = (GRID_SIZE - 2).x as usize;
    let result = loop {
        // println!("{n}");
        let end = falling_bytes.len().min(n);
        // dbg!(&falling_bytes[0..end]);
        let start_node = IVec2::ZERO;
        let mut positions_visited = vec![];
        let result = dijkstra(
            &start_node,
            |position| {
                DIRECTIONS
                    .iter()
                    .filter_map(|dir| {
                        let next_pos = position + dir;
                        if !((0..=GRID_SIZE.x)
                            .contains(&next_pos.x)
                            && (0..=GRID_SIZE.y)
                                .contains(&next_pos.y))
                        {
                            return None;
                        }
                        if falling_bytes[0..end]
                            .contains(&next_pos)
                            .not()
                        {
                            positions_visited
                                .push(next_pos);
                            Some((next_pos, 1usize))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            },
            |&p| p == GRID_SIZE,
        );
        if result.is_none() {
            break &falling_bytes[n - 1];
        }
        last_visited = positions_visited;
        n += 1;
    };
    println!(
        "{}",
        debug_grid(
            &falling_bytes[0..(n - 1)],
            // &positions_visited[..]
            // &result.as_ref().unwrap().0[..]
            last_visited.as_slice(),
            &result
        )
        .unwrap()
    );

    Ok(format!("{},{}", result.x, result.y))
}

fn parse(input: &str) -> IResult<&str, Vec<IVec2>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::i32,
            tag(","),
            complete::i32,
        )
        .map(|(x, y)| IVec2::new(x, y)),
    )(input)
}

fn debug_grid(
    objects: &[IVec2],
    visited: &[IVec2],
    ns: &IVec2,
) -> Result<String, fmt::Error> {
    let map_size = GRID_SIZE;

    let mut output = String::new();
    writeln!(&mut output, "")?;
    for y in 0..=map_size.y {
        for x in 0..=map_size.x {
            let pos = IVec2::new(x, y);
            if ns == &pos {
                write!(&mut output, "{}", "X".white())?;
                continue;
            }
            match (
                objects.contains(&pos),
                visited.contains(&pos),
            ) {
                (true, false) => {
                    write!(&mut output, "{}", "#".red())?;
                }
                (false, true) => {
                    write!(&mut output, "{}", "O".green())?;
                }
                (false, false) => {
                    write!(&mut output, "{}", ".".black())?;
                }
                _ => unreachable!(""),
            }
        }
        writeln!(&mut output)?;
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!("22", process(input)?);
        Ok(())
    }
}
