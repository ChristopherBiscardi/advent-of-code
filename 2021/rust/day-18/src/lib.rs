#![feature(box_patterns)]

use std::{cmp::Ordering, ops::Add};

use itertools::Itertools;
use ndarray::{concatenate, Array2, Axis};
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

#[derive(Debug, Eq, PartialEq, Clone)]
enum Snailfish {
    Number(u32),
    Fish((Box<Snailfish>, Box<Snailfish>)),
}
use Snailfish::*;
impl Add for Snailfish {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::Fish((Box::new(self), Box::new(other)))
    }
}

#[derive(Debug)]
enum Operation {
    Explode((Option<u32>, Option<u32>)),
    Split,
    Stop,
}
impl Snailfish {
    fn reduce(&self) -> Self {
        let (fish, _) = self.step(0);
        fish.unwrap()
    }
    fn step(
        &self,
        level: usize,
    ) -> (Option<Snailfish>, Option<Operation>) {
        // dbg!(self);
        match self {
            Number(num) => (Some(Number(*num)), None),
            Fish((box Number(a), box Number(b))) => {
                if level >= 4 {
                    (
                        None,
                        Some(Operation::Explode((
                            Some(*a),
                            Some(*b),
                        ))),
                    )
                } else {
                    (
                        Some(Fish((
                            Box::new(Number(*a)),
                            Box::new(Number(*b)),
                        ))),
                        None,
                    )
                }
            }
            Fish(fishes) => {
                dbg!(fishes);
                let operation = fishes.0.step(level + 1);
                // dbg!(&operation);
                let (new_fish, op) = match operation {
                    (
                        new_fish,
                        Some(Operation::Explode((
                            left,
                            right,
                        ))),
                    ) => {
                        match &fishes.1 {
                            box Number(num) => {

                                if let Some(op_num) = right
                                {
                                    dbg!(op_num, num, &new_fish);

                                    let new_num =
                                        op_num + num;

                                    (
                                            Fish((
                                                match new_fish {
                                                    Some(fish) => Box::new(fish),
                                                    None => Box::new(Number(0)),
                                                },
                                                Box::new(Number(
                                                    new_num,
                                                )),
                                            )),
                                            Some(
                                                Operation::Explode(
                                                    (left, None),
                                                ),
                                            ),
                                        )
                                } else {
                                    (
                                        Fish((
                                            Box::new(new_fish.unwrap()),
                                            Box::new(Number(
                                                *num,
                                            )),
                                        )),
                                        Some(
                                            Operation::Explode(
                                                (left, None),
                                            ),
                                        ),
                                    )
                                }

                                // let new_op =
                            }
                            box Fish(fishy) => {
                                dbg!(fishy.clone());
                                if let Some(op_num) = right
                                {
                                    dbg!(op_num, num, &new_fish);

                                    let new_num =
                                        op_num + num;

                                    (
                                            Fish((
                                                match new_fish {
                                                    Some(fish) => Box::new(fish),
                                                    None => Box::new(Number(0)),
                                                },
                                                Box::new(Number(
                                                    new_num,
                                                )),
                                            )),
                                            Some(
                                                Operation::Explode(
                                                    (left, None),
                                                ),
                                            ),
                                        )
                                } else {
                                    (
                                        Fish((
                                            Box::new(new_fish.unwrap()),
                                            Box::new(Number(
                                                *num,
                                            )),
                                        )),
                                        Some(
                                            Operation::Explode(
                                                (left, None),
                                            ),
                                        ),
                                    )
                                }
                            },
                            _ => panic!("asfkj"),
                        }
                    }
                    (new_fish, Some(Operation::Split)) => {
                        todo!("split");
                    }
                    (new_fish, Some(Operation::Stop)) => (
                        new_fish.unwrap(),
                        Some(Operation::Stop),
                    ),
                    (fish, None) => (fish.unwrap(), None),
                };
                dbg!(&new_fish);
                match op {
                    Some(o) => (Some(new_fish), Some(o)),
                    None => {
                        // same as above, for right hand
                        // side
                        let operation =
                            fishes.1.step(level + 1);
                        dbg!(&operation);

                        match operation {
                            (
                                new_fish,
                                Some(Operation::Explode((
                                    left,
                                    right,
                                ))),
                            ) => {
                                match &fishes.0 {
                                    box Number(num) => {
                                        if let Some(
                                            op_num,
                                        ) = left
                                        {
                                            dbg!(
                                                op_num, num
                                            );
                                            let new_num =
                                                op_num
                                                    + num;
                                            (
                                           Some(Fish((
                                                Box::new(Number(
                                                    new_num,
                                                )),
                                                match new_fish {
                                                    Some(fish) => Box::new(fish),
                                                    None => Box::new(Number(0)),
                                                },
                                               
                                            ))),
                                            Some(
                                                Operation::Explode(
                                                    (None,right),
                                                ),
                                            ),
                                        )
                                        } else {
                                            dbg!(&num, &new_fish);
                                            (
                                       Some( Fish((
                                           
                                            Box::new(Number(
                                                *num,
                                            )),
                                            Box::new(new_fish.unwrap()),
                                        ))),
                                        Some(
                                            Operation::Explode(
                                                (None, right),
                                            ),
                                        ),

                                    )
                                        }

                                        // let new_op =
                                    }
                                    box Fish(fishy) => {
                                        dbg!(fishy.clone());
                                        todo!();
                                    },
                                    _ => panic!("asfkj"),
                                }
                            }
                            (
                                new_fish,
                                Some(Operation::Split),
                            ) => {
                                todo!("split");
                            }
                            (
                                new_fish,
                                Some(Operation::Stop),
                            ) => (
                                new_fish,
                                Some(Operation::Stop),
                            ),
                            (fish, None) => (fish, None),
                        }
                    }
                }
            }
        }
    }
    fn explode(&self) {}
    fn split() {}
}
fn snailfish(input: &str) -> IResult<&str, Snailfish> {
    let has_snailfish: IResult<&str, &str> =
        tag("[")(input);

    let (input, fish_number) = match has_snailfish {
        Ok((input, _)) => {
            let (input, fish) = separated_pair(
                snailfish,
                tag(","),
                snailfish,
            )(input)?;
            let (input, _) = tag("]")(input)?;
            (
                input,
                Fish((Box::new(fish.0), Box::new(fish.1))),
            )
        }
        Err(_) => {
            let (input, num) = complete::u32(input)?;
            (input, Number(num))
        }
    };

    Ok((input, fish_number))
}
fn puzzle_input(input: &str) -> IResult<&str, usize> {
    Ok((input, 0))
}

