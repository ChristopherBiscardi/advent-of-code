// use itertools::Itertools;
use nom::{bytes::complete::tag, character, IResult};
use std::io::{Error, ErrorKind};

pub fn process_part1(input: &str) -> usize {
    process_slope(input, (3, 1))
}

fn process_slope(input: &str, (x, y): (usize, usize)) -> usize {
    let num_rows = input.lines().count();
    // let num_columns = input.lines().nth(0).unwrap().len();
    let num_x_traversing = x * (num_rows / y) + 1;
    let results = input
        .lines()
        .map(|row| row.chars().cycle().take(num_x_traversing))
        .step_by(y)
        .enumerate()
        .filter_map(|(iter_i, mut row)| match row.nth(iter_i * x) {
            Some('.') => None,
            s => s,
        })
        .count();
    results
}
pub fn process_part2(input: &str, slopes: Vec<(usize, usize)>) -> usize {
    slopes
        .iter()
        .map(|slope| process_slope(input, slope.clone()))
        .product()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input_one() {
        assert_eq!(
            process_part1(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
            ),
            7
        )
    }
    #[test]
    fn test_input_two() {
        assert_eq!(
            process_part2(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
                vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            ),
            336
        )
    }
}
