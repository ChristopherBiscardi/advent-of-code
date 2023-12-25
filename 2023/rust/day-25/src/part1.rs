use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use petgraph::{
    algo::{self, dominators, min_spanning_tree},
    data::FromElements,
    dot::{Config, Dot},
    prelude::*,
};
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use std::{collections::HashMap, fs::File, io::Write};

use crate::custom_error::AocError;

fn parse(
    input: &str,
) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            alpha1,
            tag(": "),
            separated_list1(space1, alpha1),
        ),
    )(input)
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_, nodes) = parse(input).expect("should parse");

    let uniques = nodes
        .iter()
        .flat_map(|(key, values)| {
            let mut vs = values.clone();
            vs.push(key);
            vs
        })
        .unique()
        .collect::<Vec<&str>>();
    let mut graph = UnGraph::<&str, u32>::default();

    let node_map: HashMap<&str, NodeIndex> = uniques
        .iter()
        .map(|node| (*node, graph.add_node(&node)))
        .collect();

    for (key, values) in nodes.iter() {
        for node in values {
            graph.add_edge(
                node_map[key],
                node_map[node],
                1,
            );
        }
    }

    let min: rustworkx_core::Result<
        Option<(usize, Vec<_>)>,
    > = stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (cut_size, nodes_in_partition) =
        min.unwrap().unwrap();
    let total_nodes = uniques.len();

    // let dot_txt = format!(
    //     "{:?}",
    //     Dot::with_config(&graph, &[Config::EdgeNoLabel])
    // );
    // let mut file = File::create("graph.dot")?;
    // file.write_all(dot_txt.as_bytes())?;

    Ok(
        ((total_nodes - nodes_in_partition.len())
            * nodes_in_partition.len())
        .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        assert_eq!("54", process(input)?);
        Ok(())
    }
}
