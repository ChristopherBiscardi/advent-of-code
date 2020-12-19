// use std::collections::HashMap;
use nom::{
    branch::alt,
    character::complete::{digit1, space1},
    multi::many1,
    IResult,
};
use nom_supreme::{
    error::ErrorTree,
    final_parser::{final_parser, Location},
    tag::complete::tag,
};
use std::fmt;
#[derive(Debug, Clone)]
pub enum Calc<'a> {
    Operator(&'a str),
    Number(usize),
    SubCalcs(Vec<Calc<'a>>),
}
impl fmt::Display for Calc<'_> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Calc::Operator(op) => {
                write!(f, "{}", op)
            }
            Calc::Number(n) => {
                write!(f, "{}", n)
            }
            Calc::SubCalcs(calcs) => {
                write!(f, "(");
                for calc in calcs.iter() {
                    write!(f, "{}", calc);
                }
                write!(f, ")")
            }
        }
    }
}

fn subcalc(input: &str) -> IResult<&str, Calc, ErrorTree<&str>> {
    let (input, _) = tag("(")(input)?;
    let (input, clcs) = calcs(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Calc::SubCalcs(clcs)))
}
fn operator(input: &str) -> IResult<&str, Calc, ErrorTree<&str>> {
    let (input, _) = space1(input)?;
    let (input, op) = alt((tag("+"), tag("*")))(input)?;
    let (input, _) = space1(input)?;
    Ok((input, Calc::Operator(&op)))
}
fn calc_num(input: &str) -> IResult<&str, Calc, ErrorTree<&str>> {
    let (input, num_str) = digit1(input)?;
    let num = num_str.parse::<usize>().expect("a valid number");
    Ok((input, Calc::Number(num)))
}
fn calcs(input: &str) -> IResult<&str, Vec<Calc>, ErrorTree<&str>> {
    many1(alt((operator, calc_num, subcalc)))(input)
}

pub fn parse(input: &str) -> Result<Vec<Calc>, ErrorTree<Location>> {
    final_parser(calcs)(input)
}
