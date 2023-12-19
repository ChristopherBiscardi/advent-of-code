use std::{cmp::Ordering, collections::HashMap};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, alpha1, line_ending, multispace1,
    },
    combinator::opt,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

use crate::custom_error::AocError;

// basically Ordering without Equal
#[derive(Debug, Eq, PartialEq)]
enum Condition {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Eq, PartialEq)]
enum Target<'a> {
    Workflow(&'a str),
    Accepted,
    Rejected,
}

#[derive(Debug, Eq, PartialEq)]
enum Rule<'a> {
    Test {
        part_field: &'a str,
        condition: Condition,
        value: u32,
        target: Target<'a>,
    },
    Target(Target<'a>),
}

impl<'a> Rule<'a> {
    fn apply_to(&self, part: &Part) -> Option<&Target> {
        match self {
            Rule::Test {
                part_field,
                condition,
                value,
                target,
            } => {
                let test_value = match *part_field {
                    "x" => part.x,
                    "m" => part.m,
                    "a" => part.a,
                    "s" => part.s,
                    _ => {
                        unreachable!(
                            "no letters that aren't xmas"
                        );
                    }
                };
                let cond = match condition {
                    Condition::LessThan => Ordering::Less,
                    Condition::GreaterThan => {
                        Ordering::Greater
                    }
                };
                (test_value.cmp(value) == cond)
                    .then_some(target)
            }
            Rule::Target(target) => Some(target),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Workflow<'a> {
    id: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug, Default)]
pub struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn target(input: &str) -> IResult<&str, Target> {
    alt((
        tag("A").map(|_| Target::Accepted),
        tag("R").map(|_| Target::Rejected),
        alpha1.map(|workflow_id| {
            Target::Workflow(workflow_id)
        }),
    ))(input)
}
fn rule_test(input: &str) -> IResult<&str, Rule> {
    let (input, part_field) = alpha1(input)?;
    let (input, condition) = alt((
        complete::char('>').map(|_| Condition::GreaterThan),
        complete::char('<').map(|_| Condition::LessThan),
    ))(input)?;
    let (input, value) = complete::u32(input)?;
    let (input, _) = complete::char(':')(input)?;
    let (input, target) = target(input)?;
    Ok((
        input,
        Rule::Test {
            part_field,
            condition,
            value,
            target,
        },
    ))
}

// px{a<2006:qkq,m>2090:A,rfg}
fn workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, id) = alpha1(input)?;
    let (input, rules) = delimited(
        complete::char('{'),
        separated_list1(
            complete::char(','),
            alt((rule_test, target.map(Rule::Target))),
        ),
        complete::char('}'),
    )(input)?;
    Ok((input, Workflow { id, rules }))
}

fn workflows(
    input: &str,
) -> IResult<&str, HashMap<&str, Workflow>> {
    let (input, workflows) =
        separated_list1(line_ending, workflow)(input)?;
    Ok((
        input,
        workflows.into_iter().map(|w| (w.id, w)).collect(),
    ))
}

fn part(input: &str) -> IResult<&str, Part> {
    delimited(
        complete::char('{'),
        fold_many1(
            terminated(
                separated_pair(
                    alpha1,
                    complete::char('='),
                    complete::u32,
                ),
                opt(tag(",")),
            ),
            Part::default,
            |mut part, (next_field, count)| {
                match next_field {
                    "x" => {
                        part.x = count;
                    }
                    "m" => {
                        part.m = count;
                    }
                    "a" => {
                        part.a = count;
                    }
                    "s" => {
                        part.s = count;
                    }
                    _ => unreachable!(
                        "no letters that aren't xmas"
                    ),
                }
                part
            },
        ),
        complete::char('}'),
    )(input)
}

// {x=787,m=2655,a=1222,s=2876}
fn parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(line_ending, part)(input)
}

pub fn parse(
    input: &str,
) -> IResult<&str, (Vec<Part>, HashMap<&str, Workflow>)> {
    let (input, workflows) = workflows(input)?;
    let (input, _) = multispace1(input)?;
    let (input, parts) = parts(input)?;
    Ok((input, (parts, workflows)))
}
#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_input, (parts, workflows)) =
        parse(input).expect("should parse");

    let result = parts
        .iter()
        .filter_map(|part| {
            let mut current_workflow = "in";
            let last_target: Target = 'workflow_loop: loop {
                let active_workflow =
                    workflows.get(current_workflow).expect(
                        "should only fetch valid workflows",
                    );
                'rule_loop: for rule in
                    active_workflow.rules.iter()
                {
                    match rule.apply_to(part) {
                        Some(Target::Accepted) => {
                            // break out of loop loop
                            break 'workflow_loop Target::Accepted;
                        }
                        Some(Target::Rejected) => {
                            // break out of loop loop
                            break 'workflow_loop Target::Rejected;
                        }
                        Some(Target::Workflow(
                            next_workflow_id,
                        )) => {
                            current_workflow =
                                next_workflow_id;
                            // break out of for loop
                            break 'rule_loop;
                        }
                        None => {}
                    }
                }
            };
            match last_target {
                Target::Workflow(_) => {
                    unreachable!("shouldn't end on a workflow")
                },
                Target::Accepted => Some(part.x + part.m + part.a + part.s),
                Target::Rejected => None,
            }
        })
        .sum::<u32>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_parser() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}";
        assert_eq!(
            Workflow {
                id: "px",
                rules: vec![
                    Rule::Test {
                        part_field: "a",
                        condition: Condition::LessThan,
                        value: 2006,
                        target: Target::Workflow("qkq")
                    },
                    Rule::Test {
                        part_field: "m",
                        condition: Condition::GreaterThan,
                        value: 2090,
                        target: Target::Accepted
                    },
                    Rule::Target(Target::Workflow("rfg"))
                ]
            },
            workflow(input)
                .expect("should parse a workflow")
                .1
        )
    }
    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!("19114", process(input)?);
        Ok(())
    }
}
