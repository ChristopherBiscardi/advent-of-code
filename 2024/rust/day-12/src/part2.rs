use itertools::Itertools;
use petgraph::{
    algo::condensation, dot::Dot, prelude::*,
    visit::IntoNodeReferences,
};
use std::collections::HashMap;

// order of directions matters here,
// so that we can circular_tuple_windows without
// getting a north/south or east/west combination
const DIRECTIONS: [[i32; 2]; 4] =
    [[0, 1], [1, 0], [0, -1], [-1, 0]];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                ((x as i32, y as i32), c)
            })
        })
        .collect::<HashMap<(i32, i32), char>>();

    let mut gr: UnGraphMap<(i32, i32), ()> =
        UnGraphMap::new();

    for ((x, y), c) in map.iter() {
        let node = gr.add_node((*x, *y));

        for [x1, y1] in DIRECTIONS.iter() {
            let new_node = (x + x1, y + y1);
            if map.get(&new_node).is_some_and(|c2| c == c2)
            {
                gr.add_edge(node, new_node, ());
            };
        }
    }

    let new_graph = condensation(
        gr.clone().into_graph::<NodeIndex>(),
        false,
    );

    // println!(
    //     "{:?}",
    //     Dot::with_config(&new_graph, &[])
    // );

    let result = new_graph
        .node_references()
        .map(|(_node_index, node_list)| {
            let group_id = map.get(&node_list[0]).unwrap();
            let area = node_list.len();
            let perimeter = node_list
                .iter()
                .map(|n| {
                    calculate_corner_count(
                        n, &map, group_id,
                    )
                })
                .sum::<usize>();
            area * perimeter
        })
        .sum::<usize>();

    Ok(result.to_string())
}

fn calculate_corner_count(
    n: &(i32, i32),
    map: &HashMap<(i32, i32), char>,
    group_id: &char,
) -> usize {
    let mut count = 0;
    for ([x, y], [x1, y1]) in
        DIRECTIONS.iter().circular_tuple_windows()
    {
        let test_a = map
            .get(&(x + n.0, y + n.1))
            .is_some_and(|c| c == group_id);
        let test_b = map
            .get(&(x1 + n.0, y1 + n.1))
            .is_some_and(|c| c == group_id);
        if test_a
            && test_b
            && map
                .get(&(x + x1 + n.0, y + y1 + n.1))
                .is_some_and(|c| c != group_id)
        {
            // have interior corner
            count += 1;
        } else if !test_a && !test_b {
            //have exterior corner
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1206", process(input)?);
        Ok(())
    }
}
