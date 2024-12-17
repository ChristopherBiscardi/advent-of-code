use derive_more::derive::TryFrom;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{
        self, line_ending, multispace1, one_of,
    },
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use tracing::{info, instrument};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, (mut registers, instructions)) =
        parse(input).map_err(|e| {
            miette::miette!("parse failed {}", e)
        })?;

    let outputs = run(&mut registers, &instructions);
    Ok(outputs.to_string())
}

fn run(
    registers: &mut Registers,
    instructions: &[Instruction],
) -> String {
    let mut outputs = vec![];

    while registers.pointer < instructions.len() {
        if let Some(output) = registers.op(
            &instructions[registers.pointer],
            &instructions[registers.pointer + 1],
        ) {
            outputs.push(output);
        }
        info!(?registers);
    }

    outputs.into_iter().join(",")
}

#[derive(Debug)]
struct Registers {
    a: i32,
    b: i32,
    c: i32,
    pointer: usize,
}

impl Registers {
    fn move_to_next_instruction(&mut self) {
        self.pointer += 2;
    }
    fn combo(&self, operand: &Instruction) -> i32 {
        match *operand as u32 {
            n if (0..=3).contains(&n) => n as i32,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            n => {
                unreachable!("something is wrong, {n}")
            }
        }
    }
    #[instrument(ret)]
    fn op(
        &mut self,
        instruction: &Instruction,
        operand: &Instruction,
    ) -> Option<i32> {
        match instruction {
            Instruction::Adv => {
                self.a /=
                    2i32.pow(self.combo(operand) as u32);
                self.move_to_next_instruction();
            }
            Instruction::Bxl => {
                self.b ^= *operand as i32;
                self.move_to_next_instruction();
            }
            Instruction::Bst => {
                self.b = self.combo(operand) % 8;
                self.move_to_next_instruction();
            }
            Instruction::Jnz => {
                if self.a == 0 {
                    self.move_to_next_instruction();
                    return None;
                }
                self.pointer = *operand as usize;
            }
            Instruction::Bxc => {
                self.b ^= self.c;
                self.move_to_next_instruction();
            }
            Instruction::Out => {
                self.move_to_next_instruction();
                return Some(self.combo(operand) % 8);
            }
            Instruction::Bdv => {
                self.b = self.a
                    / 2i32.pow(self.combo(operand) as u32);
                self.move_to_next_instruction();
            }
            Instruction::Cdv => {
                self.c = self.a
                    / 2i32.pow(self.combo(operand) as u32);
                self.move_to_next_instruction();
            }
        }
        None
    }
}

#[derive(TryFrom, Debug, Clone, Copy)]
#[try_from(repr)]
#[repr(u32)]
enum Instruction {
    /// The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
    Adv = 0,
    /// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
    Bxl = 1,
    /// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    Bst = 2,
    /// The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
    Jnz = 3,
    /// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
    Bxc = 4,
    /// The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
    Out = 5,
    /// The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
    Bdv = 6,
    /// The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
    Cdv = 7,
}

fn registers(input: &str) -> IResult<&str, Registers> {
    let (input, a) = delimited(
        tag("Register A: "),
        complete::i32,
        line_ending,
    )(input)?;
    let (input, b) = delimited(
        tag("Register B: "),
        complete::i32,
        line_ending,
    )(input)?;
    let (input, c) = delimited(
        tag("Register C: "),
        complete::i32,
        line_ending,
    )(input)?;
    Ok((
        input,
        Registers {
            a,
            b,
            c,
            pointer: 0,
        },
    ))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, digit) = one_of("01234567")(input)?;
    let ins =
        Instruction::try_from(digit.to_digit(10).unwrap())
            .unwrap();
    Ok((input, ins))
}

fn parse(
    input: &str,
) -> IResult<&str, (Registers, Vec<Instruction>)> {
    let (input, (registers, instructions)) =
        separated_pair(
            registers,
            multispace1,
            preceded(
                tag("Program: "),
                separated_list1(tag(","), instruction),
            ),
        )(input)?;

    let (input, _) =
        all_consuming(opt(line_ending))(input)?;

    Ok((input, (registers, instructions)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input)?);
        Ok(())
    }

    #[test]
    fn testn_01() -> miette::Result<()> {
        let input = "Register A: 0
Register B: 0
Register C: 9
        
Program: 2,6";
        let (_input, (mut registers, instructions)) =
            parse(input).map_err(|e| {
                miette::miette!("parse failed {}", e)
            })?;

        run(&mut registers, &instructions);

        assert_eq!(registers.b, 1);
        Ok(())
    }

    #[test]
    fn testn_02() -> miette::Result<()> {
        let input = "Register A: 10
Register B: 0
Register C: 0
        
Program: 5,0,5,1,5,4";
        let (_input, (mut registers, instructions)) =
            parse(input).map_err(|e| {
                miette::miette!("parse failed {}", e)
            })?;

        let output = run(&mut registers, &instructions);

        assert_eq!("0,1,2", output);
        Ok(())
    }
    #[test_log::test]
    fn testn_03() -> miette::Result<()> {
        let input = "Register A: 2024
Register B: 0
Register C: 0
        
Program: 0,1,5,4,3,0";
        let (_input, (mut registers, instructions)) =
            parse(input).map_err(|e| {
                miette::miette!("parse failed {}", e)
            })?;

        let output = run(&mut registers, &instructions);

        assert_eq!(registers.a, 0);
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", output);
        Ok(())
    }
    #[test_log::test]
    fn testn_04() -> miette::Result<()> {
        let input = "Register A: 0
Register B: 29
Register C: 0
        
Program: 1,7";
        let (_input, (mut registers, instructions)) =
            parse(input).map_err(|e| {
                miette::miette!("parse failed {}", e)
            })?;

        run(&mut registers, &instructions);

        assert_eq!(registers.b, 26);
        Ok(())
    }
    #[test_log::test]
    fn testn_05() -> miette::Result<()> {
        let input = "Register A: 0
Register B: 2024
Register C: 43690
        
Program: 4,0";
        let (_input, (mut registers, instructions)) =
            parse(input).map_err(|e| {
                miette::miette!("parse failed {}", e)
            })?;

        run(&mut registers, &instructions);

        assert_eq!(registers.b, 44354);
        Ok(())
    }
}