// x=20..30, y=-10..-5
pub fn process_part1(input: &str) -> i32 {
    let (_input, size) = puzzle_input(input).unwrap();
    0
}

pub fn process_part2(input: &str) -> usize {
    let (_input, size) = puzzle_input(input).unwrap();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        include_str!("./test-input.txt");

    // #[test]
    // fn part1_test_demo_data() {
    //     assert_eq!(45, process_part1(INPUT));
    // }

    #[test]
    fn test_parse_snailfish_A() {
        let (input, fish) = snailfish("[1,2]").unwrap();
        assert_eq!(
            Fish((
                Box::new(Number(1)),
                Box::new(Number(2))
            )),
            fish
        );
    }

    #[test]
    fn test_parse_snailfish_B() {
        let (input, fish) = snailfish("[[1,2],3]").unwrap();
        assert_eq!(
            Fish((
                Box::new(Fish((
                    Box::new(Number(1)),
                    Box::new(Number(2))
                ))),
                Box::new(Number(3))
            )),
            fish
        );
    }

    //
    // [9,[8,7]]
    // [[1,9],[8,5]]
    // [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
    // [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
    //

    //[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]
    #[test]
    fn test_parse_snailfish_F() {
        let (input, fish) = snailfish("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]").unwrap();
        let test_fish = Fish((
            Box::new(Fish((
                Box::new(Fish((
                    Box::new(Fish((
                        Box::new(Number(1)),
                        Box::new(Number(3)),
                    ))),
                    Box::new(Fish((
                        Box::new(Number(5)),
                        Box::new(Number(3)),
                    ))),
                ))),
                Box::new(Fish((
                    Box::new(Fish((
                        Box::new(Number(1)),
                        Box::new(Number(3)),
                    ))),
                    Box::new(Fish((
                        Box::new(Number(8)),
                        Box::new(Number(7)),
                    ))),
                ))),
            ))),
            Box::new(Fish((
                Box::new(Fish((
                    Box::new(Fish((
                        Box::new(Number(4)),
                        Box::new(Number(9)),
                    ))),
                    Box::new(Fish((
                        Box::new(Number(6)),
                        Box::new(Number(9)),
                    ))),
                ))),
                Box::new(Fish((
                    Box::new(Fish((
                        Box::new(Number(8)),
                        Box::new(Number(2)),
                    ))),
                    Box::new(Fish((
                        Box::new(Number(7)),
                        Box::new(Number(3)),
                    ))),
                ))),
            ))),
        ));
        assert_eq!(test_fish, fish);
    }

    #[test]
    fn test_add_basic() {
        let (_, a) = snailfish("[1,2]").unwrap();
        let (_, b) = snailfish("[[3,4],5]").unwrap();
        assert_eq!(
            Fish((
                Box::new(Fish((
                    Box::new(Number(1)),
                    Box::new(Number(2))
                ))),
                Box::new(Fish((
                    Box::new(Fish((
                        Box::new(Number(3)),
                        Box::new(Number(4))
                    ))),
                    Box::new(Number(5))
                )))
            )),
            a + b
        );
    }

    #[test]
    fn test_add_explode_left() {
        let (_,  input_fish) =
            snailfish("[[[[[9,8],1],2],3],4]").unwrap();
        let (_, answer) =
            snailfish("[[[[0,9],2],3],4]").unwrap();
        assert_eq!(answer, input_fish.reduce());
    }
    #[test]
    fn test_add_explode_right() {
        let (_,  input_fish) =
            snailfish("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        let (_, answer) =
            snailfish("[7,[6,[5,[7,0]]]]").unwrap();
        assert_eq!(answer, input_fish.reduce());
    }

    #[test]
    fn test_add_explode_both() {
        let (_,  input_fish) =
            snailfish("[[6,[5,[4,[3,2]]]],1]").unwrap();
        let (_, answer) =
            snailfish("[[6,[5,[7,0]]],3]").unwrap();
        assert_eq!(answer, input_fish.reduce());
    }
    #[test]
    fn test_add_explode_unaffected() {
        let (_,  input_fish) = snailfish(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        )
        .unwrap();
        let (_, answer) =
            snailfish("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
                .unwrap();
        assert_eq!(answer, input_fish.reduce());
    }
    #[test]
    fn test_add_explode_something() {
        let (_,  input_fish) =
            snailfish("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
                .unwrap();
        let (_, answer) =
            snailfish("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
                .unwrap();
        assert_eq!(answer, input_fish.reduce());
    }

    // #[test]
    // fn part2_test_demo_data() {
    //     assert_eq!(112, process_part2(INPUT));
    // }
}
