use ndarray::{concatenate, Array2, Axis};
use nom::{
    bytes::complete::tag,
    character::complete::{
        self, alpha1, anychar, newline, one_of, u32,
    },
    multi::{many1, separated_list1},
    sequence::{pair, separated_pair},
    IResult,
};
use petgraph::{
    algo::dijkstra, graphmap::GraphMap, Undirected,
};

fn row(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, chars) =
        many1(one_of("0123456789"))(input)?;
    let nums = chars
        .iter()
        .map(|v| {
            v.to_digit(10).expect("to have succeeded") as u8
        })
        .collect::<Vec<u8>>();

    Ok((input, nums))
}
fn puzzle_input(input: &str) -> IResult<&str, Array2<u8>> {
    let (input, outputs) =
        separated_list1(newline, row)(input)?;
    let nrows = outputs.len();
    let ncols = outputs[0].len();

    let data =
        outputs.into_iter().flatten().collect::<Vec<u8>>();

    let arr = Array2::from_shape_vec((nrows, ncols), data)
        .unwrap();
    Ok((input, arr))
}

#[derive(
    Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Copy,
)]
struct Node {
    point: (usize, usize),
    weight: u8,
}
fn insert(
    graph: &mut GraphMap<Node, u32, Undirected>,
    heightmap: &Array2<u8>,
    point: (usize, usize),
    node: &Node,
) -> () {
    let top = heightmap.get(point);
    if let Some(weight) = top {
        let node_top = Node {
            point: point,
            weight: *weight,
        };
        graph.add_node(node_top);
        graph.add_edge(
            node.clone(),
            node_top,
            *weight as u32,
        );
    };
}
pub fn process_part1(input: &str) -> u32 {
    let (_, map) = puzzle_input(input).unwrap();

    let mut graph: GraphMap<Node, u32, Undirected> =
        GraphMap::new();
    for (point, value) in map.indexed_iter() {
        let node = Node {
            point: point,
            weight: *value,
        };
        graph.add_node(node);
        insert(
            &mut graph,
            &map,
            (point.0, point.1 + 1),
            &node,
        );
        // insert(
        //     &mut graph,
        //     &map,
        //     (point.0 + 1, point.1),
        //     &node,
        // );
        if point.0 != 0 {
            insert(
                &mut graph,
                &map,
                (point.0 - 1, point.1),
                &node,
            );
        };
        // if point.1 != 0 {
        //     insert(
        //         &mut graph,
        //         &map,
        //         (point.0, point.1 - 1),
        //         &node,
        //     );
        // };
    }
    // dbg!(graph);
    let mut it = map.indexed_iter();
    let next_it = it.next().unwrap();
    let start = Node {
        point: next_it.0,
        weight: *next_it.1,
    };
    let last_it = it.last().unwrap();
    let end = Node {
        point: last_it.0,
        weight: *last_it.1,
    };
    let result =
        dijkstra(&graph, start, Some(end), |edge| {
            edge.1.weight as u32
        });

    *result.get(&end).unwrap()
}

fn puzzle_input2(input: &str) -> IResult<&str, Array2<u8>> {
    let (input, outputs) =
        separated_list1(newline, row)(input)?;
    let nrows = outputs.len();
    let ncols = outputs[0].len();

    // let mut data = vec![];
    let original_tile =
        outputs.into_iter().flatten().collect::<Vec<u8>>();

    let arr = Array2::from_shape_vec(
        (nrows, ncols),
        original_tile,
    )
    .unwrap();

    let row_1_arrs = (0..5_u8)
        .map(|i| {
            arr.mapv(|weight| {
                let m = (weight + i) % 10;

                let res = match m {
                    0 => 1,
                    n => n,
                };
                if weight + i > 10 {
                    res + 1
                } else {
                    res
                }
            })
        })
        .collect::<Vec<_>>();
    let row_1_views: Vec<_> =
        row_1_arrs.iter().map(|v| v.view()).collect();
    let row_1 =
        concatenate(Axis(1), &row_1_views[..]).unwrap();

    // cols

    let col_arrs = (0..5_u8)
        .map(|i| {
            row_1.mapv(|weight| {
                let m = (weight + i) % 10;

                let res = match m {
                    0 => 1,
                    n => n,
                };
                if weight + i > 10 {
                    res + 1
                } else {
                    res
                }
            })
        })
        .collect::<Vec<_>>();
    let col_views: Vec<_> =
        col_arrs.iter().map(|v| v.view()).collect();
    let full_grid = concatenate(Axis(0), &col_views[..]);
    // dbg!(&full_grid);
    Ok((input, full_grid.unwrap()))
}
pub fn process_part2(input: &str) -> u32 {
    let (_, map) = puzzle_input2(input).unwrap();

    let mut graph: GraphMap<Node, u32, Undirected> =
        GraphMap::new();
    for (point, value) in map.indexed_iter() {
        let node = Node {
            point: point,
            weight: *value,
        };
        graph.add_node(node);
        insert(
            &mut graph,
            &map,
            (point.0, point.1 + 1),
            &node,
        );
        // insert(
        //     &mut graph,
        //     &map,
        //     (point.0 + 1, point.1),
        //     &node,
        // );
        if point.0 != 0 {
            insert(
                &mut graph,
                &map,
                (point.0 - 1, point.1),
                &node,
            );
        };
        // if point.1 != 0 {
        //     insert(
        //         &mut graph,
        //         &map,
        //         (point.0, point.1 - 1),
        //         &node,
        //     );
        // };
    }
    // dbg!(graph);
    let mut it = map.indexed_iter();
    let next_it = it.next().unwrap();
    let start = Node {
        point: next_it.0,
        weight: *next_it.1,
    };
    let last_it = it.last().unwrap();
    let end = Node {
        point: last_it.0,
        weight: *last_it.1,
    };
    let result =
        dijkstra(&graph, start, Some(end), |edge| {
            edge.1.weight as u32
        });

    *result.get(&end).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        include_str!("./test-input.txt");

    #[test]
    fn part1_test_demo_data() {
        assert_eq!(40, process_part1(INPUT));
    }
    #[test]
    fn part2_test_demo_data() {
        assert_eq!(315, process_part2(INPUT));
    }
}
