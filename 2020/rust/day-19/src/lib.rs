#![feature(type_ascription)]

use itertools::Itertools;

use nom::{
    branch::alt,
    bytes::complete::{take_until, take_while},
    character::{
        complete::{alpha1, char, digit1, multispace1, newline},
        is_alphabetic, is_space,
    },
    combinator::{opt, recognize},
    multi::{many1, separated_list1},
    IResult,
};
// use nom_locate::{position, LocatedSpan};
use nom_supreme::{
    error::ErrorTree,
    final_parser::{final_parser, Location},
    tag::complete::tag,
};
use std::convert::TryInto;

use std::collections::HashMap;
#[derive(Debug)]
enum Rule<'a> {
    Nums(&'a str),
    Alternate(&'a str),
    Character(&'a str),
}
/// parses
/// ```
/// 1 3
/// ```
///
fn nums(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    recognize(separated_list1(tag(" "), digit1))(input)
}
fn nums_rule(input: &str) -> IResult<&str, Rule, ErrorTree<&str>> {
    recognize(separated_list1(tag(" "), digit1))(input)
        .map(|(input, output)| (input, Rule::Nums(output)))
}
/// parses
///
/// ```
/// 1 3 | 3 1
/// ```
///
/// and is responsible for just the separator, delegating
/// the number parsing to nums
fn or(input: &str) -> IResult<&str, Rule, ErrorTree<&str>> {
    recognize(separated_list1(tag(" | "), nums))(input)
        .map(|(input, output)| (input, Rule::Alternate(output)))
}
fn car(input: &str) -> IResult<&str, Rule, ErrorTree<&str>> {
    let (input, _) = tag("\"")(input)?;
    // a or b
    let (input, character) = alpha1(input)?;
    let (input, _) = tag("\"")(input)?;
    Ok((input, Rule::Character(character)))
}

fn rule(input: &str) -> IResult<&str, (&str, Rule), ErrorTree<&str>> {
    let (input, id) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, output) = alt((or, nums_rule, car))(input)?;

    Ok((input, (id, output)))
}

fn ruleset(input: &str) -> IResult<&str, HashMap<&str, Rule>, ErrorTree<&str>> {
    let (input, rules) = separated_list1(newline, rule)(input)?;
    let rules = rules.into_iter().fold(HashMap::new(), |mut map, cur| {
        map.insert(cur.0, cur.1);
        map
    });
    Ok((input, rules))
}

#[derive(Debug)]
struct Problem<'a> {
    ruleset: HashMap<&'a str, Rule<'a>>,
    messages: &'a str,
}
fn problem(input: &str) -> IResult<&str, Problem, ErrorTree<&str>> {
    let (input, ruleset) = ruleset(input)?;
    let (messages, _) = multispace1(input)?;
    Ok(("", Problem { ruleset, messages }))
}

fn parse(input: &str) -> Result<Problem, ErrorTree<Location>> {
    final_parser(problem)(input)
}

fn make_thingy(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    tag("a")(input)
}
fn tag_thingy_a(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    tag("a")(input)
}
fn tag_thingy_b(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    tag("b")(input)
}
// fn rule_parser()
fn step<'a>(
    rule_map: &HashMap<&'a str, Rule>,
    rule: &'a Rule<'a>,
) -> impl Clone + Fn(&'a str) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    match rule {
        Rule::Character("a") => return |input: &str| tag("a")(input),
        Rule::Character("b") => return |input: &str| tag("b")(input),
        Rule::Nums(s) => {
            |input: &str| {
                // let parsers = s.split(" ").collect::<Vec<&str>>();

                // for parser in parsers {
                //     match rule_map.get(parser) {
                //         Some(v) => ,
                //     }
                // }
                // tag("a")
                return tag("a");
            } //: for<'r> fn(&'r str) -> std::result::Result<(&'r str, &'r str), nom::Err<ErrorTree<&'r str>>>
              // tag("a")
        }
        Rule::Alternate(s) => {
            s.split(" | ").collect::<Vec<&str>>();
            // alt(step())
            return |input: &str| tag("a")(input);
        }
        _ => panic!("asfklj"),
    }
}
pub fn process_part1(input: &str) -> usize {
    match parse(input) {
        Ok(problem) => {
            // let mut parser = tag("#")
            let rule_0 = problem.ruleset.get("3").unwrap();
            let (input, result) = step(&problem.ruleset, rule_0)("ab").unwrap();
            dbg!(input, result);
            //1 //2

            todo!()
        }
        Err(v) => {
            println!("{}", v);
            unimplemented!()
        }
    }
}

pub fn process_part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input_process_one() {
        assert_eq!(
            process_part1(
                "\
0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"

ababa
abb
abba"
            ),
            71
        )
    }
}
