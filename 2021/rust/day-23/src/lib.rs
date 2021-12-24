use core::fmt;
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    iter::{self, Sum},
    ops::Add,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{
        self, alpha1, anychar, newline, one_of, u32,
    },
    combinator::opt,
    multi::{
        length_value, many0, many1, many_m_n,
        separated_list1,
    },
    sequence::{
        pair, preceded, separated_pair, terminated,
    },
    IResult,
};

// #[derive(Debug)]
// struct MapResult {
//     map: Map,
//     results: Vec<MapResult>,
// }
#[derive(Debug, Eq, PartialEq, Clone)]
struct Map {
    hallway: Vec<Space>,
    slots: Vec<Homes>,
}
impl Map {
    fn enumerate_moves(
        &self,
        current_cost: usize,
        num_moves: usize,
    ) -> Vec<(Map, usize)> {
        let goal_map = Map {
            hallway: vec![Empty; 11],
            slots: vec![
                Wall,
                Wall,
                Home(vec![Occupied('A'), Occupied('A')]),
                Wall,
                Home(vec![Occupied('B'), Occupied('B')]),
                Wall,
                Home(vec![Occupied('C'), Occupied('C')]),
                Wall,
                Home(vec![Occupied('D'), Occupied('D')]),
                Wall,
                Wall,
            ],
        };

        let new_maps = self.move_to_goal_slot(current_cost);
        // println!("\n--moves-to-goal-slot--\n",);
        // for (map, cost) in new_maps.iter() {
        //     println!("cost: {}", cost);
        //     println!("{}", map);
        // }

        // move from slot into hallway
        let additional_maps =
            self.move_to_hallway_slot(current_cost);
        // println!("\n--moves-to-hallway-slot--\n",);
        // for (map, cost) in additional_maps.iter() {
        //     println!("cost: {}", cost);
        //     println!("{}", map);
        // }
        // dbg!(new_maps.len(), additional_maps.len());
        // if new_maps.len() > 0 {
        //     // println!("new maps");
        //     for (map, cost) in new_maps.iter() {
        //         // println!("cost: {}", cost);
        //         // println!("{}", map);
        //     }
        // }

        if self == &goal_map {
            // println!(
            //     "\n+++parent+map+++\nnum_moves: {}\n{}\n-------\n\n",
            //     num_moves,
            //     self
            // );
            // println!("{}", current_cost);
            // println!(
            //     "out: {}, in: {}",
            //     additional_maps.len(),
            //     new_maps.len()
            // );
            return vec![];
        };
        if new_maps.len() == 0 && additional_maps.len() == 0
        {
            // println!("no more moves\n{}", self);
            return vec![];
        }
        if num_moves == 6 {
            return vec![(self.clone(), current_cost)];
        }
        new_maps
            .iter()
            .chain(additional_maps.iter())
            .map(|(m, cost)| {
                // dbg!("flat_map");

                m.enumerate_moves(*cost, num_moves + 1)
            })
            .inspect(|v| {
                if v.len() != 0 {
                    // dbg!(v);
                }
            })
            .flatten()
            .collect::<Vec<(Map, usize)>>()
    }
    fn move_to_goal_slot(
        &self,
        current_cost: usize,
    ) -> Vec<(Map, usize)> {
        let charset = ['A', 'B', 'C', 'D'];
        let costs = BTreeMap::from([
            ('A', 1),
            ('B', 10),
            ('C', 100),
            ('D', 1000),
        ]);
        let movable_chars = self
            .hallway
            .iter()
            .enumerate()
            .filter_map(|(i, c)| match c {
                Empty => None,
                Occupied(c) => Some((i, c)),
            })
            .collect::<Vec<(usize, &char)>>();
        let possible_slots = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, home)| match home {
                Wall => None,
                Home(h) => Some((i, h)),
            })
            .collect::<Vec<(usize, &Vec<Space>)>>();
        let mut new_maps: Vec<(Map, usize)> = vec![];
        for ((slot_index, slot), slot_char) in
            possible_slots.iter().zip(charset)
        {
            for (char_index, &c) in &movable_chars {
                // if the hallway char is a valid char for
                // this slot
                if c == slot_char
                // slot must have 0 or 1 char in it,
                // to be able to accept a new char
                    && slot.len() < 2
                    // slot members must all be the valid
                    // slot char to be able to move a hallway
                    // valid slot char in
                    && slot
                        .iter()
                        .all(|c| if let Occupied(c) = c {
                           c == &slot_char
                        } else {
                            false
                        })
                        // if there is *not* a char in the way of
                        // the hallway char moving into the slot
                    && movable_chars.iter().filter(|(_, &c2)| c2 != c).find(|(idx,_)| {
                           ((slot_index.min(char_index))..=(slot_index.max(char_index))).contains(&idx)
                        }).is_none()
                {
                    let mut new_map = self.clone();
                    let old_space = new_map
                        .hallway
                        .get_mut(*char_index)
                        .unwrap();
                    *old_space = Empty;
                    let slot = new_map
                        .slots
                        .get_mut(*slot_index)
                        .unwrap();
                    let mut space_cost = 0;
                    let multiplier = costs.get(&c).unwrap();
                    if let Home(slot) = slot {
                        space_cost = 2 - slot.len();
                        slot.push(Occupied(c));
                    } else {
                        panic!("tried to move into non-existing slot");
                    }
                    let hallway_cost = slot_index
                        .max(char_index)
                        - slot_index.min(char_index);
                    new_maps.push((
                        new_map,
                        current_cost
                            + ((hallway_cost + space_cost)
                                * multiplier),
                    ));

                    // then move char in
                }
            }
        }

        new_maps
    }
    fn move_to_hallway_slot(
        &self,
        current_cost: usize,
    ) -> Vec<(Map, usize)> {
        let charset = ['A', 'B', 'C', 'D'];
        let costs = BTreeMap::from([
            ('A', 1),
            ('B', 10),
            ('C', 100),
            ('D', 1000),
        ]);
        // for each possible slot end space
        // move out to any available hallway slot
        let slot_indices = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, h)| match h {
                Wall => None,
                Home(_) => Some(i),
            })
            .collect::<Vec<usize>>();
        let mut new_maps: Vec<(Map, usize)> = vec![];

        let slotsaskfljasf = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, home)| match home {
                Wall => None,
                Home(h) => Some((i, h)),
            })
            .collect::<Vec<(usize, &Vec<Space>)>>();

        for ((slot_index, slot), valid_char_for_slot) in
            slotsaskfljasf.iter().zip(charset.iter())
        {
            if !slot.is_empty()
                && !slot.iter().all(|v| {
                    v == &Occupied(*valid_char_for_slot)
                })
            {
                if let Occupied(slot_char) =
                    slot.iter().last().unwrap()
                {
                    let possible_hallway_spaces = self
                        .hallway
                        .iter()
                        .enumerate()
                        .filter_map(
                            |(hall_index, space)| {
                                if slot_indices
                                    .contains(&hall_index)
                                {
                                    None
                                } else {
                                    Some((
                                        hall_index, space,
                                    ))
                                }
                            },
                        )
                        .collect::<Vec<(usize, &Space)>>();
                    for (possible_space_index, space) in
                        possible_hallway_spaces.iter()
                    {
                        match space {
                            Empty => {
                                // possible if not blocked by Occupied space
                                let blocker =  possible_hallway_spaces.iter().find(|(i,c)|{
                                    ((slot_index.max(possible_space_index))..=slot_index.min(possible_space_index)).contains(&i)
                                });
                                if let None = blocker {
                                    // add this to maps

                                    let mut new_map =
                                        self.clone();

                                    let home = new_map
                                        .slots
                                        .get_mut(
                                            *slot_index,
                                        )
                                        .unwrap();

                                    let mut space_cost = 0;

                                    let new_char =
                                        if let Home(slot) =
                                            home
                                        {
                                            space_cost = 2
                                                - slot
                                                    .len();
                                            slot.pop()
                                                .unwrap()
                                        } else {
                                            panic!("home slot should always be a vec here");
                                        };

                                    let new_space = new_map
                                        .hallway
                                        .get_mut(
                                            *possible_space_index,
                                        )
                                        .unwrap();
                                    *new_space =
                                        new_char.clone();

                                    let multiplier =
                                        if let Occupied(c) =
                                            new_char
                                        {
                                            costs.get(&c)
                                        } else {
                                            panic!("shouldnt happen");
                                        };

                                    let hallway_cost =
                                        slot_index.max(
                                            possible_space_index,
                                        ) - slot_index.min(
                                            possible_space_index,
                                        );
                                    new_maps.push((
                                        new_map,
                                        current_cost
                                            + ((hallway_cost
                                            + space_cost) * multiplier.unwrap()),
                                    ));
                                }
                            }
                            Occupied(_) => {}
                        }
                    }
                    // slot_char;
                } else {
                    panic!("all slots should have occupied spaces");
                }
            }
        }
        new_maps
    }
}

