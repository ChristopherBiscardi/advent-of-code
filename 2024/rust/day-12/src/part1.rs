use petgraph::{
    algo::condensation, dot::Dot, prelude::*,
    visit::IntoNodeReferences,
};
use std::collections::HashMap;

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
            let area = node_list.len();
            let perimeter = node_list
                .iter()
                .map(|n| 4 - gr.neighbors(*n).count())
                .sum::<usize>();
            area * perimeter
        })
        .sum::<usize>();

    Ok(result.to_string())
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
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
