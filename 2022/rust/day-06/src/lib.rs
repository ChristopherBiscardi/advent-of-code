use std::collections::{BTreeSet, HashSet};

pub fn process_part1(input: &str) -> String {
    let window_size = 4;

    let chars = input.chars().collect::<Vec<char>>();
    let sequence = chars
        .windows(window_size)
        .enumerate()
        .find(|(_i, slice)| {
            let set =
                slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == set.len()
        })
        .unwrap();
    (sequence.0 + window_size).to_string()
}

pub fn process_part2(input: &str) -> String {
    let window_size = 14;

    let chars = input.chars().collect::<Vec<char>>();
    let sequence = chars
        .windows(window_size)
        .enumerate()
        .find(|(_i, slice)| {
            let set =
                slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == set.len()
        })
        .unwrap();

    (sequence.0 + window_size).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(
            process_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            "7"
        );
        assert_eq!(
            process_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            "5"
        );
        assert_eq!(
            process_part1("nppdvjthqldpwncqszvftbrmjlhg"),
            "6"
        );
        assert_eq!(
            process_part1(
                "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
            ),
            "10"
        );
        assert_eq!(
            process_part1(
                "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
            ),
            "11"
        );
    }

    #[test]
    fn part2_works() {
        //qmgbljsphdztnv
        assert_eq!(
            process_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            "19"
        );
        assert_eq!(
            process_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            "23"
        );
        assert_eq!(
            process_part2("nppdvjthqldpwncqszvftbrmjlhg"),
            "23"
        );
        assert_eq!(
            process_part2(
                "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
            ),
            "29"
        );
        assert_eq!(
            process_part2(
                "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
            ),
            "26"
        );
    }
}
