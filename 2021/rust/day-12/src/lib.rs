use std::collections::BTreeMap;

use ndarray::{s, Array2};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

fn puzzle_input(
    input: &str,
) -> IResult<&str, BTreeMap<&str, Vec<&str>>> {
    let (input, nodes) = separated_list1(
        newline,
        separated_pair(alpha1, tag("-"), alpha1),
    )(input)?;
    let mut map: BTreeMap<&str, Vec<&str>> =
        BTreeMap::new();
    for (a, b) in nodes {
        if a == "end" {
            map.entry(a).or_insert(vec![]);
        } else if b == "start" {
        } else {
            map.entry(a)
                .and_modify(|v| {
                    v.push(b);
                })
                .or_insert(vec![b]);
        }
        if b == "end" {
            map.entry(b).or_insert(vec![]);
        } else if a == "start" {
        } else {
            map.entry(b)
                .and_modify(|v| {
                    v.push(a);
                })
                .or_insert(vec![a]);
        }
    }

    Ok((input, map))
}
fn step(
    node_id: &str,
    path: Vec<&str>,
    allowed_ways: &BTreeMap<&str, Vec<&str>>,
) -> usize {
    if node_id == "end" {
        1
    } else {
        let next_nodes = allowed_ways.get(node_id).unwrap();
        next_nodes
            .iter()
            .map(|node| {
                if node.chars().all(|c| c.is_lowercase())
                    && path.contains(&node)
                {
                    0
                } else {
                    let mut new_path = path.clone();
                    new_path.push(node);
                    step(node, new_path, &allowed_ways)
                }
            })
            .sum()
    }
}
pub fn process_part1(input: &str) -> usize {
    let (_, allowed_ways) = puzzle_input(input).unwrap();

    let result =
        step("start", vec!["start"], &allowed_ways);

    result
}

fn step_2(
    node_id: &str,
    path: Vec<&str>,
    allowed_ways: &BTreeMap<&str, Vec<&str>>,
) -> usize {
    // println!(
    //     "path: {}",
    //     path.iter().cloned().collect::<String>()
    // );
    if node_id == "end" {
        1
    } else {
        let next_nodes = allowed_ways.get(node_id).unwrap();
        next_nodes
            .iter()
            .map(|node| {
                let count_of_all_small_cave_visits = path
                    .iter()
                    .filter(|path_node| {
                        path_node
                            .chars()
                            .all(|c| c.is_lowercase())
                    })
                    .fold(
                        BTreeMap::new(),
                        |mut acc, item| {
                            acc.entry(item)
                                .and_modify(|v| *v += 1)
                                .or_insert(1);
                            acc
                        },
                    );
                let have_visited_small_cave_twice =
                    count_of_all_small_cave_visits
                        .iter()
                        .any(|(_, &v)| v == 2);
                if node.chars().all(|c| c.is_lowercase())
                    && path.contains(&node)
                    && have_visited_small_cave_twice
                {
                    0
                } else {
                    let mut new_path = path.clone();
                    new_path.push(node);
                    step_2(node, new_path, &allowed_ways)
                }
            })
            .sum()
    }
}
pub fn process_part2(input: &str) -> usize {
    let (_, allowed_ways) = puzzle_input(input).unwrap();

    let result =
        step_2("start", vec!["start"], &allowed_ways);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        include_str!("./test-input.txt");

    #[test]
    fn part1_test_demo_data() {
        assert_eq!(10, process_part1(INPUT));
    }
    #[test]
    fn part2_test_demo_data() {
        assert_eq!(36, process_part2(INPUT));
    }
}
