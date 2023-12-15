use itertools::Itertools;
use nom::{
    bytes::complete::is_a,
    character::complete::{self, alpha1},
    combinator::opt,
    multi::separated_list1,
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug)]
enum Operation {
    Insert(u8),
    Remove,
}
#[derive(Debug)]
struct Instruction<'a> {
    label: &'a str,
    hash: u8,
    operation: Operation,
}

#[derive(Debug, Eq, PartialEq)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

impl<'a> TryFrom<&Instruction<'a>> for Lens<'a> {
    type Error = String;

    fn try_from(
        value: &Instruction<'a>,
    ) -> Result<Self, Self::Error> {
        match value.operation {
            Operation::Insert(focal_length) => Ok(Lens {
                label: value.label,
                focal_length,
            }),
            Operation::Remove => Err(format!(
                "invalid conversion for {}",
                value.label
            )),
        }
    }
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, label) = alpha1(input)?;
    let (input, _operation) = is_a("-=")(input)?;
    let (input, focal_length) = opt(complete::u8)(input)?;
    let op = match focal_length {
        Some(num) => Operation::Insert(num),
        None => Operation::Remove,
    };

    Ok((
        input,
        Instruction {
            label,
            hash: label
                .chars()
                .fold(0usize, |acc, next_char| {
                    (acc + (next_char as usize)) * 17 % 256
                })
                .try_into()
                .expect("should resolve to a u8"),
            operation: op,
        },
    ))
}

fn instructions(
    input: &str,
) -> IResult<&str, Vec<Instruction>> {
    separated_list1(complete::char(','), instruction)(input)
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_, ins) =
        instructions(input).expect("should parse");
    let boxes: Vec<Vec<Lens>> =
        (0..256).into_iter().map(|_| vec![]).collect();
    let filled_boxes = ins.iter().fold(
        boxes,
        |mut boxes, next_instruction| {
            match Lens::try_from(next_instruction) {
                Ok(lens) => {
                    // insert or replace
                    let index = boxes
                        [next_instruction.hash as usize]
                        .iter()
                        .position(|a_lens| {
                            a_lens.label == lens.label
                        });
                    match index {
                        Some(lens_index) => {
                            //replace
                            let _ = std::mem::replace(
                                &mut boxes[next_instruction
                                    .hash
                                    as usize][lens_index],
                                lens,
                            );
                        }
                        None => {
                            // insert
                            boxes[next_instruction.hash
                                as usize]
                                .push(lens);
                        }
                    }
                }
                Err(_) => {
                    // remove
                    let r#box = &mut boxes
                        [next_instruction.hash as usize];

                    r#box.retain(|lens| {
                        lens.label != next_instruction.label
                    });
                }
            }
            boxes
        },
    );

    let result = filled_boxes
        .into_iter()
        .enumerate()
        .flat_map(|(box_position, r#box)| {
            r#box.into_iter().enumerate().map(
                move |(position, lens)| {
                    let result = (box_position + 1)
                        * (position + 1)
                        * (lens.focal_length as usize);

                    result
                },
            )
        })
        .sum::<usize>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("145", process(input)?);
        Ok(())
    }
}
