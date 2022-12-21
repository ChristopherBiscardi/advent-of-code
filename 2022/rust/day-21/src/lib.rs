use std::{collections::BTreeMap, fs::File, io::Write};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    character::complete::{alpha1, line_ending, one_of},
    combinator::{eof, iterator},
    multi::separated_list1,
    sequence::{delimited, terminated},
    *,
};
use petgraph::{
    dot::{Config, Dot},
    prelude::DiGraphMap,
    visit::{Topo, Visitable, Walker},
};
use tracing::*;

// root: pppw + sjmn
// dbpl: 5

#[derive(Debug)]
enum Operation<'a> {
    Number(i64),
    Calculate {
        left: &'a str,
        operator: Math,
        right: &'a str,
    },
}
#[derive(Debug)]
struct Node<'a> {
    id: &'a str,
    operation: Operation<'a>,
}

#[derive(Debug)]
enum Math {
    Multiply,
    Add,
    Subtract,
    Divide,
}

#[instrument]
fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, left) = alpha1(input)?;
    let (input, operator) = delimited(
        tag(" "),
        one_of("*+-/").map(|v| match v {
            '*' => Math::Multiply,
            '+' => Math::Add,
            '-' => Math::Subtract,
            '/' => Math::Divide,
            _ => panic!("unknown math operator"),
        }),
        tag(" "),
    )(input)?;
    let (input, right) = alpha1(input)?;
    Ok((
        input,
        Operation::Calculate {
            left,
            operator,
            right,
        },
    ))
}
#[instrument]
fn node(input: &str) -> IResult<&str, Node> {
    let (input, id) = alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, operation) = alt((
        complete::i64.map(|num| Operation::Number(num)),
        operation,
    ))(input)?;
    Ok((input, Node { id, operation }))
}

#[instrument(skip(input))]
fn nodes(
    input: &str,
) -> IResult<
    &str,
    (
        BTreeMap<&str, Node>,
        DiGraphMap<&str, ()>,
    ),
> {
    let (input, nodes) =
        separated_list1(line_ending, node)(input)?;
    let edges = nodes.iter().flat_map(|node| {
        match &node.operation {
            Operation::Number(_num) => vec![],
            Operation::Calculate {
                left,
                operator: _,
                right,
            } => vec![(*left, node.id), (*right, node.id)],
        }
    });
    let graph = DiGraphMap::<&str, ()>::from_edges(edges);

    // let dot =
    //     Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    // println!(
    //     "{:?}",
    //     Dot::with_config(&graph, &[Config::EdgeNoLabel])
    // );
    // let mut file = File::create("graph.dot").unwrap();
    // file.write_all(format!("{:?}", dot).as_bytes())
    //     .unwrap();
    let nodes = nodes
        .into_iter()
        .map(|node| (node.id, node))
        .collect();
    Ok((input, (nodes, graph)))
}

