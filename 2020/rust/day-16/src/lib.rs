use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{take_until, take_while},
    character::{
        complete::{alpha1, char, digit1, newline},
        is_alphabetic, is_space,
    },
    combinator::opt,
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

// type Span<'a> = LocatedSpan<&'a str>;

fn range(input: &str) -> IResult<&str, std::ops::RangeInclusive<usize>, ErrorTree<&str>> {
    let (input, num1) = digit1(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, num2) = digit1(input)?;

    let num1_parsed = num1.parse::<usize>().unwrap();
    let num2_parsed = num2.parse::<usize>().unwrap();

    Ok((input, num1_parsed..=num2_parsed))
}
struct Rule<'a> {
    name: &'a str,
    ranges: Vec<std::ops::RangeInclusive<usize>>,
}
fn is_id(c: char) -> bool {
    c.is_ascii_alphabetic() || c.is_ascii_whitespace()
}
fn rule(input: &str) -> IResult<&str, Rule, ErrorTree<&str>> {
    let (input, id) = take_while(is_id)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, range1) = range(input)?;
    let (input, _) = tag(" or ")(input)?;
    let (input, range2) = range(input)?;

    Ok((
        input,
        Rule {
            name: id,
            ranges: vec![range1, range2],
        },
    ))
}

fn ticket(input: &str) -> IResult<&str, Vec<usize>, ErrorTree<&str>> {
    let (input, digits) = separated_list1(tag(","), digit1)(input)?;
    let digits = digits
        .iter()
        .map(|d| d.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    Ok((input, digits))
}
struct Note<'a> {
    rules: Vec<Rule<'a>>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}
fn notes(input: &str) -> IResult<&str, Note, ErrorTree<&str>> {
    let (input, rules) = separated_list1(newline, rule)(input)?;
    let (input, _) = tag("

your ticket:
")(input)?;
    let (input, my_ticket) = ticket(input)?;
    let (input, _) = tag("

nearby tickets:
")(input)?;
    let (input, nearby_tickets) = separated_list1(newline, ticket)(input)?;
    Ok((
        input,
        Note {
            rules,
            my_ticket,
            nearby_tickets,
        },
    ))
}

fn parse(input: &str) -> Result<Note, ErrorTree<Location>> {
    final_parser(notes)(input)
}

pub fn process_part1(input: &str) -> usize {
    let note = parse(input).unwrap();
    note.nearby_tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter(|num| {
                    // if num passes any rule, return None,
                    // Otherwise collect failed values with Some
                    !note
                        .rules
                        .iter()
                        .any(|rule| rule.ranges.iter().any(|range| range.contains(num)))
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

pub fn process_part2(input: &str) -> usize {
    let note = parse(input).unwrap();
    let valid_tickets = note.nearby_tickets.iter().filter(|ticket| {
        !ticket.iter().any(|num| {
            !note
                .rules
                .iter()
                .any(|rule| rule.ranges.iter().any(|range| range.contains(num)))
        })
    });

    let mut rule_state: HashMap<&str, HashMap<usize, bool>> = note
        .rules
        .iter()
        .map(|cur| (cur.name, HashMap::new()))
        .collect();

    for ticket in valid_tickets {
        for (i, num) in ticket.iter().enumerate() {
            for rule in note.rules.iter() {
                let rule_map = rule_state.get_mut(rule.name).unwrap();
                match &rule_map.get(&i) {
                    Some(buul) => {
                        if **buul {
                            rule_map.insert(i, rule.ranges.iter().any(|range| range.contains(num)));
                        }
                    }
                    None => {
                        rule_map.insert(i, rule.ranges.iter().any(|range| range.contains(num)));
                    }
                }
            }
        }
    }

    // name of rule, index it occupies
    let mut slots: HashMap<&str, usize> = HashMap::new();
    let valid_positions = rule_state
        .iter()
        // .filter(|(k, v)| k.starts_with("departure"))
        .map(|(k, map)| {
            (
                *k,
                map.iter()
                    .filter_map(|(k, v)| if *v { Some(k) } else { None })
                    .collect::<Vec<_>>(),
            )
        })
        // (name, valid_positions)
        .collect::<HashMap<&str, Vec<&usize>>>();

    loop {
        let sets: &Vec<(&&str, Vec<&&usize>)> = &valid_positions
            .iter()
            .filter_map(|(name, indicies)| {
                let new = indicies
                    .iter()
                    .filter(|index| match slots.iter().find(|(_, v)| v == *index) {
                        None => true,
                        Some(_) => false,
                    })
                    .collect::<Vec<&&usize>>();
                if new.len() == 1 {
                    Some((name, new))
                } else {
                    None
                }
            })
            .collect();
        if sets.len() == 0 {
            break;
        }
        for (rule_name, rule_indices) in sets.iter() {
            slots.insert(
                rule_name,
                **rule_indices[0], // rule_indices.iter().map(|(k, v)| k).collect::<Vec<usize>>()[0],
            );
        }
    }
    slots
        .iter()
        .filter_map(|(k, v)| {
            if k.starts_with("departure") {
                Some(note.my_ticket[*v])
            } else {
                None
            }
        })
        .product()
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
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            ),
            71
        )
    }
}
