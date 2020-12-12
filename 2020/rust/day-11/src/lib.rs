#![feature(non_ascii_idents)]
#![feature(bool_to_option)]

use itertools::Itertools;
use std::cmp::{Ord, Ordering};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;

#[derive(Debug, Clone, Copy)]
enum Seat {
    EMPTY,
    OCCUPIED,
    FLOOR,
}
impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Seat::EMPTY => "L",
                Seat::OCCUPIED => "#",
                Seat::FLOOR => ".",
            }
        )
    }
}
#[derive(Debug)]
enum Grid {
    Grid(HashMap<(usize, usize), Seat>),
}
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Grid::Grid(map) => {
                let len = map.len();
                for (k, v) in
                    map.iter()
                        .sorted_by(|(key1, _), (key2, _)| match Ord::cmp(&key1.0, &key2.0) {
                            Ordering::Less => Ordering::Less,
                            Ordering::Greater => Ordering::Greater,
                            Ordering::Equal => Ord::cmp(&key1.1, &key2.1),
                        })
                {
                    write!(f, "{}", v);
                }
            }
        }
        Ok(())
    }
}

fn get_close_seats(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut vec = vec![];
    if x != 0 && y != 0 {
        vec.push((x - 1, y - 1));
    }
    if y != 0 {
        vec.push((x, y - 1));

        vec.push((x + 1, y - 1));
    }
    if x != 0 {
        vec.push((x - 1, y));
    }
    // vec.push(x, y);
    vec.push((x + 1, y));
    if x != 0 {
        vec.push((x - 1, y + 1));
    }
    vec.push((x, y + 1));
    vec.push((x + 1, y + 1));
    vec
}
fn get_seat(
    x: usize,
    y: usize,
    x_inc: isize,
    y_inc: isize,
    grid: &HashMap<(usize, usize), Seat>,
) -> Option<&Seat> {
    let mut x: isize = (x as isize) + x_inc;
    let mut y: isize = (y as isize) + y_inc;
    loop {
        if x < 0 || y < 0 {
            break None;
        }
        match grid.get(&(x.try_into().unwrap(), y.try_into().unwrap())) {
            Some(thing) => match thing {
                Seat::OCCUPIED | Seat::EMPTY => {
                    break Some(thing);
                }
                _ => {
                    x = x + x_inc;
                    y = y + y_inc;
                }
            },
            None => {
                break None;
            }
        }
    }
}
fn get_visible_seats(x: usize, y: usize, grid: &HashMap<(usize, usize), Seat>) -> Vec<&Seat> {
    let mut vec = vec![];
    if x != 0 && y != 0 {
        match get_seat(x, y, -1, -1, grid) {
            Some(seat) => vec.push(seat),
            None => {}
        }
    };
    if y != 0 {
        match get_seat(x, y, 0, -1, grid) {
            Some(seat) => vec.push(seat),
            None => {}
        }
        match get_seat(x, y, 1, -1, grid) {
            Some(seat) => vec.push(seat),
            None => {}
        }
    };
    if x != 0 {
        match get_seat(x, y, -1, 0, grid) {
            Some(seat) => vec.push(seat),
            None => {}
        }
    };
    // vec.push(x, y);
    match get_seat(x, y, 1, 0, grid) {
        Some(seat) => vec.push(seat),
        None => {}
    };
    if x != 0 {
        match get_seat(x, y, -1, 1, grid) {
            Some(seat) => vec.push(seat),
            None => {}
        }
    };
    match get_seat(x, y, 0, 1, grid) {
        Some(seat) => vec.push(seat),
        None => {}
    };
    match get_seat(x, y, 1, 1, grid) {
        Some(seat) => vec.push(seat),
        None => {}
    };
    vec
}
impl Grid {
    fn step(&self) -> Grid {
        if let Grid::Grid(grid) = self {
            let next_grid: HashMap<(usize, usize), _> =
                grid.iter().fold(HashMap::new(), |mut map, (k, v)| {
                    match v {
                        Seat::FLOOR => {
                            map.insert(*k, Seat::FLOOR);
                        }
                        Seat::EMPTY => {
                            let occupied_seats = get_close_seats(k.0, k.1)
                                .iter()
                                .map(|k| grid.get(k))
                                .filter_map(|seat| {
                                    seat.map(|s| match s {
                                        Seat::OCCUPIED => Some(Seat::OCCUPIED),
                                        _ => None,
                                    })
                                    .flatten()
                                })
                                .count();
                            if occupied_seats == 0 {
                                map.insert(*k, Seat::OCCUPIED);
                            } else {
                                map.insert(*k, *v);
                            }
                        }
                        Seat::OCCUPIED => {
                            let occupied_seats = get_close_seats(k.0, k.1)
                                .iter()
                                .map(|k| grid.get(k))
                                .filter_map(|seat| {
                                    seat.map(|s| match s {
                                        Seat::OCCUPIED => Some(Seat::OCCUPIED),
                                        _ => None,
                                    })
                                    .flatten()
                                })
                                .count();
                            if occupied_seats >= 4 {
                                map.insert(*k, Seat::EMPTY);
                            } else {
                                map.insert(*k, *v);
                            }
                        }
                    };
                    map
                });

            Grid::Grid(next_grid)
        } else {
            panic!("Asfkjlasf")
        }
    }
    fn step2(&self) -> Grid {
        if let Grid::Grid(grid) = self {
            let next_grid: HashMap<(usize, usize), _> =
                grid.iter().fold(HashMap::new(), |mut map, (k, v)| {
                    match v {
                        Seat::FLOOR => {
                            map.insert(*k, Seat::FLOOR);
                        }
                        Seat::EMPTY => {
                            let occupied_seats = get_visible_seats(k.0, k.1, grid)
                                .iter()
                                .filter_map(|seat| match seat {
                                    Seat::OCCUPIED => Some(Seat::OCCUPIED),
                                    _ => None,
                                })
                                .count();
                            if occupied_seats == 0 {
                                map.insert(*k, Seat::OCCUPIED);
                            } else {
                                map.insert(*k, *v);
                            }
                        }
                        Seat::OCCUPIED => {
                            let occupied_seats = get_visible_seats(k.0, k.1, grid)
                                .iter()
                                .filter_map(|seat| match seat {
                                    Seat::OCCUPIED => Some(Seat::OCCUPIED),
                                    _ => None,
                                })
                                .count();
                            if occupied_seats >= 5 {
                                map.insert(*k, Seat::EMPTY);
                            } else {
                                map.insert(*k, *v);
                            }
                        }
                    };
                    map
                });

            Grid::Grid(next_grid)
        } else {
            panic!("Asfkjlasf")
        }
    }
}
fn parse_grid(input: &str) -> Grid {
    let (starting_grid, _, _) =
        input
            .chars()
            .fold((HashMap::new(), 0, 0), |(mut map, x, y), cur| match cur {
                '.' => {
                    map.insert((x, y), Seat::FLOOR);
                    (map, x + 1, y)
                }
                'L' => {
                    map.insert((x, y), Seat::EMPTY);
                    (map, x + 1, y)
                }
                '#' => {
                    map.insert((x, y), Seat::OCCUPIED);
                    (map, x + 1, y)
                }
                _ => (map, 0, y + 1),
            });
    Grid::Grid(starting_grid)
}
pub fn process_part1(input: &str) -> usize {
    let mut last = parse_grid(input);
    loop {
        let next = last.step();
        if last.to_string() == next.to_string() {
            break;
        } else {
            last = next;
        }
    }
    if let Grid::Grid(g) = last {
        g.iter()
            .filter(|(_, v)| if let Seat::OCCUPIED = v { true } else { false })
            .count()
    } else {
        panic!("Asfklj")
    }
}