impl fmt::Display for Map {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "{}\n{}\n{}",
            self.hallway
                .iter()
                .map(|s| {
                    match s {
                        Empty => ".".to_string(),
                        Occupied(c) => c.to_string(),
                    }
                })
                .collect::<String>(),
            self.slots
                .iter()
                .map(|homes| {
                    match homes {
                        Wall => "#".to_string(),
                        Home(slot) => match slot.get(1) {
                            Some(Occupied(c)) => {
                                c.to_string()
                            }
                            Some(Empty) => ".".to_string(),
                            None => ".".to_string(),
                        },
                    }
                })
                .collect::<String>(),
            self.slots
                .iter()
                .map(|homes| {
                    match homes {
                        Wall => "#".to_string(),
                        Home(slot) => match slot.get(0) {
                            Some(Occupied(c)) => {
                                c.to_string()
                            }
                            Some(Empty) => ".".to_string(),
                            None => ".".to_string(),
                        },
                    }
                })
                .collect::<String>()
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Space {
    Empty,
    Occupied(char),
}
impl fmt::Display for Space {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "{}",
            match self {
                Empty => ".".to_string(),
                Occupied(c) => c.to_string(),
            }
        )
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
enum Homes {
    Wall,
    Home(Vec<Space>),
}
use Homes::*;
use Space::*;

fn puzzle_input(input: &str) -> IResult<&str, Vec<usize>> {
    // let (input, commands) =
    //     separated_list1(newline, command)(input)?;
    // Ok((input, commands))
    todo!()
}

// #############
// #...........#
// ###B#C#B#D###
//   #A#D#C#A#
//   #########
// fn cost()
pub fn process_part1(input: &str) -> usize {
    // let (_, commands) =
    //     puzzle_input(input).expect("input to be valid");
    let start_map = Map {
        hallway: vec![Empty; 11],
        slots: vec![
            Wall,
            Wall,
            Home(vec![Occupied('A'), Occupied('B')]),
            Wall,
            Home(vec![Occupied('D'), Occupied('C')]),
            Wall,
            Home(vec![Occupied('C'), Occupied('B')]),
            Wall,
            Home(vec![Occupied('A'), Occupied('D')]),
            Wall,
            Wall,
        ],
    };
    // let start_map = Map {
    //     hallway: vec![Empty; 11],
    //     slots: vec![
    //         Wall,
    //         Wall,
    //         Home(vec![Occupied('B'), Occupied('D')]),
    //         Wall,
    //         Home(vec![Occupied('C'), Occupied('B')]),
    //         Wall,
    //         Home(vec![Occupied('A'), Occupied('C')]),
    //         Wall,
    //         Home(vec![Occupied('A'), Occupied('D')]),
    //         Wall,
    //         Wall,
    //     ],
    // };
    println!("--start_map--\n\n{}", &start_map);

    let goal_map = Map {
        hallway: vec![Empty; 11],
        slots: vec![
            Wall,
            Wall,
            Home(vec![Occupied('A'), Occupied('A')]),
            Wall,
            Home(vec![Occupied('B'), Occupied('B')]),
            Wall,
            Home(vec![Occupied('C'), Occupied('C')]),
            Wall,
            Home(vec![Occupied('D'), Occupied('D')]),
            Wall,
            Wall,
        ],
    };

    let new_maps = start_map.enumerate_moves(0, 0);
    dbg!(new_maps.len());
    dbg!(new_maps
        .iter()
        .filter(|v| v.0 == goal_map)
        .min_by_key(|v| v.1));
    0
}

pub fn process_part2(input: &str) -> usize {
    let (_, commands) =
        puzzle_input(input).expect("input to be valid");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "";
    // #[test]
    // fn part1_test_demo_data() {
    //     assert_eq!(12521, process_part1(INPUT));
    // }
    #[test]
    fn test_move_into_slot2() {
        let map = Map {
            hallway: vec![
                Empty,
                Occupied('A'),
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
            ],
            slots: vec![
                Wall,
                Wall,
                Home(vec![Occupied('A')]),
                Wall,
                Home(vec![Occupied('B'), Occupied('B')]),
                Wall,
                Home(vec![Occupied('C'), Occupied('C')]),
                Wall,
                Home(vec![Occupied('D'), Occupied('D')]),
                Wall,
                Wall,
            ],
        };
        assert_eq!(
            vec![(
                Map {
                    hallway: vec![Empty; 11],
                    slots: vec![
                        Wall,
                        Wall,
                        Home(vec![
                            Occupied('A'),
                            Occupied('A')
                        ]),
                        Wall,
                        Home(vec![
                            Occupied('B'),
                            Occupied('B')
                        ]),
                        Wall,
                        Home(vec![
                            Occupied('C'),
                            Occupied('C')
                        ]),
                        Wall,
                        Home(vec![
                            Occupied('D'),
                            Occupied('D')
                        ]),
                        Wall,
                        Wall,
                    ],
                },
                2
            )],
            map.move_to_goal_slot(0)
        )
    }

    #[test]
    fn test_move_into_slot_fail() {
        let map = Map {
            hallway: vec![
                Empty,
                Occupied('A'),
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
            ],
            slots: vec![
                Wall,
                Wall,
                Home(vec![Occupied('A'), Occupied('B')]),
                Wall,
                Home(vec![Occupied('B')]),
                Wall,
                Home(vec![Occupied('C'), Occupied('C')]),
                Wall,
                Home(vec![Occupied('D'), Occupied('D')]),
                Wall,
                Wall,
            ],
        };
        let answer: Vec<(Map, usize)> = vec![];
        assert_eq!(answer, map.move_to_goal_slot(0))
    }

    #[test]
    fn test_move_into_hallway() {
        let map = Map {
            hallway: vec![
                Empty, Empty, Empty, Empty, Empty, Empty,
                Empty, Empty, Empty, Empty, Empty,
            ],
            slots: vec![
                Wall,
                Wall,
                Home(vec![Occupied('A'), Occupied('B')]),
                Wall,
                Home(vec![Occupied('B'), Occupied('A')]),
                Wall,
                Home(vec![Occupied('C'), Occupied('C')]),
                Wall,
                Home(vec![Occupied('D'), Occupied('D')]),
                Wall,
                Wall,
            ],
        };
        assert_eq!(
            vec![
                (
                    Map {
                        hallway: vec![
                            Occupied('B'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![Occupied('A')]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    2
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Occupied('B'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![Occupied('A')]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    1
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Occupied('B'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![Occupied('A')]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    1
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('B'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![Occupied('A')]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    3
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('B'),
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![Occupied('A')]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    5
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('B'),
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![Occupied('A')]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    7
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('B')
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![Occupied('A')]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    8
                ),
                (
                    Map {
                        hallway: vec![
                            Occupied('A'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![Occupied('B')]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    4
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Occupied('A'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![Occupied('B')]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    3
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Occupied('A'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![Occupied('B')]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    1
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('A'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![Occupied('B')]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    1
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('A'),
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![Occupied('B')]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    3
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('A'),
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![Occupied('B')]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    5
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('A')
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![Occupied('B')]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    6
                ),
                (
                    Map {
                        hallway: vec![
                            Occupied('C'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![Occupied('C')]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    6
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Occupied('C'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![Occupied('C')]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    5
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Occupied('C'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![Occupied('C')]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    3
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('C'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![Occupied('C')]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    1
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('C'),
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![Occupied('C')]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    1
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('C'),
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![Occupied('C')]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    3
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('C')
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![Occupied('C')]),
                            Wall,
                            Home(vec![
                                Occupied('D'),
                                Occupied('D')
                            ]),
                            Wall,
                            Wall
                        ]
                    },
                    4
                ),
                (
                    Map {
                        hallway: vec![
                            Occupied('D'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![Occupied('D')]),
                            Wall,
                            Wall
                        ]
                    },
                    8
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Occupied('D'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![Occupied('D')]),
                            Wall,
                            Wall
                        ]
                    },
                    7
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Occupied('D'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![Occupied('D')]),
                            Wall,
                            Wall
                        ]
                    },
                    5
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('D'),
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![Occupied('D')]),
                            Wall,
                            Wall
                        ]
                    },
                    3
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('D'),
                            Empty,
                            Empty,
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![Occupied('D')]),
                            Wall,
                            Wall
                        ]
                    },
                    1
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('D'),
                            Empty
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![Occupied('D')]),
                            Wall,
                            Wall
                        ]
                    },
                    1
                ),
                (
                    Map {
                        hallway: vec![
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Empty,
                            Occupied('D')
                        ],
                        slots: vec![
                            Wall,
                            Wall,
                            Home(vec![
                                Occupied('A'),
                                Occupied('B')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('B'),
                                Occupied('A')
                            ]),
                            Wall,
                            Home(vec![
                                Occupied('C'),
                                Occupied('C')
                            ]),
                            Wall,
                            Home(vec![Occupied('D')]),
                            Wall,
                            Wall
                        ]
                    },
                    2
                )
            ],
            map.move_to_hallway_slot(0)
        )
    }
    // #[test]
    // fn part2_test_demo_data() {
    //     assert_eq!(3351, process_part2(INPUT));
    // }
}
