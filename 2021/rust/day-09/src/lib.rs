use ndarray::Array2;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{
        alpha1, anychar, char, digit1, newline, u16,
    },
    multi::{many1, separated_list1},
    IResult,
};
use petgraph::{
    algo::{condensation, connected_components},
    dot::{Config, Dot},
};
use petgraph::{
    data::Build, graphmap::GraphMap,
    matrix_graph::MatrixGraph, Undirected,
};
use std::io::Write;
use std::{collections::HashSet, fs::File};

fn row(input: &str) -> IResult<&str, Vec<Option<u32>>> {
    let (input, chars) = many1(alt((
        char('0'),
        char('1'),
        char('2'),
        char('3'),
        char('4'),
        char('5'),
        char('6'),
        char('7'),
        char('8'),
        char('9'),
    )))(input)?;
    let nums: Vec<Option<u32>> = chars
        .iter()
        .map(|v| {
            Some(v.to_digit(10).expect("to have succeeded"))
        })
        .collect();
    let mut res = vec![None];
    res.extend(nums);
    res.extend([None]);

    Ok((input, res))
}
fn puzzle_input(
    input: &str,
) -> IResult<&str, Array2<Option<u32>>> {
    let (input, outputs) =
        separated_list1(newline, row)(input)?;
    // dbg!(&outputs);
    let nrows = outputs.len();
    let ncols = outputs[0].len();

    let mut data: Vec<Option<u32>> = vec![None; ncols];

    let real_data: Vec<Option<u32>> =
        outputs.into_iter().flatten().collect();
    data.extend(real_data);
    data.extend(vec![None; ncols]);

    // dbg!(ncols, nrows);
    let arr =
        Array2::from_shape_vec((nrows + 2, ncols), data)
            .unwrap();
    // dbg!(&arr);
    Ok((input, arr))
}

pub fn process_part1(input: &str) -> u32 {
    let (_, heightmap) = puzzle_input(input).unwrap();
    // println!(
    //     "{}",
    //     heightmap
    //         .rows()
    //         .into_iter()
    //         .map(|arr| {
    //             let mut res = arr
    //                 .iter()
    //                 .map(|v| match v {
    //                     None => "x".to_string(),
    //                     Some(v) => v.to_string(),
    //                 })
    //                 .collect::<String>();
    //             res.push('\n');
    //             res
    //         })
    //         .collect::<String>()
    // );
    let results: u32 = heightmap
        .windows((3, 3))
        .into_iter()
        .filter_map(|points| {
            // println!(
            //     "{}",
            //     points
            //         .rows()
            //         .into_iter()
            //         .map(|arr| {
            //             let mut res = arr
            //                 .iter()
            //                 .map(|v| match v {
            //                     None => "x".to_string(),
            //                     Some(v) => v.to_string(),
            //                 })
            //                 .collect::<String>();
            //             res.push('\n');
            //             res
            //         })
            //         .collect::<String>()
            // );
            let top = points[(0, 1)];
            let left = points[(1, 0)];
            let right = points[(1, 2)];
            let bottom = points[(2, 1)];
            let point = points[(1, 1)];
            match [top, left, right, bottom]
                .iter()
                .filter(|v| v.is_some())
                .all(|&v| v > point)
            {
                true => {
                    // dbg!(point);
                    point.map(|v| v + 1)
                }
                false => None,
            }
        })
        .sum();
    results
}

fn row_2(input: &str) -> IResult<&str, Vec<Option<u32>>> {
    let (input, chars) = many1(alt((
        char('0'),
        char('1'),
        char('2'),
        char('3'),
        char('4'),
        char('5'),
        char('6'),
        char('7'),
        char('8'),
        char('9'),
    )))(input)?;
    let nums: Vec<Option<u32>> = chars
        .iter()
        .map(|v| {
            match v.to_digit(10).expect("to have succeeded")
            {
                9 => None,
                i => Some(i),
            }
        })
        .collect();
    let mut res = vec![None];
    res.extend(nums);
    res.extend([None]);

    Ok((input, res))
}
fn puzzle_input_2(
    input: &str,
) -> IResult<&str, Array2<Option<u32>>> {
    let (input, outputs) =
        separated_list1(newline, row_2)(input)?;

    let nrows = outputs.len();
    let ncols = outputs[0].len();

    let mut data: Vec<Option<u32>> = vec![None; ncols];

    let real_data: Vec<Option<u32>> =
        outputs.into_iter().flatten().collect();
    data.extend(real_data);
    data.extend(vec![None; ncols]);

    let arr =
        Array2::from_shape_vec((nrows + 2, ncols), data)
            .unwrap();

    Ok((input, arr))
}

#[derive(
    Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Copy,
)]
struct Node {
    point: (usize, usize),
    weight: u32,
}
fn insert(
    graph: &mut GraphMap<Node, (), Undirected>,
    heightmap: &Array2<Option<u32>>,
    point: (usize, usize),
    node: &Node,
) -> () {
    let top = heightmap.get(point);
    if let Some(Some(weight)) = top {
        let node_top = Node {
            point: point,
            weight: *weight,
        };
        graph.add_node(node_top);
        graph.add_edge(node.clone(), node_top, ());
    };
}
pub fn process_part2(input: &str) -> usize {
    let (_, heightmap) = puzzle_input_2(input).unwrap();
    let mut graph: GraphMap<Node, (), Undirected> =
        GraphMap::new();
    for (point, maybe_value) in heightmap.indexed_iter() {
        // println!(
        //     "{}",
        //     points
        //         .rows()
        //         .into_iter()
        //         .map(|arr| {
        //             let mut res = arr
        //                 .iter()
        //                 .map(|v| match v {
        //                     None => "x".to_string(),
        //                     Some(v) => v.to_string(),
        //                 })
        //                 .collect::<String>();
        //             res.push('\n');
        //             res
        //         })
        //         .collect::<String>()
        // );

        if let Some(value) = maybe_value {
            let node = Node {
                point: point,
                weight: *value,
            };
            graph.add_node(node);
            insert(
                &mut graph,
                &heightmap,
                (point.0, point.1 + 1),
                &node,
            );
            insert(
                &mut graph,
                &heightmap,
                (point.0 + 1, point.1),
                &node,
            );
            insert(
                &mut graph,
                &heightmap,
                (point.0 - 1, point.1),
                &node,
            );
            insert(
                &mut graph,
                &heightmap,
                (point.0, point.1 - 1),
                &node,
            );
        }
    }
    // dbg!(connected_components(&graph));
    // let dot =
    //     Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    // let mut file = File::create("dot.txt").unwrap();
    // file.write_all(format!("{:?}", dot).as_bytes())
    //     .unwrap();
    let condensed_graph =
        condensation::<Node, (), Undirected, u32>(
            graph.into_graph(),
            false,
        );
    let mut sums = condensed_graph
        .node_weights()
        .map(|basin| basin.len())
        .collect::<Vec<usize>>();
    sums.sort();
    sums.reverse();
    let mut finals = sums.iter();
    finals.next().unwrap()
        * finals.next().unwrap()
        * finals.next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &'static str =
        include_str!("./test-input.txt");

    // #[test]
    // fn part1_test_demo_data() {
    //     assert_eq!(15, process_part1(input));
    // }

    #[test]
    fn part2_test_demo_data() {
        assert_eq!(1134, process_part2(input));
    }
}
