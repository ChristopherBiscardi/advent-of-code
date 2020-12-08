use std::collections::HashMap;

use daggy::{Dag, Walker};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    combinator::opt,
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
struct Bag<'a> {
    name: &'a str,
    holds: Vec<(usize, &'a str)>,
}
// 1 bright white bag, 2 muted yellow bags.
fn held_bag(input: Span) -> IResult<Span, (usize, &str)> {
    let (input, num) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    //todo
    let (input, bag_name) = take_until(" bag")(input)?;
    let (input, _) = alt((tag(" bags"), tag(" bag")))(input)?;
    let (input, _) = alt((tag(","), tag(".")))(input)?;
    let n = num.parse::<usize>();
    match n {
        Ok(i) => Ok((input, (i, &bag_name))),
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Digit,
        })),
    }
}
fn no_bags(input: Span) -> IResult<Span, Vec<(usize, &str)>> {
    let (input, _) = tag("no other bags.")(input)?;
    Ok((input, vec![]))
}
fn bag_line(input: Span) -> IResult<Span, Bag> {
    let (input, key) = take_until(" bags contain ")(input)?;
    let (input, _) = tag(" bags contain ")(input)?;
    // let (input, pos) = position(input)?;
    // dbg!(pos);
    let (input, bags) = alt((no_bags, separated_list1(tag(" "), held_bag)))(input)?;

    Ok((
        input,
        Bag {
            name: &key,
            holds: bags,
        },
    ))
}

fn bags(input: Span) -> IResult<Span, Vec<Bag>> {
    let (input, vs) = separated_list1(char('\n'), bag_line)(input)?;
    Ok((input, vs))
}
pub fn process_part1(input: &str) -> usize {
    let bags = opt(bags)(Span::new(input))
        .ok()
        .and_then(|(_, res)| res)
        .unwrap();

    let (mut dag, mut map) = bags.iter().enumerate().fold(
        {
            let d: Dag<usize, usize, usize> = Dag::new();
            let m: HashMap<&str, usize> = HashMap::new();
            (d, m)
        },
        |(mut dag, mut map), (i, bag)| {
            dag.add_node(i);
            map.insert(bag.name, i);
            (dag, map)
        },
    );

    for (i, bag) in bags.iter().enumerate() {
        for (num, name) in bag.holds.iter() {
            dag.add_edge(
                i.into(),
                daggy::NodeIndex::new(*map.get(name).unwrap()),
                *num,
            )
            .expect("failed to add_edge");
        }
    }
    let gold_index = daggy::NodeIndex::new(*map.get("shiny gold").unwrap());
    // dbg!(gold_index);

    let mut visited: Vec<daggy::NodeIndex<usize>> = vec![];
    let mut parents: Vec<_> = dag.parents(gold_index).iter(&dag).collect();

    loop {
        if parents.len() == 0 {
            break;
        };
        let mut new_parents = vec![];
        for parent in parents.iter() {
            for grandparent in dag.parents(parent.1).iter(&dag) {
                new_parents.push(grandparent)
            }
            visited.push(parent.1)
        }
        parents = new_parents;
    }

    visited.iter().unique().count()
}

pub fn process_part2(input: &str) -> usize {
    let bags = input
        .lines()
        .flat_map(|v| opt(bag_line)(Span::new(v)).ok().and_then(|(span, res)| res))
        .collect::<Vec<Bag>>();

    let (mut dag, mut map) = bags.iter().enumerate().fold(
        {
            let d: Dag<usize, usize, usize> = Dag::new();
            let m: HashMap<&str, usize> = HashMap::new();
            (d, m)
        },
        |(mut dag, mut map), (i, bag)| {
            dag.add_node(i);
            map.insert(bag.name, i);
            (dag, map)
        },
    );

    for (i, bag) in bags.iter().enumerate() {
        for (num, name) in bag.holds.iter() {
            dag.add_edge(
                i.into(),
                daggy::NodeIndex::new(*map.get(name).unwrap()),
                *num,
            )
            .expect("failed to add_edge");
        }
    }
    let gold_index = daggy::NodeIndex::new(*map.get("shiny gold").unwrap());

    let mut visited: usize = 0;
    let mut children: Vec<_> = dag
        .children(gold_index)
        .iter(&dag)
        .map(|v| (1, v))
        .collect();

    loop {
        if children.len() == 0 {
            break visited;
        };
        let mut new_children = vec![];
        for (num_instances_of_gold_bag, (edge, child)) in children.iter() {
            let edge_weight = dag.edge_weight(*edge).expect("expected an edge");
            visited = visited + (num_instances_of_gold_bag * edge_weight);
            for grandchild in dag.children(*child).iter(&dag) {
                new_children.push((*edge_weight * num_instances_of_gold_bag, grandchild))
            }
        }
        children = new_children;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_held_bag() {
        assert_eq!(
            alt((no_bags, many1(held_bag)))(Span::new("1 bright white bag,"))
                .unwrap()
                .1,
            vec![(1, "bright white")]
        );
    }
    #[test]
    fn test_held_bag_2() {
        assert_eq!(
            alt((no_bags, many1(held_bag)))(Span::new("2 muted yellow bags."))
                .unwrap()
                .1,
            vec![(2, "muted yellow")]
        );
    }

    #[test]
    fn test_input_process_one() {
        assert_eq!(
            process_part1(
                "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            ),
            4
        );
    }
    #[test]
    fn test_input_process_two_example_1() {
        assert_eq!(
            process_part2(
                "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            ),
            32
        );
    }
    #[test]
    fn test_input_process_two_example_2() {
        assert_eq!(
            process_part2(
                "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            ),
            126
        );
    }
}