pub fn process_part2(input: &str) -> usize {
    let mut last = parse_grid(input);
    loop {
        let next = last.step2();
        if last.to_string() == next.to_string() {
            break;
        } else {
            last = next;
        }
    }
    if let Grid::Grid(g) = last {
        g.iter()
            .filter(|(_, v)| if let Seat::OCCUPIED = v { true } else { false })
            .count()
    } else {
        panic!("Asfklj")
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const starting: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    const one: &str = "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";
    const two: &str = "\
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";
    const three: &str = "\
#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##";

    const four: &str = "\
#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##";

    const five: &str = "\
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";

    #[test]
    fn test_input_process_one() {
        assert_eq!(process_part1(starting), 37)
    }
    #[test]
    fn test_input_process_two() {
        assert_eq!(process_part2(starting), 26)
    }
    #[test]
    fn test_input_process_one_step() {
        assert_eq!(
            parse_grid(starting).step().to_string(),
            parse_grid(one).to_string()
        );
        assert_eq!(
            parse_grid(one).step().to_string(),
            parse_grid(two).to_string()
        );
        assert_eq!(
            parse_grid(two).step().to_string(),
            parse_grid(three).to_string()
        );
        assert_eq!(
            parse_grid(three).step().to_string(),
            parse_grid(four).to_string()
        );
        assert_eq!(
            parse_grid(four).step().to_string(),
            parse_grid(five).to_string()
        );
    }
}
