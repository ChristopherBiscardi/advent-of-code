use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    *,
};
use std::{
    cmp::Ordering::{self, *},
    fmt::Display,
};

#[derive(Debug, PartialEq)]
pub struct Pair {
    left: Packet,
    right: Packet,
}
#[derive(Debug, Eq)]
pub enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl Display for Packet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Packet::List(list) => format!(
                    "[{}]",
                    list.iter()
                        .map(|v| v.to_string())
                        .intersperse(",".to_string())
                        .collect::<String>()
                ),
                Packet::Number(num) => num.to_string(),
            }
        )
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => {
                l0 == r0
            }
            (Self::List(l0), Self::Number(r0)) => {
                l0 == &vec![Packet::Number(*r0)]
            }
            (Self::Number(l0), Self::List(r0)) => {
                &vec![Packet::Number(*l0)] == r0
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(a), Packet::Number(b)) => {
                a.cmp(&vec![Packet::Number(*b)])
            }
            (Packet::Number(a), Packet::List(b)) => {
                vec![Packet::Number(*a)].cmp(&b)
            }
            (Packet::Number(a), Packet::Number(b)) => {
                a.cmp(b)
            }
        }
    }
}

pub fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(
            tag("["),
            separated_list0(tag(","), packet),
            tag("]"),
        )
        .map(|vec| Packet::List(vec)),
        nom::character::complete::u32
            .map(|num| Packet::Number(num)),
    ))(input)
}
pub fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, newline, packet).map(
            |(p1, p2)| Pair {
                left: p1,
                right: p2,
            },
        ),
    )(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, pair_list) = pairs(input).unwrap();
    pair_list
        .iter()
        .enumerate()
        .filter_map(|(i, Pair { left, right })| match left
            .cmp(right)
        {
            Less => Some(i),
            Equal => panic!("equal??"),
            Greater => None,
        })
        .map(|v| v + 1)
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, pair_list) = pairs(input).unwrap();
    let packet_2 = Packet::List(vec![Packet::List(vec![
        Packet::Number(2),
    ])]);
    let packet_6 = Packet::List(vec![Packet::List(vec![
        Packet::Number(6),
    ])]);
    let mut packets: Vec<&Packet> = pair_list
        .iter()
        .flat_map(|Pair { left, right }| [left, right])
        .chain([&packet_2, &packet_6])
        .collect();
    packets.sort();
    // println!(
    //     "{}",
    //     &packets
    //         .iter()
    //         .map(|v| v.to_string())
    //         .intersperse("\n".to_string())
    //         .collect::<String>()
    // );
    let index_2 = packets
        .iter()
        .enumerate()
        .find(|(_i, packet)| packet == &&&packet_2)
        .unwrap();
    let index_6 = packets
        .iter()
        .enumerate()
        .find(|(_i, packet)| packet == &&&packet_6)
        .unwrap();
    // dbg!(index_2, index_6);
    ((index_2.0 + 1) * (index_6.0 + 1)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn parser_works() {
        use Packet::*;
        assert_eq!(
            pairs(INPUT).unwrap().1,
            vec![
                Pair {
                    left: List(vec![
                        Number(1),
                        Number(1),
                        Number(3),
                        Number(1),
                        Number(1),
                    ]),
                    right: List(vec![
                        Number(1),
                        Number(1),
                        Number(5),
                        Number(1),
                        Number(1),
                    ]),
                },
                Pair {
                    left: List(vec![
                        List(vec![Number(1),]),
                        List(vec![
                            Number(2),
                            Number(3),
                            Number(4),
                        ]),
                    ]),
                    right: List(vec![
                        List(vec![Number(1),]),
                        Number(4),
                    ]),
                },
                Pair {
                    left: List(vec![Number(9),]),
                    right: List(vec![List(vec![
                        Number(8),
                        Number(7),
                        Number(6),
                    ]),]),
                },
                Pair {
                    left: List(vec![
                        List(vec![Number(4), Number(4),]),
                        Number(4),
                        Number(4),
                    ]),
                    right: List(vec![
                        List(vec![Number(4), Number(4),]),
                        Number(4),
                        Number(4),
                        Number(4),
                    ]),
                },
                Pair {
                    left: List(vec![
                        Number(7),
                        Number(7),
                        Number(7),
                        Number(7),
                    ]),
                    right: List(vec![
                        Number(7),
                        Number(7),
                        Number(7),
                    ]),
                },
                Pair {
                    left: List(vec![]),
                    right: List(vec![Number(3),]),
                },
                Pair {
                    left: List(vec![List(vec![List(
                        vec![],
                    ),]),]),
                    right: List(vec![List(vec![]),]),
                },
                Pair {
                    left: List(vec![
                        Number(1),
                        List(vec![
                            Number(2),
                            List(vec![
                                Number(3),
                                List(vec![
                                    Number(4),
                                    List(vec![
                                        Number(5),
                                        Number(6),
                                        Number(7),
                                    ]),
                                ]),
                            ]),
                        ]),
                        Number(8),
                        Number(9),
                    ]),
                    right: List(vec![
                        Number(1),
                        List(vec![
                            Number(2),
                            List(vec![
                                Number(3),
                                List(vec![
                                    Number(4),
                                    List(vec![
                                        Number(5),
                                        Number(6),
                                        Number(0),
                                    ]),
                                ]),
                            ]),
                        ]),
                        Number(8),
                        Number(9),
                    ]),
                },
            ]
        )
    }
    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "13");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "140");
    }
}
