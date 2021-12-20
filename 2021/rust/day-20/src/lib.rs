use core::fmt;
use std::{cmp::Ordering, iter::Sum, ops::Add};

use itertools::Itertools;
use ndarray::{arr1, arr2, concatenate, Array2, Axis};
use nom::{
    bytes::complete::{tag, take},
    character::complete::{
        self, alpha1, anychar, newline, one_of, u32,
    },
    combinator::opt,
    multi::{
        length_value, many0, many1, many_m_n,
        separated_list1,
    },
    sequence::{
        pair, preceded, separated_pair, terminated,
    },
    IResult,
};

fn newlines(input: &str) -> IResult<&str, ()> {
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    Ok((input, ()))
}
fn pad_array(
    original: &Array2<char>,
    // zero_char: char,
) -> Array2<char> {
    let pad_axis_1 = Array2::from_elem(
        (original.len_of(Axis(0)), 1),
        '.',
    );

    let padded_axis_1 = concatenate(
        Axis(1),
        &[
            // pad_axis_1.view(),
            pad_axis_1.view(),
            pad_axis_1.view(),
            original.view(),
            // pad_axis_1.view(),
            pad_axis_1.view(),
            pad_axis_1.view(),
        ],
    )
    .unwrap();

    let pad_axis_0 = Array2::from_elem(
        (1, padded_axis_1.len_of(Axis(1))),
        '.',
    );

    let padded_axis_0 = concatenate(
        Axis(0),
        &[
            // pad_axis_0.view(),
            pad_axis_0.view(),
            pad_axis_0.view(),
            padded_axis_1.view(),
            // pad_axis_0.view(),
            pad_axis_0.view(),
            pad_axis_0.view(),
        ],
    )
    .unwrap();
    padded_axis_0
}
fn image(input: &str) -> IResult<&str, Array2<char>> {
    let (input, raw_input) = separated_list1(
        newline,
        many1(one_of("#.")),
    )(input)?;

    let nrows = raw_input.len();
    let ncols = raw_input[0].len();

    let image = Array2::from_shape_vec(
        (nrows, ncols),
        raw_input
            .into_iter()
            .flatten()
            .collect::<Vec<char>>(),
    )
    .unwrap();

    Ok((input, image))
}
fn puzzle_input(
    input: &str,
) -> IResult<&str, (Vec<char>, Array2<char>)> {
    let (input, (algo, image)) = separated_pair(
        many1(one_of(".#")),
        newlines,
        image,
    )(input)?;
    Ok((input, (algo, image)))
}

fn process(
    image: &Array2<char>,
    algo: &Vec<char>,
) -> Array2<char> {
    let padded_image = pad_array(image);
    let processed_image = padded_image
        .windows((3, 3))
        .into_iter()
        .map(|elems| {
            let string_num = elems
                .iter()
                .map(|item| match item {
                    '#' => "1",
                    '.' => "0",
                    _ => panic!("input wasn't correct"),
                })
                .collect::<String>();
            let num = usize::from_str_radix(&string_num, 2)
                .expect("a valid parse");
            algo.get(num).expect("a valid index")
        })
        .cloned()
        .collect::<Vec<char>>();
    // print_image(
    //     Array2::from_shape_vec(
    //         (
    //             padded_image.len_of(Axis(0)),
    //             padded_image.len_of(Axis(1)),
    //         ),
    //         processed_image.clone(),
    //     )
    //     .unwrap(),
    // );

    // Array2::from_shape_vec(
    //     (
    //         padded_image.len_of(Axis(0)) - 2,
    //         padded_image.len_of(Axis(1)) - 2,
    //     ),
    //     processed_image,
    // )
    // .unwrap()
    Array2::from_shape_vec(
        (
            padded_image.len_of(Axis(0)) - 2,
            padded_image.len_of(Axis(1)) - 2,
        ),
        processed_image,
    )
    .unwrap()
}
fn print_image(input: &Array2<char>) {
    for list in input.axis_iter(Axis(0)) {
        for c in list.iter() {
            print!("{}", c);
        }
        print!("\n");
    }
    print!("\n");
}
pub fn process_part1(input: &str) -> usize {
    let (_, (algo, image)) =
        puzzle_input(input).expect("input to be valid");
    let new_image = process(&image, &algo);
    print_image(&new_image);
    let new_image = process(&new_image, &algo);
    print_image(&new_image);
    // dbg!(algo, image);
    new_image
        .iter()
        .filter(|v| match v {
            '#' => true,
            _ => false,
        })
        .count()
}

pub fn process_part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        include_str!("./test-input.txt");

    #[test]
    fn part1_test_demo_data() {
        assert_eq!(35, process_part1(INPUT));
    }
    //     #[test]
    //     fn part1_test_random_data() {
    //         let SECOND = "#.#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

    // #....
    // #....
    // #...#
    // ..#..
    // ..###";
    //         let (_, (algo, image)) = puzzle_input(SECOND)
    //             .expect("input to be valid");
    //         let new_image = process(&image, &algo);
    //         print_image(&new_image);
    //         // assert_eq!(35, process_part1(SECOND));
    //         assert_eq!(false, true);
    //     }

    // #[test]
    // fn part2_test_demo_data() {
    //     assert_eq!(3993, process_part2(INPUT));
    // }
}
