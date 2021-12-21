use core::fmt;
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    iter::{self, Sum},
    ops::Add,
};

use nom::{
    bytes::complete::{tag, take},
    character::complete::{
        self, alpha1, anychar, newline, one_of, u32,
    },
    combinator::opt,
    multi::{
        length_value, many0, many1, many_m_n,
        separated_list1,
    },
    sequence::{
        pair, preceded, separated_pair, terminated,
    },
    IResult,
};

#[derive(Debug)]
struct Player {
    id: u64,
    start_position: u64,
    position: u64,
    score: u64,
}
fn player(input: &str) -> IResult<&str, Player> {
    let (input, _) = tag("Player ")(input)?;
    let (input, id) = complete::u64(input)?;
    let (input, _) = tag(" starting position: ")(input)?;
    let (input, start_position) = complete::u64(input)?;

    Ok((
        input,
        Player {
            id: id,
            start_position: start_position,
            position: start_position,
            score: 0,
        },
    ))
}
fn puzzle_input(input: &str) -> IResult<&str, Vec<Player>> {
    let (input, players) =
        separated_list1(newline, player)(input)?;
    Ok((input, players))
}

pub fn process_part1(input: &str) -> usize {
    let (_, mut players) =
        puzzle_input(input).expect("input to be valid");
    let it = (1..=100).cycle();
    let mut num_rolls = 0;
    for (i, chunk) in it.chunks(3).into_iter().enumerate() {
        num_rolls += 3;
        // let player_id = (i % 2) + 1;
        let mut player = players
            .get_mut(i % 2)
            .expect("an existing player");
        let rolls: u64 = chunk.into_iter().sum();

        let full_position = player.position + rolls;
        let new_position = match full_position % 10 {
            0 => 10,
            n => {
                if n > 9 {
                    panic!("bad num")
                } else {
                    n
                }
            }
        };
        (*player).position = new_position;
        (*player).score += new_position;
        // dbg!(new_position);
        if player.score >= 1000 {
            break;
        }
    }
    dbg!(&players);

    let loser = players
        .iter()
        .find(|player| player.score < 1000)
        .unwrap();

    (loser.score * num_rolls).try_into().unwrap()
}

fn quantum_rolls() -> HashMap<u64, usize> {
    let it = (1..=3)
        .cartesian_product(1_u64..=3_u64)
        .cartesian_product(1_u64..=3_u64);

    let n: HashMap<u64, usize> = it
        .map(|((a, b), c)| a + b + c)
        // .sorted()
        .counts();
    n
}
pub fn process_part2(input: &str) -> usize {
    let (_, mut players) =
        puzzle_input(input).expect("input to be valid");
    let mut player_1_map = BTreeMap::new();
    player_1_map.insert((players[0].start_position, 0), 1);

    let mut tree = player_1_map;
    for i in 1.. {
        tree = step(&tree);
        if tree
            .iter()
            .all(|((_pos, score), _count)| score > &21)
        {
            dbg!((27_usize).pow(i));
            break;
        }
        // {
        //     Some(val) => {
        //         dbg!((27_usize).pow(i), val);
        //         break;
        //     }
        //     None => {}
        // }
        // dbg!((27_usize).pow(i));
    }
    // step for player 1
    // let tree = step(&player_1_map);
    // dbg!(&tree);
    // let tree = step(&tree);
    // let tree = step(&tree);
    let sum: usize = tree
        .iter()
        .map(|((pos, score), count)| count)
        .sum();
    dbg!(sum);
    // dbg!(new_player_1_map);

    // HashMap(player position, count of players)
    // iter
    // new hashmap
    // player count on position *

    // 1,2,3,4,5,6,7,8,9,10

    0
}
fn step(
    map: &BTreeMap<(u64, u64), usize>,
) -> BTreeMap<(u64, u64), usize> {
    let rolls = quantum_rolls();
    let mut new_player_1_map = BTreeMap::new();
    for ((position, score), count) in map.iter() {
        for (roll, roll_count) in rolls.iter() {
            let new_pos = match (position + roll) % 10 {
                0 => 10,
                n => {
                    if n > 9 {
                        panic!("bad num")
                    } else {
                        n
                    }
                }
            };
            // println!("{}:{}", new_pos, new_pos + score);
            // dbg!(new_player_1_map
            //     .get(&(new_pos, new_pos + score)));
            new_player_1_map
                .entry((new_pos, new_pos + score))
                .and_modify(|val| {
                    *val += count * roll_count
                })
                .or_insert(count * roll_count);
        }
    }
    new_player_1_map
}
// step 0 = 1
// player 1
// 7
// player 2
// 49
// player -> calculate -> 7 players

// pub fn step(input: &str) -> impl Iterator {
//     for i in 0.. {
//         // let player_id = (i % 2) + 1;
//         let mut player = players
//             .get_mut(i % 2)
//             .expect("an existing player");
//         let rolls: u64 = chunk.into_iter().sum();

//         let full_position = player.position + rolls;
//         let new_position = match full_position % 10 {
//             0 => 10,
//             n => {
//                 if n > 9 {
//                     panic!("bad num")
//                 } else {
//                     n
//                 }
//             }
//         };
//         (*player).position = new_position;
//         (*player).score += new_position;
//         // dbg!(new_position);
//         if player.score >= 1000 {
//             break;
//         }
//     }
//     dbg!(&players);

//     let loser = players
//         .iter()
//         .find(|player| player.score < 1000)
//         .unwrap();

//     (loser.score * num_rolls).try_into().unwrap()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        include_str!("./test-input.txt");

    #[test]
    fn part1_test_demo_data() {
        assert_eq!(739785, process_part1(INPUT));
    }

    #[test]
    fn part2_test_demo_data() {
        assert_eq!(3351, process_part2(INPUT));
    }

    // #[test]
    // fn test_quantum_die() {
    //     assert_eq!(HashMap::new(), quantum_rolls());
    // }
}
