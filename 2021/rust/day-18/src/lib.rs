#![feature(box_patterns)]

use core::fmt;
use std::{cmp::Ordering, ops::Add, iter::Sum};

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
impl fmt::Display for Snailfish {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number(n) =>  write!(f, "{}", n),
            Fish((box a, box b)) =>  write!(f, "[{},{}]", a,b),
        }
       
    }
}
impl Add for Snailfish {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::Fish((Box::new(self), Box::new(other)))
    }
}
impl Sum<Snailfish> for Snailfish {
    fn sum<I>(iter: I) -> Self
    where
    I: Iterator<Item = Snailfish>
    {
        let final_fish = iter.reduce(|acc, fish| {
            println!("{} + {}", &acc, &fish);
            let result = (acc+fish).reduce_all_the_way();
            println!("result {}", &result);
            result
        });
        match final_fish {
            Some(f) => {
                f
            } 
            None => {
                dbg!(":askdfl;jasfkljas");
                Number(0)
            }
        }
    }

}
#[derive(Debug)]
enum Operation {
    Explode((Option<u32>, Option<u32>)),
    Split,
    Stop,
}
impl Snailfish {
    /// magnitude only works on reduced snailfish
    fn magnitude(&self) -> usize {
        match self {
            Fish((box Number(a), box Number(b))) => {
                let left: usize = 3_usize * (*a as usize);
                let right: usize = 2_usize * (*b as usize);
                println!("{} + {} = {}", left, right, left+right);
                left + right
            }
            Fish((fish_a, fish_b)) => {
                let left = fish_a.magnitude();
                let right = fish_b.magnitude();
                println!("mag {} + mag {} = {}", left, right, 3*left + 2*right);
                3*left + 2*right
            }
            Number(n) => *n as usize,
        }
    }
    
  
    fn reduce_all_the_way(&self) -> Self {
        println!("Starting Fish {}", &self);
        let mut fish = self.clone();
        loop {
          let new_fish = fish.reduce();
          println!("new fish {}", &new_fish);

          if new_fish == fish {
              break;
          } else {
              fish = new_fish;
          }
        };
        fish
       
    }
    fn reduce(&self) -> Self {
        let (fish, applied_operation) = self.step(0);
        match applied_operation {
            Some(Operation::Explode(_)) => fish.unwrap(),
            None => {
                let fish = self.try_split();
                fish
            },
            panic_op=> {
                dbg!(panic_op);
                panic!("shouldn't be possible to have alt operations")
            },
        }

    }
    fn try_split(&self) -> Self
    {

        match self {
            Number(num) => {
                // dbg!(num);
                Number(*num);
                if *num >= 10 {
                    let split_num = num/2;
                    Fish((
                        Box::new(Number(split_num)),
                        Box::new(Number(split_num + *num % 2))
                    ))
                } else {
                    Number(*num)
                }
            },
            Fish((fish_a, fish_b)) => {
let new_fish_a =                fish_a.try_split();
if new_fish_a != **fish_a {
    // split happened
    Fish((Box::new(new_fish_a), fish_b.clone()))
} else {
Fish((fish_a.clone(),    Box::new(fish_b.try_split())))
}
            }
        }
    }
    fn step(
        &self,
        level: usize,
    ) -> (Option<Snailfish>, Option<Operation>) {
        // dbg!(self);
        match self {
            Number(num) => {
                // dbg!(num);
                (Some(Number(*num)), None)
            },
            Fish((box Number(a), box Number(b))) => {
                // dbg!(a,b);
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
                // dbg!(fishes);
                let operation = fishes.0.step(level + 1);
                // dbg!(&operation);
                let (new_left_fish, op) = match operation {
                    (
                        new_fish,
                        Some(Operation::Explode((
                            left,
                            right,
                        ))),
                    ) => {
                        match &fishes.1 {
                            box Number(num) => {
                                // dbg!(num);

                                if let Some(op_num) = right
                                {
                                    // dbg!(op_num, num, &new_fish);

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
                            box Fish((box Number(num), fishy)) => {
                                // dbg!(num);

                                // TODO: merge top number
                                // dbg!(fishy.clone());
                                if let Some(op_num) = right
                                {
                                    // dbg!(op_num, num, &new_fish);

                                    let new_num =
                                        op_num + num;

                                    (
                                            Fish((
                                                match new_fish {
                                                    Some(fish) => Box::new(fish),
                                                    None => Box::new(Number(0)),
                                                },
                                                Box::new(Fish((
                                                    Box::new(Number(
                                                    new_num,
                                                )),fishy.clone())
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
                                            Box::new(Fish((
                                                Box::new(Number(
                                                *num,
                                            )),fishy.clone())
                                        ))
                                        )),
                                        Some(
                                            Operation::Explode(
                                                (left, None),
                                            ),
                                        ),
                                    )
                                }
                            },
                            fish => {
                                match right {
                                    Some(value) => {
                                        let fish_fish = munge_leftmost(&fish, value);
                                        ( Fish((
                                            Box::new(new_fish.unwrap()),
Box::new(                                                  fish_fish.clone())
                                       )), Some(Operation::Explode((left, None))))
                                    },
                                    None => ( Fish((
                                        Box::new(new_fish.unwrap()),
                                              fish.clone()
                                   )), Some(Operation::Explode((left, right)))),
                                }
                                
                            },
                            // fish => {
                            //     match left {
                            //         Some(value) => {
                            //             let fish_fish = munge_rightmost(&fish, value);
                            //             ( Some(Fish((
                            //                 Box::new(fish_fish.clone()),
                            //                 Box::new(new_right_fish.unwrap())
                            //      ))), Some(Operation::Explode((left, right))))
                            //         },
                            //         None =>   ( Some(Fish((
                            //             fish.clone(),
                            //             Box::new(new_right_fish.unwrap())
                            //  ))), Some(Operation::Explode((left, right)))),
                            //     }
                            //     // dbg!(&op);
                              
                            // },
                        }
                    }
                    (new_fish, Some(Operation::Split)) => {
                        todo!("split");
                    }
                    (new_fish, Some(Operation::Stop)) => (
                        new_fish.unwrap(),
                        Some(Operation::Stop),
                    ),
                    (fish, None) => {
                        // dbg!(&fish);
                        (fish.unwrap(), None)
                    }
                };
                // dbg!(&new_fish);
                match op {
                    Some(o) => (Some(new_left_fish), Some(o)),
                    None => {
                        // same as above, for right hand
                        // side
                        let operation =
                            fishes.1.step(level + 1);
                        // dbg!(&operation);

                        match operation {
                            (
                                new_right_fish,
                                Some(Operation::Explode((
                                    left,
                                    right,
                                ))),
                            ) => {
                                match &fishes.0 {
                                    box Number(num) => {
                                        // dbg!(num);
                                        if let Some(
                                            op_num,
                                        ) = left
                                        {
                                            // dbg!(
                                            //     op_num, num
                                            // );
                                            let new_num =
                                                op_num
                                                    + num;
                                            (
                                           Some(Fish((
                                                Box::new(Number(
                                                    new_num,
                                                )),
                                                match new_right_fish {
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
                                            // dbg!(&num, &new_right_fish);
                                            (
                                       Some( Fish((
                                            Box::new(Number(
                                                *num,
                                            )),
                                            Box::new(new_right_fish.unwrap()),
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
                                    box Fish((fishy, box Number(num)))=> {
                                        println!("{} -- {}", &fishy, num);

                                        // dbg!(num);
                                        if let Some(
                                            op_num,
                                        ) = left
                                        {
                                            // dbg!(
                                            //     op_num, num
                                            // );
                                            let new_num =
                                                op_num
                                                    + num;
                                            (
                                           Some(Fish((
                                            Box::new(Fish((
                                                fishy.clone(),
                                                Box::new(Number(
                                                new_num,
                                            ))))),
                                                match new_right_fish {
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
                                            // dbg!(&num, &new_right_fish);
                                            (
                                       Some( Fish((
                                           
                                        Box::new(Fish((
                                            fishy.clone(),
                                            Box::new(Number(
                                            *num,
                                        ))))),
                                            Box::new(new_right_fish.unwrap()),
                                        ))),
                                        Some(
                                            Operation::Explode(
                                                (None, right),
                                            ),
                                        ),

                                    )
                                        }

                                        // let new_op =
                                    },
                                    fish => {
                                        match left {
                                            Some(value) => {
                                                let fish_fish = munge_rightmost(&fish, value);
                                                ( Some(Fish((
                                                    Box::new(fish_fish.clone()),
                                                    Box::new(new_right_fish.unwrap())
                                         ))), Some(Operation::Explode((None, right))))
                                            },
                                            None =>   ( Some(Fish((
                                                fish.clone(),
                                                Box::new(new_right_fish.unwrap())
                                     ))), Some(Operation::Explode((left, right)))),
                                        }
                                        // dbg!(&op);
                                      
                                    },
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
fn munge_rightmost(fish: &Snailfish, value: u32) -> Snailfish {
    match fish {
        Number(n) =>panic!("helpmeee"),
        Fish((a,b)) => {
            Fish((a.clone(), match b {
                box Number(n) => Box::new(Number(n + value)),
                box fishy => Box::new(munge_rightmost(&fishy, value)),
            }))
        },
    }
}
fn munge_leftmost(fish: &Snailfish, value: u32) -> Snailfish {
    match fish {
        Number(n) =>panic!("helpmeee"),
        Fish((a,b)) => {
            Fish((match a {
                box Number(n) => Box::new(Number(n + value)),
                box fishy => Box::new(munge_leftmost(&fishy, value)),
            }, b.clone()))
        },
    }

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


// x=20..30, y=-10..-5
pub fn process_part1(input: &str) -> usize {
    let result: Snailfish = input
                                .lines()
                                .map(|line| snailfish(line).unwrap().1)
                                .sum();
    result.magnitude()
}

pub fn process_part2(input: &str) -> usize {
    // let (_input, size) = puzzle_input(input).unwrap();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        include_str!("./test-input.txt");

    // #[test]
    // fn part1_test_demo_data() {
    //     assert_eq!(4140, process_part1(INPUT));
    // }

    #[test]
    // #[ignore]
    fn test_multi_addition_reduction() {
        let TEST_INPUT = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";

let (_, answer) = snailfish("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap();
        assert_eq!(answer, TEST_INPUT
            .lines()
            .map(|line| snailfish(line).unwrap().1)
            .inspect(|s| {
                println!("inspect {}", s);
            })
            .sum())
    }

    #[test]
    fn test_multi_addition_reduction_small_A() {
        let TEST_INPUT = "[1,1]
[2,2]
[3,3]
[4,4]";

let (_, answer) = snailfish("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap();
        assert_eq!(answer, TEST_INPUT
            .lines()
            .map(|line| snailfish(line).unwrap().1)
            .sum())
    }

    #[test]
    fn test_multi_addition_reduction_small_B() {
        let TEST_INPUT = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]";

let (_, answer) = snailfish("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap();
        assert_eq!(answer, TEST_INPUT
            .lines()
            .map(|line| snailfish(line).unwrap().1)
            .sum())
    }

    #[test]
    fn test_multi_addition_reduction_small_C() {
        let TEST_INPUT = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]";

let (_, answer) = snailfish("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap();
        assert_eq!(answer, TEST_INPUT
            .lines()
            .map(|line| snailfish(line).unwrap().1)
            .sum())
    }



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

    #[test]
    fn test_add_split_and_explode() {
        let (_,  input_fish_a) =
            snailfish("[[[[4,3],4],4],[7,[[8,4],9]]]")
                .unwrap();
        let (_,  input_fish_b) =
            snailfish("[1,1]")
                .unwrap();
        let (_, after_addition) =
            snailfish("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
                .unwrap();
        let (_, after_explode_1) =
            snailfish("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]")
                .unwrap();
        let (_, after_explode_2) =
            snailfish("[[[[0,7],4],[15,[0,13]]],[1,1]]")
                .unwrap();
        let (_, after_split_1) =
            snailfish("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")
                .unwrap();
        let (_, after_split_2) =
            snailfish("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")
                .unwrap();
        let (_, after_explode_3) =
            snailfish("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
                .unwrap();
             
        println!("--step--");
        let op_fish = input_fish_a + input_fish_b;
        assert_eq!(after_addition, op_fish);
        
        println!("--step--");
        let op_fish_2 = op_fish.reduce();
        assert_eq!(after_explode_1, op_fish_2);

        println!("--step--");
        let op_fish_3 = op_fish_2.reduce();
        assert_eq!(after_explode_2, op_fish_3);

        println!("--step--");
        let op_fish_4 = op_fish_3.reduce();
        assert_eq!(after_split_1, op_fish_4);

        println!("--step--");
        let op_fish_5 = op_fish_4.reduce();
        assert_eq!(after_split_2, op_fish_5);
        
        println!("--step--");
        let op_fish_6 = op_fish_5.reduce();
        assert_eq!(after_explode_3, op_fish_6);
    }

    #[test]
    fn test_add_step_1() {
        let (_,  input_fish_a) =
            snailfish("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")
                .unwrap();
                let (_, input_fish_b) =
            snailfish("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")
                .unwrap();
        let (_, answer) =
            snailfish("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
                .unwrap();
        assert_eq!(answer.to_string(), (input_fish_a + input_fish_b).reduce_all_the_way().to_string());
    }

    
  

    #[test]
    fn test_magnitude_A() {
        let (_,  input_fish) =
            snailfish("[[1,2],[[3,4],5]]")
                .unwrap();
        
        assert_eq!(143, input_fish.magnitude());
    }
    #[test]
    fn test_magnitude_B() {
        let (_,  input_fish) =
            snailfish("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
                .unwrap();
        
        assert_eq!(1384, input_fish.magnitude());
    }
    #[test]
    fn test_magnitude_C() {
        let (_,  input_fish) =
            snailfish("[[[[1,1],[2,2]],[3,3]],[4,4]]")
                .unwrap();
        
        assert_eq!(445, input_fish.magnitude());
    }
    #[test]
    fn test_magnitude_D() {
        let (_,  input_fish) =
            snailfish("[[[[3,0],[5,3]],[4,4]],[5,5]]")
                .unwrap();
        
        assert_eq!(791, input_fish.magnitude());
    }
    #[test]
    fn test_magnitude_E() {
        let (_,  input_fish) =
            snailfish("[[[[5,0],[7,4]],[5,5]],[6,6]]")
                .unwrap();
        
        assert_eq!(1137, input_fish.magnitude());
    }
    #[test]
    fn test_magnitude_F() {
        let (_,  input_fish) =
            snailfish("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .unwrap();
        
        assert_eq!(3488, input_fish.magnitude());
    }
 
    // #[test]
    // fn part2_test_demo_data() {
    //     assert_eq!(112, process_part2(INPUT));
    // }
}
