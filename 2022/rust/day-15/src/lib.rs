use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete,
    character::complete::line_ending,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    *,
};
use rayon::prelude::*;
use std::collections::BTreeMap;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Sensor {
    x: i32,
    y: i32,
}
#[derive(Debug, PartialEq)]
struct Beacon {
    x: i32,
    y: i32,
}
fn position(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        preceded(tag("x="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32),
    )(input)
}
fn map(
    input: &str,
) -> IResult<&str, BTreeMap<Sensor, Beacon>> {
    let (input, list) = separated_list1(
        line_ending,
        preceded(
            tag("Sensor at "),
            separated_pair(
                position.map(|(x, y)| Sensor { x, y }),
                tag(": closest beacon is at "),
                position.map(|(x, y)| Beacon { x, y }),
            ),
        ),
    )(input)?;

    Ok((
        input,
        list.into_iter()
            .collect::<BTreeMap<Sensor, Beacon>>(),
    ))
}

pub fn process_part1(
    input: &str,
    line_number: i32,
) -> String {
    let (_, map) = map(input).unwrap();
    let distances: BTreeMap<&Sensor, i32> = map
        .iter()
        .map(|(sensor, beacon)| {
            (
                sensor,
                (beacon.x - sensor.x).abs()
                    + (beacon.y - sensor.y).abs(),
            )
        })
        .collect();

    distances
        .iter()
        .filter(|(sensor, distance)| {
            let sensor_range = (sensor.y - **distance)
                ..(sensor.y + **distance);
            sensor_range.contains(&line_number)
        })
        .flat_map(|(sensor, max_distance)| {
            // let width = distance * 2 + 1;
            let distance_to_line = sensor.y - line_number;
            // let direction_to_line =
            //     distance_to_line.signum();

            let max_distance_on_line =
                max_distance - distance_to_line.abs();

            (sensor.x - max_distance_on_line)
                ..=sensor.x + max_distance_on_line
        })
        .unique()
        .filter(|x| {
            !map.values().contains(&Beacon {
                x: *x,
                y: line_number,
            })
        })
        .count()
        .to_string()
}

pub fn process_part2(input: &str, limit: i32) -> String {
    let (_, map) = map(input).unwrap();
    let distances: BTreeMap<&Sensor, i32> = map
        .iter()
        .map(|(sensor, beacon)| {
            (
                sensor,
                (beacon.x - sensor.x).abs()
                    + (beacon.y - sensor.y).abs(),
            )
        })
        .collect();
    let possible_beacon_location = (0..=limit)
        .cartesian_product(0..=limit)
        .par_bridge()
        .find_any(|(y, x)| {
            if y < &0 || x < &0 || y > &limit || x > &limit
            {
                return false;
            }
            let is_beacon = map
                .values()
                .contains(&Beacon { x: *x, y: *y });
            if is_beacon {
                return false;
            }
            let is_sensed = distances
                .iter()
                .filter(|(sensor, distance)| {
                    let sensor_range = (sensor.y
                        - **distance)
                        ..(sensor.y + **distance);
                    sensor_range.contains(&y)
                })
                .find(|(sensor, max_distance)| {
                    let distance_to_line = sensor.y - y;

                    let max_distance_on_line =
                        **max_distance
                            - distance_to_line.abs();

                    let sensor_range = (sensor.x
                        - max_distance_on_line)
                        ..=(sensor.x
                            + max_distance_on_line);
                    sensor_range.contains(x)
                });
            // if position is not sensed by sensor
            is_sensed.is_none()
        });

    let Some(beacon) = possible_beacon_location else {
        panic!("ohnooooo");
    };
    (beacon.1 * 4000000 + beacon.0).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT, 10), "26");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT, 20), "56000011");
    }
}
