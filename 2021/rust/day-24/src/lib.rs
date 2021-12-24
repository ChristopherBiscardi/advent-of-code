use core::fmt;
use itertools::{Itertools, iproduct};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap, VecDeque},
    iter::{self, Sum},
    ops::Add,
};

use nom::{
    branch::alt,
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
struct ALU {
    input: VecDeque<i32>,
    state: BTreeMap<char, i32>,
}
#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Input(char),
    Add((char, Argument)),
    Multiply((char, Argument)),
    Divide((char, Argument)),
    Mod((char, Argument)),
    Equal((char, Argument)),
}
use Instruction::*;

#[derive(Debug, Eq, PartialEq)]
enum Argument {
    Name(char),
    Number(i32),
}
impl ALU {
    fn new(input: VecDeque<i32>) -> Self {
        let map = BTreeMap::from([
            ('x', 0),
            ('y', 0),
            ('z', 0),
            ('w', 0),
        ]);
        ALU { input, state: map }
    }
    fn apply_instructions(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            self.apply(instruction);
        }
    }
    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Input(key) => match self.input.pop_front() {
                Some(value) => {
                    self.state.entry(*key).and_modify(|v| {
                    *v = value;
                });
            },
                None => panic!("input number not available for Input instruction. perhaps the input was too short."),
            },
            Instruction::Add((key, argument)) => {
                let value = match argument {
                    Argument::Name(name) => {
                        *self.state.get(&name).unwrap()
                    },
                    Argument::Number(num) => {
                       *num
                    },
                };
                self.state.entry(*key).and_modify(|v| {
                   *v += value;
                });
            },
            Multiply((key, argument)) => {
                let value = match argument {
                    Argument::Name(name) => {
                        *self.state.get(&name).unwrap()
                    },
                    Argument::Number(num) => {
                       *num
                    },
                };
                self.state.entry(*key).and_modify(|v| {
                   *v *= value;
                });
            },
            Divide((key, argument)) => {
                let value = match argument {
                    Argument::Name(name) => {
                        *self.state.get(&name).unwrap()
                    },
                    Argument::Number(num) => {
                       *num
                    },
                };
                self.state.entry(*key).and_modify(|v| {
                   *v /= value;
                });
            },
            Mod((key, argument)) => {
                let value = match argument {
                    Argument::Name(name) => {
                        *self.state.get(&name).unwrap()
                    },
                    Argument::Number(num) => {
                       *num
                    },
                };
                self.state.entry(*key).and_modify(|v| {
                   *v = *v % value;
                });
            },
            Equal((key, argument)) => {
                let value = match argument {
                    Argument::Name(name) => {
                        *self.state.get(&name).unwrap()
                    },
                    Argument::Number(num) => {
                       *num
                    },
                };
                self.state.entry(*key).and_modify(|v| {
                   *v = if *v == value {
                       1
                   } else { 
                       0
                   };
                });
            },
        }
    }
}
fn puzzle_input(
    input: &str,
) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) =
        separated_list1(newline, instruction)(input)?;
    Ok((input, instructions))
}

pub fn process_part1(input: &str) -> usize {
    let (_, instructions) =
        puzzle_input(input).expect("input to be valid");
        let range = (1..10).rev();
        
        for a in range.clone() {
            for b in range.clone() {
                for c in range.clone() {
                    for d in range.clone() {
                        for e in range.clone() {
                            for f in range.clone() {
    dbg!(&f);
                                for g in range.clone() {
    dbg!(&g);
                                    for h in range.clone() {
    dbg!(&h);
                                        for i in range.clone() {
                                            for j in range.clone() {
                                                for k in range.clone() {
                                                    for l in range.clone() {
                                                        for m in range.clone() {
                                                            for n in range.clone() {
    let input = vec![a,b,c,d,e,f,g,h,i,j,k,l,m,n];
let has_zero = input.iter().any(|num| *num == 0);
if has_zero {
    continue
} else {
    let mut alu = ALU::new(VecDeque::from(input));
    alu.apply_instructions(&instructions);
    if *alu.state.get(&'z').unwrap() == 0 {
        dbg!(alu);
        break;
    }
}
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

   
    0
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, name) = alt((
        tag("inp"),
        tag("add"),
        tag("mul"),
        tag("div"),
        tag("mod"),
        tag("eql"),
    ))(input)?;

    let (input, _) = tag(" ")(input)?;

    if name == "inp" {
        let (input, state_var) = anychar(input)?;
        Ok((input, Input(state_var)))
    } else {
        let (input, (a, b)) =
            separated_pair(anychar, tag(" "), argument)(
                input,
            )?;
        let instruction = match name {
            "add" => Add((a, b)),
            "mul" => Multiply((a, b)),
            "div" => Divide((a, b)),
            "mod" => Mod((a, b)),
            "eql" => Equal((a, b)),
            _ => panic!("invalid input"),
        };
        Ok((input, instruction))
    }
}
fn argument(input: &str) -> IResult<&str, Argument> {
    let result: IResult<&str, i32> = complete::i32(input);
    match result {
        Ok((input, num)) => {
            Ok((input, Argument::Number(num)))
        }
        Err(_) => {
            let (input, c) = anychar(input)?;
            Ok((input, Argument::Name(c)))
        }
    }
}
pub fn process_part2(input: &str) -> usize {
    let (_, commands) =
        puzzle_input(input).expect("input to be valid");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        include_str!("./test-input.txt");
    #[test]
    fn part1_test_demo_data() {
        assert_eq!(1, process_part1(INPUT));
    }

    #[test]
    fn test_binary_program() {
        let (_, instructions) =
        puzzle_input(INPUT).expect("input to be valid");
    let mut alu = ALU::new(VecDeque::from([13]));
    alu.apply_instructions(&instructions);
    let output_str = alu.state.iter().map(|(_,v)| v.to_string()).collect::<String>();
        assert_eq!(13, i32::from_str_radix(&output_str, 2).unwrap());
    }

    #[test]
    fn test_parse_instruction_inp() {
        let test_input = "inp w";

        assert_eq!(
            Input('w'),
            instruction(test_input).unwrap().1
        );
    }
    #[test]
    fn test_parse_instruction_add() {
        let test_input = "add z w";
        assert_eq!(
            Add(('z', Argument::Name('w'))),
            instruction(test_input).unwrap().1
        );
    }
    #[test]
    fn test_parse_instruction_inp_add() {
        let test_input = "inp w\nadd z w";
        assert_eq!(
            vec![
                Input('w'),
                Add(('z', Argument::Name('w')))
            ],
            puzzle_input(test_input).unwrap().1
        );
    }
    // #[test]
    // fn part2_test_demo_data() {
    //     assert_eq!(3351, process_part2(INPUT));
    // }
}
