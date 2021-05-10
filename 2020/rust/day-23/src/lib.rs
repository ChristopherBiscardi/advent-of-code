#![feature(iter_advance_by)]

use itertools::Itertools;
use std::convert::TryInto;

pub fn process_part1(input: &str) -> String {
    let mut advance: usize = 0;
    let mut nums: Vec<usize> = vec![];
    let mut iter = input
        .chars()
        .map(|c| {
            let t: usize = c.to_digit(10).unwrap().try_into().unwrap();
            t
        })
        .cycle();
    for step in 0..10 {
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let c = iter.next().unwrap();
        dbg!(advance);
        iter.advance_by(advance)
            .expect("expected to advanced in a cycle");
        let next = iter.next().unwrap();
        advance = next;
        nums.push(next);
    }
    nums.iter().map(|v| v.to_string()).collect::<String>()
}

pub fn process_part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input_process_one() {
        assert_eq!(process_part1("389125467"), "67384529".to_string())
    }
}
