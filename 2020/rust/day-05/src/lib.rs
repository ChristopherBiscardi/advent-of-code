#![feature(iterator_fold_self)]

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
    let groups = input
        .chars()
        .group_by(|c| *c == 'F' || *c == 'B')
        .into_iter()
        .map(|(_, s)| s.collect::<String>())
        .collect::<Vec<String>>();
    let row = groups[0].chars().fold((0, 128), |acc, cur| match cur {
        'F' => {
            let size = acc.1 - acc.0;
            (acc.0, acc.1 - size / 2)
        }
        'B' => {
            let size = acc.1 - acc.0;
            (acc.0 + size / 2, acc.1)
        }
        c => panic!("FB panic {}", c),
    });
    let column = groups[1].chars().fold((0, 8), |acc, cur| match cur {
        'L' => {
            let size = acc.1 - acc.0;
            (acc.0, acc.1 - size / 2)
        }
        'R' => {
            let size = acc.1 - acc.0;
            (acc.0 + size / 2, acc.1)
        }
        c => panic!("LR panic {}", c),
    });
    Seat {
        row: row.0,
        column: column.0,
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
