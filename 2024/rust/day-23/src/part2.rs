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
            g.neighbors(node).combinations(12).filter_map(
                move |neighbor_subset| {
                    if neighbor_subset
                        .iter()
                        .tuple_combinations()
                        .all(move |(a, b)| {
                            g.contains_edge(a, b)
                        })
                    {
                        let mut nodes = vec![node]
                            .into_iter()
                            .chain(
                                neighbor_subset.into_iter(),
                            )
                            .collect::<Vec<_>>();
                        nodes.sort();
                        Some(nodes)
                    } else {
                        None
                    }
                },
            )
        })
        .unique()
        .collect::<Vec<_>>();

    if output.len() == 1 {
        return Ok(output[0].join(","));
    } else {
        dbg!(output);
        panic!("0 or many answers")
    }
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
