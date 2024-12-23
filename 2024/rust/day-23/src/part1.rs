use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use petgraph::prelude::UnGraphMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, edges) = parse(input).map_err(|e| {
        miette::miette!("parse failed {}", e)
    })?;

    let g = &UnGraphMap::<&str, ()>::from_edges(&edges);

    let output = g
        .nodes()
        .flat_map(|node| {
            // dbg!(node);
            g.neighbors(node)
                .tuple_combinations()
                .filter(move |(a, b)| {
                    g.contains_edge(a, b)
                        && [node, a, b]
                            .iter()
                            .any(|n| n.starts_with("t"))
                })
                .map(move |(a, b)| {
                    let mut nodes = [node, a, b];
                    nodes.sort();
                    nodes
                })
        })
        .unique()
        // .inspect(|value| {
        //     info!(?value);
        // })
        .count();

    // println!("{:?}", Dot::with_config(&g, &[]));

    // let graph = g.into_graph::<u32>();
    // // find all cycles
    // let cycles = graph.cycles();

    // let output = cycles
    //     .iter()
    //     .filter(|cycle| cycle.len() == 3)
    //     .map(|cycle| {
    //         cycle
    //             .iter()
    //             .map(|idx| graph.node_weight(*idx).unwrap())
    //             .sorted()
    //             .collect::<Vec<_>>()
    //     })
    //     .unique()
    //     .filter(|cycle| {
    //         cycle.iter().any(|node_weight| {
    //             node_weight.contains("t")
    //         })
    //     })
    //     .collect::<Vec<_>>();

    // graph.visit_all_cycles(
    //     |_g, indices: &[NodeIndex<u32>]| {
    //         if indices.len() == 3 {
    //             println!(
    //                 "Found new cycle with vertices {:?}",
    //                 indices
    //                     .iter()
    //                     .map(|idx| graph.node_weight(*idx))
    //                     .collect::<Vec<_>>()
    //             );
    //         }
    //     },
    // );
    Ok(output.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(
        line_ending,
        separated_pair(alpha1, tag("-"), alpha1),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