#[instrument(skip(input))]
pub fn process_part1(input: &str) -> String {
    let (_, (btree, graph)) = nodes(input).unwrap();
    let topological = Topo::new(&graph);
    let mut cache: BTreeMap<&str, i64> = BTreeMap::new();
    for node_id in topological.iter(&graph) {
        match &btree.get(node_id).unwrap().operation {
            Operation::Number(num) => {
                cache.insert(node_id, *num);
            }
            Operation::Calculate {
                left,
                operator,
                right,
            } => {
                let left_value = cache.get(left).unwrap();
                let right_value = cache.get(right).unwrap();

                match operator {
                    Math::Multiply => {
                        cache.insert(
                            node_id,
                            left_value * right_value,
                        );
                    }
                    Math::Add => {
                        cache.insert(
                            node_id,
                            left_value + right_value,
                        );
                    }
                    Math::Subtract => {
                        cache.insert(
                            node_id,
                            left_value - right_value,
                        );
                    }
                    Math::Divide => {
                        cache.insert(
                            node_id,
                            left_value / right_value,
                        );
                    }
                }
            }
        }
    }
    cache.get("root").unwrap().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, (btree, graph)) = nodes(input).unwrap();
    let topological = Topo::new(&graph);
    let mut cache: BTreeMap<&str, i64> = BTreeMap::new();

    let mut second_graph = DiGraphMap::<&str, ()>::new();

    for node_id in topological.iter(&graph) {
        match &btree.get(node_id).unwrap().operation {
            Operation::Number(num) => {
                if node_id != "humn" {
                    cache.insert(node_id, *num);
                }
            }
            Operation::Calculate {
                left,
                operator,
                right,
            } => {
                let left_value = cache.get(left);
                let right_value = cache.get(right);
                if node_id == "root" {
                    match (left_value, right_value) {
                        (None, None) => {
                            panic!("eek2");
                        }
                        (None, Some(r)) => {
                            cache.insert(left, *r);
                            continue;
                        }
                        (Some(l), None) => {
                            cache.insert(right, *l);
                            continue;
                        }
                        (Some(_), Some(_)) => panic!("eek"),
                    }
                }
                match (left_value, right_value) {
                    (
                        Some(left_value),
                        Some(right_value),
                    ) => match operator {
                        Math::Multiply => {
                            cache.insert(
                                node_id,
                                left_value * right_value,
                            );
                        }
                        Math::Add => {
                            cache.insert(
                                node_id,
                                left_value + right_value,
                            );
                        }
                        Math::Subtract => {
                            cache.insert(
                                node_id,
                                left_value - right_value,
                            );
                        }
                        Math::Divide => {
                            cache.insert(
                                node_id,
                                left_value / right_value,
                            );
                        }
                    },
                    (Some(_), None) => {
                        // dbg!("a");
                        second_graph.add_edge(
                            node_id,
                            right,
                            (),
                        );
                        second_graph.add_edge(
                            left,
                            right,
                            (),
                        );
                    }
                    (None, Some(_)) => {
                        // dbg!("b");
                        second_graph.add_edge(
                            node_id,
                            left,
                            (),
                        );
                        second_graph.add_edge(
                            right,
                            left,
                            (),
                        );
                    }
                    (None, None) => {
                        panic!("NoneNone");
                    }
                };
            }
        }
    }

    // let dot = Dot::with_config(
    //     &second_graph,
    //     &[Config::EdgeNoLabel],
    // );
    // // println!(
    // //     "{:?}",
    // //     Dot::with_config(&graph, &[Config::EdgeNoLabel])
    // // );
    // let mut file = File::create("graph2.dot").unwrap();
    // file.write_all(format!("{:?}", dot).as_bytes())
    //     .unwrap();

    // dbg!(cache.get("root"));
    // dbg!(&second_graph);
    let topological = Topo::new(&second_graph);
    for node_id in topological.iter(&second_graph) {
        // dbg!(node_id);
        match &btree.get(node_id).unwrap().operation {
            Operation::Number(_num) => {
                // if node_id != "humn" {
                //     dbg!(cache.get(node_id));
                //     // cache.insert(node_id, *num);
                // } else {
                //     // dbg!("calc human", node_id);
                // }
            }
            Operation::Calculate {
                left,
                operator,
                right,
            } => {
                let root_value =
                    cache.get(node_id).unwrap();
                let left_value = cache.get(left);
                let right_value = cache.get(right);

                match operator {
                    Math::Multiply => {
                        match (left_value, right_value) {
                            (None, Some(r)) => {
                                cache.insert(
                                    left,
                                    root_value / r,
                                );
                            }
                            (Some(l), None) => {
                                cache.insert(
                                    right,
                                    root_value / l,
                                );
                            }
                            (None, None) => panic!("eek2"),
                            (Some(_), Some(_)) => {
                                // panic!("eek")
                            }
                        }
                    }
                    Math::Add => {
                        match (left_value, right_value) {
                            (None, Some(r)) => {
                                cache.insert(
                                    left,
                                    root_value - r,
                                );
                            }
                            (Some(l), None) => {
                                cache.insert(
                                    right,
                                    root_value - l,
                                );
                            }
                            (None, None) => panic!("eek2"),
                            (Some(_), Some(_)) => {}
                        }
                    }
                    Math::Subtract => {
                        // 5 = x - 3; ; x=8; node_id + right_value;
                        // 5 = 3 - x; ; x=-2; * -1; (-1*node_id) - (-1*left_value);
                        match (left_value, right_value) {
                            (None, Some(r)) => {
                                cache.insert(
                                    left,
                                    root_value + r,
                                );
                            }
                            (Some(l), None) => {
                                cache.insert(
                                    right,
                                    (-1 * root_value)
                                        - (-1 * l),
                                );
                            }
                            (None, None) => panic!("eek2"),
                            (Some(_), Some(_)) => {
                                // panic!("eek")
                            }
                        }
                    }
                    Math::Divide => {
                        // root = left / right;
                        // 10 = 100 / right
                        // 10 = left / 100
                        match (left_value, right_value) {
                            (None, Some(r)) => {
                                cache.insert(
                                    left,
                                    root_value * r,
                                );
                            }
                            (Some(l), None) => {
                                cache.insert(
                                    right,
                                    l / root_value,
                                );
                            }
                            (None, None) => panic!("eek2"),
                            (Some(_), Some(_)) => {
                                // panic!("eek")
                            }
                        }
                    }
                }
            }
        }
    }
    // dbg!(second_graph);
    cache.get("humn").unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    #[ignore]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part1(INPUT), "152");
    }

    #[test]
    fn part2_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part2(INPUT), "301");
    }
}
