use std::collections::{HashMap, HashSet};

use crate::custom_error::AocError;
use glam::{IVec2, IVec3, Vec3Swizzles};
use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

/// parses an IVec3 from `1,2,3`.
fn brick_end(input: &str) -> IResult<&str, IVec3> {
    let (input, x) = complete::i32(input)?;
    let (input, _) = complete::char(',')(input)?;
    let (input, y) = complete::i32(input)?;
    let (input, _) = complete::char(',')(input)?;
    let (input, z) = complete::i32(input)?;
    Ok((input, IVec3::new(x, y, z)))
}

#[derive(Debug, Eq, PartialEq)]
pub struct Brick {
    pub cubes: HashSet<IVec3>,
}
fn brick(input: &str) -> IResult<&str, Brick> {
    let (input, (start, end)) = separated_pair(
        brick_end,
        complete::char('~'),
        brick_end,
    )(input)?;

    let cubes =
        [start.x..=end.x, start.y..=end.y, start.z..=end.z]
            .into_iter()
            .multi_cartesian_product()
            .map(|cube| {
                IVec3::new(cube[0], cube[1], cube[2])
            })
            .collect();
    Ok((input, Brick { cubes }))
}
pub fn parse_bricks(
    input: &str,
) -> IResult<&str, Vec<Brick>> {
    separated_list1(line_ending, brick)(input)
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_, bricks) =
        parse_bricks(input).expect("should parse");

    let sorted_bricks = bricks.iter().sorted_by(|a, b| {
        a.cubes
            .iter()
            .map(|cube| cube.z)
            .min()
            .unwrap()
            .cmp(
                &b.cubes
                    .iter()
                    .map(|cube| cube.z)
                    .min()
                    .unwrap(),
            )
    });

    let sb =
        sorted_bricks.into_iter().collect::<Vec<&Brick>>();
    let (fallen_bricks, _) = fall(&sb);

    let total_falling_bricks = &fallen_bricks
        .iter()
        .map(|brick| {
            let new_bricks = &fallen_bricks
                .iter()
                .filter(|b| b != &brick)
                .collect::<Vec<&Brick>>();
            let (_, fall_count) = fall(&new_bricks);
            fall_count
        })
        .sum::<i32>();

    Ok(total_falling_bricks.to_string())
}

fn fall(sorted_bricks: &[&Brick]) -> (Vec<Brick>, i32) {
    let (fallen_bricks, num_falling_bricks) =
        sorted_bricks.into_iter().fold(
            (vec![], 0),
            |(mut acc, mut fall_count): (
                Vec<Brick>,
                i32,
            ),
             brick| {
                let min_cubes = brick
                    .cubes
                    .iter()
                    .min_set_by_key(|cube| cube.z);
                let min_cubes_xy: Vec<IVec2> = min_cubes
                    .iter()
                    .map(|cube| cube.xy())
                    .collect();

                let max_z_underneath = acc
                    .iter()
                    .flat_map(|brick| brick.cubes.iter())
                    .filter_map(|cube| {
                        min_cubes_xy
                            .contains(&cube.xy())
                            .then_some(cube.z)
                    })
                    .max()
                    .unwrap_or(0);
                let landing_z = max_z_underneath + 1;
                let brick_z =
                    min_cubes.iter().next().unwrap().z;
                let diff = brick_z - landing_z;
                if diff >= 1 {
                    fall_count += 1;
                }
                let new_cubes = brick
                    .cubes
                    .iter()
                    .map(|cube| {
                        IVec3::new(
                            cube.x,
                            cube.y,
                            cube.z - diff,
                        )
                    })
                    .collect();
                acc.push(Brick { cubes: new_cubes });
                (acc, fall_count)
            },
        );
    (fallen_bricks, num_falling_bricks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
