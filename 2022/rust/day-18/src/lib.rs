use glam::{IVec3, Vec3};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    *,
};
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fmt::Display,
};
pub mod camera;

pub fn points(input: &str) -> IResult<&str, Vec<IVec3>> {
    separated_list1(
        line_ending,
        separated_list1(tag(","), complete::i32)
            .map(|vec| IVec3::new(vec[0], vec[1], vec[2])),
    )(input)
}
pub fn process_part1(input: &str) -> String {
    let (_, points) = points(input).unwrap();
    let points: HashSet<IVec3> =
        HashSet::from_iter(points.into_iter());

    let surface_area = points
        .iter()
        .map(|&IVec3 { x, y, z }| {
            // number of free sides
            let x_low = IVec3::new(x - 1, y, z);
            let x_high = IVec3::new(x + 1, y, z);
            let y_low = IVec3::new(x, y - 1, z);
            let y_high = IVec3::new(x, y + 1, z);
            let z_low = IVec3::new(x, y, z - 1);
            let z_high = IVec3::new(x, y, z + 1);
            [x_low, x_high, y_low, y_high, z_low, z_high]
                .iter()
                .filter(|ivec| points.get(ivec).is_none())
                .count()
        })
        .sum::<usize>();

    surface_area.to_string()
}

fn process_block(
    &IVec3 { x, y, z }: &IVec3,
    points: &HashSet<IVec3>,
) -> usize {
    // number of free sides
    let x_low = IVec3::new(x - 1, y, z);
    let x_high = IVec3::new(x + 1, y, z);
    let y_low = IVec3::new(x, y - 1, z);
    let y_high = IVec3::new(x, y + 1, z);
    let z_low = IVec3::new(x, y, z - 1);
    let z_high = IVec3::new(x, y, z + 1);
    [x_low, x_high, y_low, y_high, z_low, z_high]
        .iter()
        .filter(|ivec| points.get(ivec).is_none())
        .map(|ivec| {
            if is_interior_block(&ivec, &points) {
                // (interior wall, exterior wall)
                (1, 0)
            } else {
                (0, 1)
            }
        })
        .map(|(_interior, exterior)| exterior)
        .sum::<usize>()
}

pub fn process_part2(input: &str) -> String {
    let (_, points) = points(input).unwrap();
    let points: HashSet<IVec3> =
        HashSet::from_iter(points.into_iter());

    let surface_area = points
        .iter()
        .map(|&IVec3 { x, y, z }| {
            // number of free sides
            let x_low = IVec3::new(x - 1, y, z);
            let x_high = IVec3::new(x + 1, y, z);
            let y_low = IVec3::new(x, y - 1, z);
            let y_high = IVec3::new(x, y + 1, z);
            let z_low = IVec3::new(x, y, z - 1);
            let z_high = IVec3::new(x, y, z + 1);
            [x_low, x_high, y_low, y_high, z_low, z_high]
                .iter()
                .filter(|ivec| points.get(ivec).is_none())
                .map(|ivec| {
                    if is_interior_block(&ivec, &points) {
                        let IVec3 { x, y, z } = *ivec;
                        let x_low = IVec3::new(x - 1, y, z);
                        let x_high =
                            IVec3::new(x + 1, y, z);
                        let y_low = IVec3::new(x, y - 1, z);
                        let y_high =
                            IVec3::new(x, y + 1, z);
                        let z_low = IVec3::new(x, y, z - 1);
                        let z_high =
                            IVec3::new(x, y, z + 1);
                        // (interior wall, exterior wall)
                        let is_really_exterior_block = [
                            x_low, x_high, y_low, y_high,
                            z_low, z_high,
                        ]
                        .iter()
                        .filter(|ivec| {
                            points.get(ivec).is_none()
                        })
                        .any(|block| {
                            process_block(block, &points)
                                >= 1
                        });
                        if is_really_exterior_block {
                            (0, 1)
                        } else {
                            (1, 0)
                        }
                    } else {
                        (0, 1)
                    }
                })
                .map(|(_interior, exterior)| exterior)
                .sum::<usize>()
        })
        .sum::<usize>();

    surface_area.to_string()
}

fn is_interior_block(
    &IVec3 { x, y, z }: &IVec3,
    points: &HashSet<IVec3>,
) -> bool {
    let bounded_x_pos = points
        .iter()
        .find(|point| {
            point.x > x && point.y == y && point.z == z
        })
        .is_some();
    let bounded_x_neg = points
        .iter()
        .find(|point| {
            point.x < x && point.y == y && point.z == z
        })
        .is_some();
    let bounded_y_pos = points
        .iter()
        .find(|point| {
            point.x == x && point.y > y && point.z == z
        })
        .is_some();
    let bounded_y_neg = points
        .iter()
        .find(|point| {
            point.x == x && point.y < y && point.z == z
        })
        .is_some();
    let bounded_z_pos = points
        .iter()
        .find(|point| {
            point.x == x && point.y == y && point.z > z
        })
        .is_some();
    let bounded_z_neg = points
        .iter()
        .find(|point| {
            point.x == x && point.y == y && point.z < z
        })
        .is_some();
    [
        bounded_x_pos,
        bounded_x_neg,
        bounded_y_pos,
        bounded_y_neg,
        bounded_z_pos,
        bounded_z_neg,
    ]
    .iter()
    .all(|v| *v)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "64");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "58");
    }
}
