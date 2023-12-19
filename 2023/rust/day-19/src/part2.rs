use std::{
    cmp::Ordering, collections::HashMap,
    ops::RangeInclusive,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, alpha1, line_ending, multispace1,
    },
    multi::separated_list1,
    sequence::delimited,
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
        value: u64,
        target: Target<'a>,
    },
    Target(Target<'a>),
}

enum Apply<'a> {
    Split {
        pass: (Part, &'a Target<'a>),
        fails: Part,
    },
    PassedTest(&'a Target<'a>),
    FailedTest,
}
impl<'a> Rule<'a> {
    fn apply_to(&self, part: &Part) -> Apply {
        match self {
            Rule::Test {
                part_field,
                condition,
                value,
                target,
            } => {
                let test_value = match *part_field {
                    "x" => &part.x,
                    "m" => &part.m,
                    "a" => &part.a,
                    "s" => &part.s,
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
                // 0..10
                // v = 5
                //
                // a..b < v
                // a..v
                // v..b
                // a..b > v
                // a..=v
                // (v+1)..b
                if test_value.contains(value) {
                    // split range
                    // Apply::Split((part, target), part);
                    match condition {
                        Condition::LessThan => {
                            let new_range_low = *test_value
                                .start()
                                ..=(value - 1);
                            let mut part_low = part.clone();
                            part_low.set_field(
                                part_field,
                                new_range_low,
                            );

                            let new_range_high =
                                *value..=*test_value.end();
                            let mut part_high =
                                part.clone();
                            part_high.set_field(
                                part_field,
                                new_range_high,
                            );
                            Apply::Split {
                                pass: (part_low, target),
                                fails: part_high,
                            }
                        }
                        Condition::GreaterThan => {
                            let new_range_low = *test_value
                                .start()
                                ..=*value;
                            let mut part_low = part.clone();
                            part_low.set_field(
                                part_field,
                                new_range_low,
                            );

                            let new_range_high = (*value
                                + 1)
                                ..=*test_value.end();
                            let mut part_high =
                                part.clone();
                            part_high.set_field(
                                part_field,
                                new_range_high,
                            );
                            Apply::Split {
                                pass: (part_high, target),
                                fails: part_low,
                            }
                        }
                    }
                } else {
                    // only do one target
                    if (test_value.end() < value
                        && cond == Ordering::Less)
                        || (test_value.start() > value
                            && cond == Ordering::Greater)
                    {
                        // entire range satisfies test
                        // return part/target
                        Apply::PassedTest(target)
                    } else {
                        // satisfies no test
                        Apply::FailedTest
                    }
                }
            }
            Rule::Target(target) => {
                Apply::PassedTest(target)
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Workflow<'a> {
    id: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug, Clone)]
struct Part {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
}
impl Part {
    fn set_field(
        &mut self,
        field: &str,
        new_range: RangeInclusive<u64>,
    ) {
        match field {
            "x" => {
                self.x = new_range;
            }
            "m" => {
                self.m = new_range;
            }
            "a" => {
                self.a = new_range;
            }
            "s" => {
                self.s = new_range;
            }
            _ => {
                unreachable!("no letters that aren't xmas");
            }
        };
    }
}
impl Default for Part {
    fn default() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }
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
    let (input, value) = complete::u64(input)?;
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

fn parse(
    input: &str,
) -> IResult<&str, HashMap<&str, Workflow>> {
    let (input, workflows) = workflows(input)?;
    let (input, _) = multispace1(input)?;
    Ok((input, workflows))
}
fn process_part(
    part: Part,
    workflows: &HashMap<&str, Workflow>,
    next_target: &Target,
) -> u64 {
    match next_target {
        Target::Workflow(id) => {
            let current_workflow = workflows.get(id).expect("should always produce a valid workflow given a valid id");
            let mut current_part = part;
            let mut sum = 0;
            for rule in current_workflow.rules.iter() {
                match rule.apply_to(&current_part) {
                    Apply::Split { pass, fails } => {
                        sum += process_part(
                            pass.0, workflows, pass.1,
                        );
                        current_part = fails;
                    }
                    Apply::PassedTest(target) => {
                        sum += process_part(
                            current_part.clone(),
                            workflows,
                            target,
                        );
                        break;
                    }
                    Apply::FailedTest => {}
                }
            }
            sum
        }
        Target::Accepted => {
            (part.x.end() - part.x.start() + 1)
                * (part.m.end() - part.m.start() + 1)
                * (part.a.end() - part.a.start() + 1)
                * (part.s.end() - part.s.start() + 1)
        }
        Target::Rejected => 0,
    }
}
#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_input, workflows) =
        parse(input).expect("should parse");

    let result = process_part(
        Part::default(),
        &workflows,
        &Target::Workflow("in"),
    );
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
        assert_eq!("167409079868000", process(input)?);
        Ok(())
    }
}
