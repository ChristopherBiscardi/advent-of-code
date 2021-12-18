use std::cmp::Ordering;

use itertools::Itertools;
use ndarray::{concatenate, Array2, Axis};
use nom::{
    bytes::complete::{tag, take},
    character::complete::{
        self, alpha1, anychar, newline, one_of, u32,
    },
    multi::{
        length_value, many0, many1, many_m_n,
        separated_list1,
    },
    sequence::{
        pair, preceded, separated_pair, terminated,
    },
    IResult,
};

fn arc(
    (x, y): (i32, i32),
    (
        x_lower_bound,
        x_upper_bound,
        y_lower_bound,
        y_upper_bound,
    ): (i32, i32, i32, i32),
) -> Vec<(i32, i32)> {
    let mut steps = vec![];
    for timestep in 0.. {
        let new_y =
            (0..timestep).map(|step| y - step).sum();
        let new_x =
            (0..timestep)
                .map(|step| {
                    if (x - step) <= 0 {
                        0
                    } else {
                        x - step
                    }
                })
                .sum();
        if new_x > x_upper_bound {
            break;
        };
        if new_y < y_lower_bound {
            break;
        };
        // dbg!((new_x, new_y));
        steps.push((new_x, new_y));
    }
    steps
}

fn puzzle_input(
    input: &str,
) -> IResult<&str, (i32, i32, i32, i32)> {
    let (input, _) = tag("target area: x=")(input)?;
    let (input, x_lower_bound) = complete::i32(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, x_upper_bound) = complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y_lower_bound) = complete::i32(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, y_upper_bound) = complete::i32(input)?;

    Ok((
        input,
        (
            x_lower_bound,
            x_upper_bound,
            y_lower_bound,
            y_upper_bound,
        ),
    ))
}

// x=20..30, y=-10..-5
pub fn process_part1(input: &str) -> i32 {
    let (
        _input,
        (
            x_lower_bound,
            x_upper_bound,
            y_lower_bound,
            y_upper_bound,
        ),
    ) = puzzle_input(input).unwrap();
    let possible_xs = 0..=x_upper_bound;
    let possible_ys = 0..=(y_lower_bound).abs();
    let x_target = x_lower_bound..x_upper_bound;
    let y_target = y_lower_bound..y_upper_bound;
    let result = possible_xs
        .cartesian_product(possible_ys)
        .map(|(x, y)| {
            arc(
                (x, y),
                (
                    x_lower_bound,
                    x_upper_bound,
                    y_lower_bound,
                    y_upper_bound,
                ),
            )
        })
        // filter out any arcs that don't hit target
        .filter(|arc| {
            arc.iter().any(|(x, y)| {
                x_target.contains(x) && y_target.contains(y)
            })
        })
        .max_by_key(|arc| {
            arc.iter().max_by_key(|(_, y)| y).unwrap().1
        });
    // get max y value
    result.unwrap().iter().max_by_key(|(_, y)| y).unwrap().1
}

pub fn process_part2(input: &str) -> usize {
    let (
        _input,
        (
            x_lower_bound,
            x_upper_bound,
            y_lower_bound,
            y_upper_bound,
        ),
    ) = puzzle_input(input).unwrap();
    let possible_xs = 0..=x_upper_bound;
    let new_upper_bound =
        match y_lower_bound.abs().cmp(&y_upper_bound.abs())
        {
            Ordering::Less => y_upper_bound.abs(),
            Ordering::Equal => y_upper_bound.abs(),
            Ordering::Greater => y_lower_bound.abs(),
        };
    let possible_ys = y_lower_bound..=new_upper_bound;
    let x_target = x_lower_bound..=x_upper_bound;
    let y_target = y_lower_bound..=y_upper_bound;
    let result = possible_xs
        .cartesian_product(possible_ys)
        .map(|(x, y)| {
            arc(
                (x, y),
                (
                    x_lower_bound,
                    x_upper_bound,
                    y_lower_bound,
                    y_upper_bound,
                ),
            )
        })
        // filter out any arcs that don't hit target
        .filter(|arc| {
            arc.iter().any(|(x, y)| {
                let valid = x_target.contains(x)
                    && y_target.contains(y);
                valid
            })
        });
    let results = result
        .flat_map(|arc| {
            let mut it = arc.into_iter();
            it.next();
            it.next()
        })
        .collect::<Vec<(i32, i32)>>();
    // get max y value
    results.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        include_str!("./test-input.txt");

    #[test]
    fn part1_test_demo_data() {
        assert_eq!(45, process_part1(INPUT));
    }
    #[test]
    fn part2_test_demo_data() {
        assert_eq!(112, process_part2(INPUT));
    }
}
