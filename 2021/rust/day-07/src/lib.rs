use nom::{
    bytes::complete::tag, character::complete::u16,
    multi::separated_list1, IResult,
};
use std::collections::BTreeMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct InitialPosition(pub u16);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct NumCrabs(pub u16);

pub fn puzzle_input2(
    input: &str,
) -> IResult<&str, Vec<u16>> {
    let (input, crabs) =
        separated_list1(tag(","), u16)(input)?;

    Ok((input, crabs))
}

pub fn puzzle_input(
    input: &str,
) -> IResult<&str, BTreeMap<InitialPosition, NumCrabs>> {
    let (input, crabs) =
        separated_list1(tag(","), u16)(input)?;
    let mut map: BTreeMap<InitialPosition, NumCrabs> =
        BTreeMap::new();
    for crab in crabs {
        map.entry(InitialPosition(crab))
            .and_modify(|v| (*v).0 += 1)
            .or_insert(NumCrabs(1));
    }
    // dbg!(&map);

    Ok((input, map))
}

pub fn process_part1(input: &str) -> u32 {
    let (_, crabs) = puzzle_input(input).unwrap();
    let mut crab_rave = crabs.iter();
    let min = crab_rave.next().unwrap().0;
    let max = crab_rave.last().unwrap().0;
    // dbg!(min, max);
    let totals = (min.0..=max.0)
        .map(|goal_position| {
            // dbg!(goal_position);
            let total_movement: u32 = crabs
                .iter()
                .map(|(initial_position, num_crabs)| {
                    let dx: u32 = (initial_position.0
                        as i32
                        - goal_position as i32)
                        .abs()
                        as u32;
                    dx * (num_crabs.0 as u32)
                })
                .sum();
            total_movement
        })
        .min();
    totals.unwrap()
}

pub fn process_part1_opt2(input: &str) -> u32 {
    let (_, mut crabs) = puzzle_input2(input).unwrap();
    let optimal_crab_position_idx = (crabs.len() + 1) / 2;

    crabs.sort();
    let goal_position =
        crabs.get(optimal_crab_position_idx).unwrap();

    // dbg!(goal_position);
    let total_movement: u32 = crabs
        .iter()
        .map(|initial_position| {
            let dx: u32 = (*initial_position as i32
                - *goal_position as i32)
                .abs() as u32;
            dx
        })
        .sum();

    total_movement
}
pub fn process_part2(input: &str) -> u32 {
    let (_, crabs) = puzzle_input(input).unwrap();
    let mut crab_rave = crabs.iter();
    let min = crab_rave.next().unwrap().0;
    let max = crab_rave.last().unwrap().0;
    // dbg!(min, max);
    let totals = (min.0..=max.0)
        .map(|goal_position| {
            // dbg!(goal_position);
            let total_movement: u32 = crabs
                .iter()
                .map(|(initial_position, num_crabs)| {
                    let dx: u32 = (initial_position.0
                        as i32
                        - goal_position as i32)
                        .abs()
                        as u32;
                    // old cost fn
                    // let cost: u32 = (0..=dx).sum();
                    let cost = dx * (1 + dx) / 2;
                    cost * (num_crabs.0 as u32)
                })
                .sum();
            total_movement
        })
        .min();
    totals.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1_test_demo_data() {
        assert_eq!(37, process_part1(input));
    }
    #[test]
    fn part1_opt2_test_demo_data() {
        assert_eq!(37, process_part1(input));
    }
    #[test]
    fn part2_test_demo_data() {
        assert_eq!(168, process_part2(input));
    }
}
