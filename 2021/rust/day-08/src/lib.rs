use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, digit1, newline, u16},
    multi::separated_list1,
    IResult,
};
use std::collections::{BTreeMap, HashMap, HashSet};

fn output(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = take_until(" | ")(input)?;
    let (input, _) = tag(" | ")(input)?;
    let (input, outputs) =
        separated_list1(tag(" "), alpha1)(input)?;
    Ok((input, outputs))
}

fn puzzle_input(
    input: &str,
) -> IResult<&str, Vec<Vec<&str>>> {
    let (input, outputs) =
        separated_list1(newline, output)(input)?;
    Ok((input, outputs))
}

pub fn process_part1(input: &str) -> usize {
    let (_, lines) = puzzle_input(input).unwrap();
    lines
        .iter()
        .flat_map(|v| v.iter().map(|v| v.len()))
        .filter(|num| [7, 3, 2, 4].contains(num))
        .count()
}

fn signals(
    input: &str,
) -> IResult<&str, (Vec<HashSet<char>>, Vec<HashSet<char>>)>
{
    let (input, inputs) =
        separated_list1(tag(" "), alpha1)(input)?;
    let (input, _) = tag(" | ")(input)?;
    let (input, outputs) =
        separated_list1(tag(" "), alpha1)(input)?;

    let sets = inputs
        .iter()
        .map(|s| s.chars().collect::<HashSet<char>>())
        .collect();
    let output_sets = outputs
        .iter()
        .map(|s| s.chars().collect::<HashSet<char>>())
        .collect();
    Ok((input, (sets, output_sets)))
}

fn puzzle_input_2(
    input: &str,
) -> IResult<
    &str,
    Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)>,
> {
    let (input, stuff) =
        separated_list1(newline, signals)(input)?;
    Ok((input, stuff))
}

pub fn process_part2(input: &str) -> usize {
    let (_, lines) = puzzle_input_2(input).unwrap();
    let sum = lines.iter().map(process_line).sum();
    sum
}

// 7,3,2,4
// 1,4,7,8

// 0,6,9
// 2,3,5

// diff 1 and 7 -> top bar
// intersection of 1,4,7,8 -> top and bottom right vertical

// if hashset.len() == 5, and includes all hashset 7, -> 3
// diff 3 and 1, then remove top bar,
// || then intersect 4 -> mid bar
// || || then diff with mid+bottom -> bottom bar

// hashset.length == 6 && diff 3's hashset -> 9 and top-left

// hashset of 8 <diff> hashset midbar -> hashset 0
// left with hashset.len() == 6 is 6

// 6 u 2 is len 7
// 6 u 5 is len 6
// 1,4,7,8, 3,9, 0, 6
fn process_line(
    (sets, outputs): &(
        Vec<HashSet<char>>,
        Vec<HashSet<char>>,
    ),
) -> usize {
    let one = sets.iter().find(|v| v.len() == 2).unwrap();
    let four = sets.iter().find(|v| v.len() == 4).unwrap();
    let seven = sets.iter().find(|v| v.len() == 3).unwrap();
    let eight = sets.iter().find(|v| v.len() == 7).unwrap();
    let length_fives: Vec<&HashSet<char>> =
        sets.iter().filter(|v| v.len() == 5).collect();
    let length_sixes: Vec<&HashSet<char>> =
        sets.iter().filter(|v| v.len() == 6).collect();
    let three = length_fives
        .iter()
        .find(|v| {
            v.intersection(seven)
                .cloned()
                .collect::<HashSet<char>>()
                == *seven
        })
        .unwrap();
    let nine = length_sixes
        .iter()
        .find(|v| {
            v.symmetric_difference(three).count() == 1
        })
        .unwrap();
    let zero = length_sixes
        .iter()
        .filter(|v| v != &nine)
        .find(|v| v.symmetric_difference(one).count() == 4)
        .unwrap();
    let six = length_sixes
        .iter()
        .find(|v| v != &nine && v != &zero)
        .unwrap();
    let two = length_fives
        .iter()
        .filter(|v| v != &three)
        .find(|v| v.union(six).count() == 7)
        .unwrap();
    let five = length_fives
        .iter()
        .find(|v| v != &three && v != &two)
        .unwrap();
    let nums = outputs
        .iter()
        .map(|output| match output {
            x if &x == zero => "0",
            x if x == one => "1",
            x if &x == two => "2",
            x if &x == three => "3",
            x if x == four => "4",
            x if &x == five => "5",
            x if &x == six => "6",
            x if x == seven => "7",
            x if x == eight => "8",
            x if &x == nine => "9",

            x => {
                panic!("oh no");
            }
        })
        .collect::<String>();
    let value = nums.parse::<usize>().unwrap();

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &'static str =
        include_str!("./test-input.txt");

    #[test]
    fn part1_test_demo_data() {
        assert_eq!(26, process_part1(input));
    }

    #[test]
    fn part2_test_demo_data() {
        assert_eq!(61229, process_part2(input));
    }
    #[test]
    fn part2_test_demo_data_line() {
        let i = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(5353, process_part2(i));
    }
}
