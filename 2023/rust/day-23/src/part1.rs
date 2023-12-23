use crate::custom_error::AocError;
use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    combinator::opt,
    multi::many1,
    sequence::delimited,
    IResult, Parser,
};
use nom_locate::LocatedSpan;
use petgraph::{
    algo,
    data::FromElements,
    dot::{Config, Dot},
    prelude::*,
};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::File,
    io::Write,
};

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum PlotType {
    Directional(Direction),
    Empty,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn step(&self, position: &IVec2) -> IVec2 {
        *position
            + match self {
                Direction::North => IVec2::new(0, -1),
                Direction::South => IVec2::new(0, 1),
                Direction::East => IVec2::new(1, 0),
                Direction::West => IVec2::new(-1, 0),
            }
    }
}

#[derive(Debug)]
struct Plot<'a> {
    r#type: PlotType,
    span: SpanIVec2<'a>,
}

impl<'a> Display for Plot<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let c = match self.r#type {
            PlotType::Directional(Direction::North) => "^",
            PlotType::Directional(Direction::South) => "v",
            PlotType::Directional(Direction::East) => ">",
            PlotType::Directional(Direction::West) => "<",
            PlotType::Empty => ".",
        };
        write!(f, "{c}")
    }
}

fn with_xy(span: Span) -> SpanIVec2 {
    // column/location are 1-indexed
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}
fn parse_grid(input: Span) -> IResult<Span, Vec<Plot>> {
    let (input, grid) = many1(delimited(
        opt(is_a("#\n")),
        alt((
            tag("^").map(|span| {
                let located = with_xy(span);
                Plot {
                    r#type: PlotType::Directional(
                        Direction::North,
                    ),
                    span: located,
                }
            }),
            tag(">").map(|span| {
                let located = with_xy(span);
                Plot {
                    r#type: PlotType::Directional(
                        Direction::East,
                    ),
                    span: located,
                }
            }),
            tag("v").map(|span| {
                let located = with_xy(span);
                Plot {
                    r#type: PlotType::Directional(
                        Direction::South,
                    ),
                    span: located,
                }
            }),
            tag("<").map(|span| {
                let located = with_xy(span);
                Plot {
                    r#type: PlotType::Directional(
                        Direction::West,
                    ),
                    span: located,
                }
            }),
            tag(".").map(|span| {
                let located = with_xy(span);
                Plot {
                    r#type: PlotType::Empty,
                    span: located,
                }
            }),
        )),
        opt(is_a("#\n")),
    ))(input)?;
    Ok((input, grid))
}

#[allow(dead_code)]
fn print_grid(
    map: &HashMap<&IVec2, &Plot>,
    boundaries: &IVec2,
) {
    for y in 0..boundaries.y {
        for x in 0..boundaries.x {
            match map.get(&IVec2::new(x, y)) {
                Some(plot) => {
                    print!("{plot}");
                }
                None => print!("#"),
            }
        }
        println!();
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let row_count = input.lines().count() as i32;
    let column_count =
        input.lines().next().unwrap().len() as i32;
    let bounds = IVec2::new(column_count, row_count);
    let (_input, grid) = parse_grid(Span::new(input))
        .expect("should parse a grid");

    let grid_map = grid
        .iter()
        .map(|plot| (&plot.span.extra, plot))
        .collect::<HashMap<&IVec2, &Plot>>();

    // print_grid(&grid_map, &bounds);

    let start_position = grid
        .iter()
        .min_by_key(|plot| plot.span.extra.y)
        .unwrap();
    let end_position = grid
        .iter()
        .max_by_key(|plot| plot.span.extra.y)
        .unwrap();

    // grid_walk

    let mut graph = DiGraph::<&Plot, u32>::new();

    let node_map: HashMap<IVec2, NodeIndex> = grid
        .iter()
        .map(|plot| {
            (plot.span.extra, graph.add_node(&plot))
        })
        .collect();

    grid.iter()
        .flat_map(|plot| {
            let possible_directions = match plot.r#type {
                PlotType::Directional(direction) => {
                    vec![direction]
                }
                PlotType::Empty => vec![
                    Direction::North,
                    Direction::South,
                    Direction::East,
                    Direction::West,
                ],
            };
            possible_directions.into_iter().filter_map(
                |dir| {
                    let next_pos =
                        dir.step(&plot.span.extra);
                    let can_move_here =
                        grid_map.get(&next_pos).is_some();
                    can_move_here.then(|| {
                        (
                            node_map[&plot.span.extra],
                            node_map[&next_pos],
                            1,
                        )
                    })
                },
            )
        })
        .for_each(|(a, b, weight)| {
            graph.add_edge(a, b, weight);
        });
    // .collect::<Vec<(NodeIndex, NodeIndex, i32)>>();

    // graph.extend_with_edges(&edges);

    let ways = algo::all_simple_paths::<Vec<_>, _>(
        &graph,
        node_map[&start_position.span.extra],
        node_map[&end_position.span.extra],
        0,
        None,
    )
    .max_by(|a, b| a.len().cmp(&b.len()))
    .unwrap();
    // dbg!(ways.len());
    // .collect::<Vec<_>>();
    // dbg!(graph);
    // let dot_txt = format!(
    //     "{:?}",
    //     Dot::with_config(&graph, &[Config::EdgeNoLabel])
    // );
    // let mut file = File::create("graph.dot")?;
    // file.write_all(dot_txt.as_bytes())?;

    // step count is tiles_visited - 1
    Ok((ways.len() - 1).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!("94", process(input)?);
        Ok(())
    }
}
