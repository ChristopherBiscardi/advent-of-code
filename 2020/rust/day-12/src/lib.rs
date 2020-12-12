use itertools::Itertools;
use std::cmp::{Ord, Ordering};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;

fn instruction(input: &str) -> (&str, isize) {
    let (dir, i) = input.split_at(1);
    (dir, i.parse::<isize>().unwrap())
}
pub fn process_part1(input: &str) -> isize {
    let ((x, y), _) = input.lines().fold(
        ((0, 0), 0),
        |((x, y), direction): ((isize, isize), isize), ins| {
            let (letter, i) = instruction(ins);
            match letter {
                "N" => ((x, y + i), direction),
                "S" => ((x, y - i), direction),
                "E" => ((x + i, y), direction),
                "W" => ((x - i, y), direction),
                "L" => ((x, y), direction - i),
                "R" => ((x, y), direction + i),
                "F" => match direction.rem_euclid(360) {
                    0 => ((x + i, y), direction),
                    90 => ((x, y - i), direction),
                    180 => ((x - i, y), direction),
                    270 => ((x, y + i), direction),
                    _ => panic!("whoops"),
                },
                _ => panic!("no letter"),
            }
        },
    );
    x.abs() + y.abs()
}

fn turn_right(x: isize, y: isize) -> (isize, isize) {
    (y, -x)
}
fn turn_left(x: isize, y: isize) -> (isize, isize) {
    (-y, x)
}
pub fn process_part2(input: &str) -> isize {
    let ((x, y), _) = input
        .lines()
        .fold(((0, 0), (10, 1)), |((x, y), (wx, wy)), ins| {
            let (letter, i) = instruction(ins);
            match letter {
                "N" => ((x, y), (wx, wy + i)),
                "S" => ((x, y), (wx, wy - i)),
                "E" => ((x, y), (wx + i, wy)),
                "W" => ((x, y), (wx - i, wy)),
                "L" => {
                    let num_turns = (i).div_euclid(90);
                    let pos = (0..num_turns).fold((wx, wy), |(xa, ya), _| turn_left(xa, ya));
                    ((x, y), (pos.0, pos.1))
                }
                "R" => {
                    let num_turns = (i).div_euclid(90);
                    let pos = (0..num_turns).fold((wx, wy), |(xa, ya), _| turn_right(xa, ya));
                    ((x, y), (pos.0, pos.1))
                }
                "F" => ((x + wx * i, y + wy * i), (wx, wy)),
                _ => panic!("no letter"),
            }
        });
    // dbg!(x, y);
    x.abs() + y.abs()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input_process_one() {
        assert_eq!(
            process_part1(
                "\
F10
N3
F7
R90
F11"
            ),
            25
        )
    }

    #[test]
    fn test_input_process_two() {
        assert_eq!(
            process_part2(
                "\
F10
N3
F7
R90
F11"
            ),
            286
        )
    }
}
