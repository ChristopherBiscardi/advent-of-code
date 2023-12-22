use std::collections::HashMap;

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

#[derive(Debug)]
pub struct Brick {
    pub cubes: Vec<IVec3>,
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

    let fallen_bricks = sorted_bricks.into_iter().fold(
        vec![],
        |mut acc: Vec<Brick>, brick| {
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
            acc
        },
    );

    let cube_to_id_map = fallen_bricks
        .iter()
        .enumerate()
        .flat_map(|(id, brick)| {
            brick
                .cubes
                .iter()
                .map(move |cube| (cube.xyz(), id))
        })
        .collect::<HashMap<IVec3, usize>>();
    let id_to_cube_map = cube_to_id_map.iter().fold(
        HashMap::<usize, Vec<&IVec3>>::new(),
        |mut map, (cube, id)| {
            map.entry(*id)
                .and_modify(|arr| {
                    arr.push(cube);
                })
                .or_insert(vec![cube]);
            map
        },
    );

    let dissovable = fallen_bricks
        .iter()
        .filter(|brick| {
            let max_cubes = brick
                .cubes
                .iter()
                .max_set_by_key(|cube| cube.z);
            let our_id = cube_to_id_map[&max_cubes[0]];
            let max_z = max_cubes[0].z;
            // vec of ids
            let bricks_we_support: Vec<usize> = max_cubes
                .iter()
                .filter_map(|cube| {
                    cube_to_id_map.get(&IVec3::new(
                        cube.x,
                        cube.y,
                        max_z + 1,
                    ))
                })
                .unique()
                .cloned()
                .collect();
            // dbg!(&bricks_we_support);
            if bricks_we_support.is_empty() {
                return true;
            }
            // if we support cubes
            // check to see if we're the only
            // supporting cube
            // if not, we can dissolve
            //
            // our_id
            bricks_we_support
                .iter()
                .filter(|brick_id| {
                    let cubes = id_to_cube_map
                        .get(&brick_id)
                        .unwrap();
                    let min_cubes = cubes
                        .iter()
                        .min_set_by_key(|cube| cube.z);
                    // let min_cubes_xy: Vec<IVec2> =
                    //     min_cubes
                    //         .iter()
                    //         .map(|cube| cube.xy())
                    //         .collect();
                    min_cubes
                        .iter()
                        .filter_map(|cube| {
                            // get id of supporting cubes
                            cube_to_id_map.get(&IVec3::new(
                                cube.x,
                                cube.y,
                                cube.z - 1,
                            ))
                        })
                        .unique()
                        // .inspect(|supporting_cube| {
                        //     dbg!(supporting_cube);
                        // })
                        .count()
                        == 1
                    // cube_to_id_map[cube];
                })
                // .inspect(|c| {
                //     dbg!(id_to_cube_map.get(c));
                // })
                .count()
                == 0
        })
        // no 4th level brick here
        // .inspect(|brick| {
        //     dbg!(brick);
        // })
        .count();
    Ok(dissovable.to_string())
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
        assert_eq!("5", process(input)?);
        Ok(())
    }
}
