// #![feature(iterator_fold_self)]

use bitvec::prelude::*;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
struct Seat {
    row: usize,
    column: usize,
}
impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.column
    }
}

fn process_seat(input: &str) -> Seat {
    let mut groups = input
        .chars()
        .group_by(|c| *c == 'F' || *c == 'B')
        .into_iter()
        .map(|(_, s)| {
            s.map(|v| match v {
                'B' | 'R' => true,
                'F' | 'L' => false,
                _ => panic!("asfkj"),
            })
            .collect::<BitVec>()
        })
        .collect::<Vec<_>>();

    groups[0].reverse();
    groups[1].reverse();

    Seat {
        row: groups[0][0..7].load::<u8>().into(),
        column: groups[1][0..].load::<u8>().into(),
    }
}

pub fn process_part1(input: &str) -> usize {
    input.lines().map(|v| process_seat(v).id()).max().unwrap()
}

pub fn process_part2(input: &str) -> usize {
    let sorted = input.lines().map(|v| process_seat(v).id()).sorted();
    let mut offset_iter = sorted.clone();
    offset_iter.next();
    sorted
        .zip(offset_iter)
        .find(|(a, b)| a + 1 != *b)
        .unwrap()
        .0
        + 1
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input_process_one() {
        assert_eq!(
            process_part1(
                "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"
            ),
            820
        );
    }
    #[test]
    fn test_input_one() {
        assert_eq!(process_seat("FBFBBFFRLR"), Seat { row: 44, column: 5 });
    }
    #[test]
    fn test_input_two() {
        assert_eq!(process_seat("BFFFBBFRRR"), Seat { row: 70, column: 7 });
    }
    #[test]
    fn test_input_three() {
        assert_eq!(process_seat("FFFBBBFRRR"), Seat { row: 14, column: 7 });
    }
    #[test]
    fn test_input_four() {
        assert_eq!(
            process_seat("BBFFBBFRLL"),
            Seat {
                row: 102,
                column: 4
            }
        );
    }
}
