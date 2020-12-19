// use std::collections::HashMap;
use nom::{
    branch::alt,
    character::complete::{digit1, space1},
    combinator::{opt, peek, recognize},
    multi::many1,
    sequence::tuple,
    IResult,
};
use nom_supreme::{
    error::ErrorTree,
    final_parser::{final_parser, Location},
    tag::complete::tag,
};

use crate::libpart1::Calc;

fn plus_num(input: &str) -> IResult<&str, Vec<Calc>, ErrorTree<&str>> {
    let (input, op) = recognize(tuple((space1, tag("+"), space1, digit1)))(input)?;
    let (next_input, o) = operator(op)?;
    let (_, n) = calc_num(next_input)?;

    Ok((input, vec![o, n]))
}

fn subcalc(input: &str) -> IResult<&str, Calc, ErrorTree<&str>> {
    let (input, _) = tag("(")(input)?;
    let (input, clcs) = calcs(input)?;
    let (input, _) = tag(")")(input)?;

    // if after subcalc has a `+ n`, then group all fo that
    let (input, things) = opt(many1(plus_num))(input)?;

    match things {
        Some(ts) => {
            let post_calcs = ts.into_iter().flatten().collect::<Vec<Calc>>();
            let mut calc_vec = vec![Calc::SubCalcs(clcs)];
            for calc in post_calcs {
                calc_vec.push(calc);
            }
            Ok((input, Calc::SubCalcs(calc_vec)))
        }
        None => Ok((input, Calc::SubCalcs(clcs))),
    }
    // for c in post_calcs.iter() {
    //     post_calcs.push(c);
    // }
}

fn num_plus(input: &str) -> IResult<&str, Vec<Calc>, ErrorTree<&str>> {
    let (input, op) = recognize(tuple((digit1, space1, tag("+"), space1)))(input)?;
    let (next_input, n) = calc_num(op)?;
    let (_, o) = operator(next_input)?;
    Ok((input, vec![n, o]))
}

fn subcalc_precedence(input: &str) -> IResult<&str, Calc, ErrorTree<&str>> {
    let (input, things) = many1(num_plus)(input)?;
    let mut t = things.into_iter().flatten().collect::<Vec<Calc>>();
    // num or calcs
    let ((pk_input, pk)) = opt(calc_num)(input)?;
    match pk {
        Some(dgt) => {
            t.push(dgt);
            Ok((pk_input, Calc::SubCalcs(t)))
        }
        None => {
            let (input, clcs) = calcs(input)?;
            if clcs.len() == 1 {
                t.push(clcs[0].clone());
                Ok((input, Calc::SubCalcs(t)))
            } else {
                t.push(Calc::SubCalcs(clcs));
                Ok((input, Calc::SubCalcs(t)))
            }
        }
    }
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
    many1(alt((subcalc_precedence, operator, calc_num, subcalc)))(input)
}

pub fn parse(input: &str) -> Result<Vec<Calc>, ErrorTree<Location>> {
    let res = final_parser(calcs)(input)?;
    //     println!(
    //         "{},
    // {}",
    //         input,
    //         &res.iter().map(|v| format!("{}", v)).collect::<String>()
    //     );
    Ok(res)
}
