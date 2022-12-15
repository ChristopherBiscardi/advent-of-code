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
use std::{
    collections::BTreeMap,
    ops::{RangeBounds, RangeInclusive},
};

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

    let mut low_high: BTreeMap<
        i32,
        Vec<RangeInclusive<i32>>,
    > = BTreeMap::new();
    for (y, range) in distances.iter().flat_map(
        |(sensor, max_distance)| {
            ((sensor.y - max_distance)
                ..(sensor.y + max_distance))
                .map(|y| {
                    let distance_to_line = sensor.y - y;

                    let max_distance_on_line = *max_distance
                        - distance_to_line.abs();

                    (
                        y,
                        ((sensor.x - max_distance_on_line)
                            .max(0))
                            ..=((sensor.x
                                + max_distance_on_line)
                                .min(limit)),
                    )
                })
        },
    )
    // .inspect(|x| {
    //     if x.0 == 11 {
    //         dbg!(x);
    //     }
    // })
    {
        if y >= 0 && y <= limit {
            low_high
                .entry(y)
                .and_modify(|lh| lh.push(range.clone()))
                .or_insert(vec![range]);
        }
    }
    println!("counting");

    let v: Vec<(i32, i32)> = low_high
        .into_iter()
        .filter_map(|(key, mut ranges)| {
            // println!("{}", key);
            ranges.sort_by(|a, b| a.start().cmp(b.start()));
            let result: (RangeInclusive<i32>, Option<i32>) =
                ranges.iter().fold(
                    (0..=0, None),
                    |mut acc, range| {
                        if acc.1.is_some() {
                            return acc;
                        }
                        if acc.0.end() + 1 >= *range.start()
                        {
                            acc.0 = *acc.0.start()
                                ..=(*acc
                                    .0
                                    .end()
                                    .max(range.end()));
                        } else {
                            dbg!(&acc, range);
                            acc.1 = Some(acc.0.end() + 1);
                        }

                        acc
                    },
                );
            result.1.map(|x| {
                dbg!(ranges);
                (x, key)
            })
        })
        .collect();
    println!("done counting");
    dbg!(&v);
    let x = v[0].0;
    let y = v[0].1;
    // let row = low_high
    //     .iter()
    //     .filter(|(key, bounds)| {
    //         (bounds.0..bounds.1).len() < limit as usize
    //     })
    //     .collect::<Vec<_>>();
    // dbg!(row);
    // let y = row_index;
    // let possible_beacon_location =
    //     (0..=limit).into_par_iter().find_any(|x| {
    //         if y < &0 || x < &0 || y > &limit || x > &limit
    //         {
    //             return false;
    //         }
    //         let is_beacon = map
    //             .values()
    //             .contains(&Beacon { x: *x, y: *y });
    //         if is_beacon {
    //             return false;
    //         }
    //         let is_sensed = distances
    //             .iter()
    //             .filter(|(sensor, distance)| {
    //                 let sensor_range = (sensor.y
    //                     - **distance)
    //                     ..(sensor.y + **distance);
    //                 sensor_range.contains(&y)
    //             })
    //             .find(|(sensor, max_distance)| {
    //                 let distance_to_line = sensor.y - y;

    //                 let max_distance_on_line =
    //                     **max_distance
    //                         - distance_to_line.abs();

    //                 let sensor_range = (sensor.x
    //                     - max_distance_on_line)
    //                     ..=(sensor.x
    //                         + max_distance_on_line);
    //                 sensor_range.contains(x)
    //             });
    //         // if position is not sensed by sensor
    //         is_sensed.is_none()
    //     });

    // let Some(beacon_x) = possible_beacon_location else {
    //     panic!("ohnooooo");
    // };
    (x * 4000000 + y).to_string()
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
