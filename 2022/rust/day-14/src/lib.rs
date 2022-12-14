use std::collections::BTreeSet;

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete,
    character::complete::line_ending,
    multi::separated_list1, sequence::separated_pair, *,
};

fn line(
    input: &str,
) -> IResult<&str, impl Iterator<Item = (u32, u32)>> {
    let (input, pairs) = separated_list1(
        tag(" -> "),
        separated_pair(
            complete::u32,
            complete::char(','),
            complete::u32,
        ),
    )(input)?;
    let it = pairs.into_iter().tuple_windows().flat_map(
        |((ax, ay), (bx, by))| {
            let x_min = ax.min(bx);
            let x_max = ax.max(bx);
            let x_range = x_min..=x_max;

            let y_min = ay.min(by);
            let y_max = ay.max(by);
            let y_range = y_min..=y_max;
            x_range.cartesian_product(y_range)
        },
    );
    Ok((input, it))
}
fn rocks(
    input: &str,
) -> IResult<&str, BTreeSet<(u32, u32)>> {
    let (input, pairs) =
        separated_list1(line_ending, line)(input)?;
    let map = pairs.into_iter().flatten().collect();
    Ok((input, map))
}
pub fn process_part1(input: &str) -> String {
    let (_, mut board) = rocks(input).unwrap();

    let rock_count = board.len();

    let mut rocks_vec =
        board.iter().collect::<Vec<&(u32, u32)>>();
    rocks_vec.sort_by(|a, b| a.1.cmp(&b.1));
    let lowest_rock = **rocks_vec.last().unwrap();

    let mut current_sand = (500, 0);
    loop {
        // println!(
        //     "({},{})",
        //     current_sand.0, current_sand.1
        // );
        if current_sand.1 > lowest_rock.1 {
            println!("break");
            break;
        }
        let down = (current_sand.0, current_sand.1 + 1);
        let left = (current_sand.0 - 1, current_sand.1 + 1);
        let right =
            (current_sand.0 + 1, current_sand.1 + 1);
        match (
            board.get(&down),
            board.get(&left),
            board.get(&right),
        ) {
            (Some(_), Some(_), Some(_)) => {
                // no valid move
                // aka "frozen"
                board.insert(current_sand);
                current_sand = (500, 0);
            }
            (None, _, _) => {
                // println!("down {:?}", down);
                // valid down move
                current_sand = down;
            }
            (_, None, _) => {
                // println!("left");
                //valid left move
                current_sand = left;
            }
            (_, _, None) => {
                // println!("right");
                // valid right move
                current_sand = right;
            }
        };
    }

    (board.len() - rock_count).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut board) = rocks(input).unwrap();

    let rock_count = board.len();

    let mut rocks_vec =
        board.iter().collect::<Vec<&(u32, u32)>>();
    rocks_vec.sort_by(|a, b| a.1.cmp(&b.1));
    let lowest_rock = **rocks_vec.last().unwrap();

    let mut current_sand = (500, 0);
    while let None = board.get(&(500, 0)) {
        // println!(
        //     "({},{})",
        //     current_sand.0, current_sand.1
        // );
        let down = (current_sand.0, current_sand.1 + 1);
        let left = (current_sand.0 - 1, current_sand.1 + 1);
        let right =
            (current_sand.0 + 1, current_sand.1 + 1);
        match (
            board.get(&down).or_else(|| {
                if down.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
            board.get(&left).or_else(|| {
                if left.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
            board.get(&right).or_else(|| {
                if right.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
        ) {
            (Some(_), Some(_), Some(_)) => {
                // no valid move
                // aka "frozen"
                board.insert(current_sand);
                current_sand = (500, 0);
            }
            (None, _, _) => {
                // println!("down {:?}", down);
                // valid down move
                current_sand = down;
            }
            (_, None, _) => {
                // println!("left");
                //valid left move
                current_sand = left;
            }
            (_, _, None) => {
                // println!("right");
                // valid right move
                current_sand = right;
            }
        };
    }
    (board.len() - rock_count).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "24");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "93");
    }
}
