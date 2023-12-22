use std::{collections::HashSet, ops::RangeInclusive};

use glam::{IVec2, Mat3, Vec3};
use itertools::{Itertools, MinMaxResult};
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_until},
    character::complete::line_ending,
    combinator::opt,
    multi::{fold_many1, many1},
    sequence::{preceded, terminated},
    IResult, Parser,
};
use nom_locate::LocatedSpan;

use crate::custom_error::AocError;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

fn with_xy(span: Span) -> SpanIVec2 {
    // column/location are 1-indexed
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}

fn parse_grid(
    input: Span,
) -> IResult<Span, (HashSet<IVec2>, IVec2)> {
    fold_many1(
        terminated(
            alt((tag("."), tag("S"))).map(with_xy),
            opt(alt((line_ending, is_a("#")))),
        ),
        || (HashSet::new(), IVec2::splat(0)),
        |(mut set, mut start), next| {
            if next.fragment() == &"S" {
                start = next.extra;
            }
            set.insert(next.extra);
            (set, start)
        },
    )(input)
}

fn parse_rock_grid(
    input: Span,
) -> IResult<Span, (HashSet<IVec2>, IVec2)> {
    fold_many1(
        preceded(take_until("#"), tag("#").map(with_xy)),
        || (HashSet::new(), IVec2::splat(0)),
        |(mut set, mut start), next| {
            if next.fragment() == &"S" {
                start = next.extra;
            }
            set.insert(next.extra);
            (set, start)
        },
    )(input)
}

#[tracing::instrument]
pub fn process(
    input: &str,
    step_count: usize,
) -> miette::Result<String, AocError> {
    let (_, (set, start)) = parse_grid(Span::new(input))
        .expect("a valid parse");

    let (_, (rock_set, _)) =
        parse_rock_grid(Span::new(input))
            .expect("a valid second parse");

    let row_count = input.lines().count() as i32;
    let column_count =
        input.lines().next().unwrap().len() as i32;
    let bounds = IVec2::new(column_count, row_count);

    let mut starting_hashset = HashSet::new();
    starting_hashset.insert(start);

    let last_set = std::iter::successors(
        Some(starting_hashset),
        |occupied_positions| {
            let mut new_set: HashSet<IVec2> =
                HashSet::new();

            for pos in occupied_positions.into_iter() {
                [
                    IVec2::X,
                    IVec2::NEG_X,
                    IVec2::Y,
                    IVec2::NEG_Y,
                ]
                .into_iter()
                .filter_map(|offset| {
                    let cell = offset + *pos;
                    set.contains(&(cell.rem_euclid(bounds)))
                        .then_some(cell)
                })
                .for_each(|pos| {
                    new_set.insert(pos);
                });
            }
            Some(new_set)
        },
    )
    .enumerate()
    .inspect(|(i, set)| {
        if ((i - 65) % 131) == 0 {
            // this produces the sequence numbers used below
            println!("{:03}: {}", i, set.len());
        }

        // sequence numbers
        let seq0: i64 = 3776;
        let seq1: i64 = 33652;
        let seq2: i64 = 93270;

        // the n we're looking for (which is not the step count
        // from the problem description)
        let n: i64 = 202300;

        // Vandermonde matrix is built from the sequence number input
        // so x=0, y=1, z=2
        // x.pow(0), x.pow(1), x.pow(2)
        // y.pow(0), y.pow(1), y.pow(2)
        // z.pow(0), z.pow(1), z.pow(2)
        //
        // 1, 0, 0
        // 1, 1, 1
        // 1, 2, 4
        let x: f32 = 0.;
        let y: f32 = 1.;
        let z: f32 = 2.;
        let vandermonde = Mat3::from_cols(
            Vec3::new(x.powf(0.), y.powf(0.), z.powf(0.)),
            Vec3::new(x.powf(1.), y.powf(1.), z.powf(1.)),
            Vec3::new(x.powf(2.), y.powf(2.), z.powf(2.)),
        );
        // multiplying vandermonde by the sequence numbers
        // yields our a,b,c values
        let [c, b, a] = (vandermonde.inverse()
            * Vec3::new(
                seq0 as f32,
                seq1 as f32,
                seq2 as f32,
            ))
        .to_array();
        let a = a as i64;
        let b = b as i64;
        let c = c as i64;

        dbg!(a, b, c);
        dbg!(a * n.pow(2) + b * n + c);
    })
    // .inspect(|set| {
    //     println!("");
    //     println!("{:?}", set);
    //     // dbg!(set.len());
    //     let (x_min, x_max) =
    //         match set.iter().map(|pos| pos.x).minmax() {
    //             MinMaxResult::NoElements => panic!(""),
    //             MinMaxResult::OneElement(x) => (x, x),
    //             MinMaxResult::MinMax(x_min, x_max) => {
    //                 (x_min - 1, x_max + 1)
    //             }
    //         };
    //     let (y_min, y_max) =
    //         match set.iter().map(|pos| pos.x).minmax() {
    //             MinMaxResult::NoElements => panic!(""),
    //             MinMaxResult::OneElement(x) => (x, x),
    //             MinMaxResult::MinMax(y_min, y_max) => {
    //                 (y_min - 1, y_max + 1)
    //             }
    //         };
    //     print_grid(
    //         &set.iter().collect::<Vec<&IVec2>>(),
    //         &rock_set,
    //         // y_min..=y_max,
    //         // x_min..=x_max,
    //         -33..=33,
    //         -33..=33,
    //         bounds,
    //     );
    // })
    .skip(step_count)
    .next()
    .unwrap()
    .1;

    Ok(last_set.len().to_string())
}

#[allow(dead_code)]
fn print_grid(
    map: &[&IVec2],
    rocks: &HashSet<IVec2>,
    y_bound: RangeInclusive<i32>,
    x_bound: RangeInclusive<i32>,
    rock_bounds: IVec2,
) {
    for y in y_bound {
        for x in x_bound.clone() {
            match (
                map.contains(&&IVec2::new(x, y)),
                rocks.contains(
                    &(IVec2::new(x, y)
                        .rem_euclid(rock_bounds)),
                ),
            ) {
                (true, _) => {
                    print!("O");
                }
                (false, true) => print!("#"),
                (false, false) => print!("."),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(6, 16)]
    #[case(10, 50)]
    #[case(50, 1594)]
    #[case(100, 6536)]
    #[case(500, 167004)]
    #[case(1000, 668697)]
    #[case(5000, 16733044)]
    fn test_process(
        #[case] step_count: usize,
        #[case] expected_tiles: usize,
    ) -> miette::Result<()> {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(
            expected_tiles.to_string(),
            process(input, step_count)?
        );

        Ok(())
    }
}
