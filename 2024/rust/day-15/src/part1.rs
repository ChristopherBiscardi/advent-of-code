use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::is_a,
    character::complete::{self, line_ending, multispace1},
    combinator::{opt, value},
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};
use nom_locate::{position, LocatedSpan};
use std::{
    collections::HashMap,
    fmt::{self, Display, Write},
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (mut map, directions)) =
        parse(Span::new(input)).map_err(|e| {
            miette::miette!("parse failed {}", e)
        })?;

    // println!("{}", debug_grid(&map).unwrap());
    for direction in directions {
        // println!("move {:?}", direction);
        let robot = map
            .iter()
            .find(|(_, object)| object == &&Object::Robot)
            .expect("a robot")
            .0
            .clone();

        let next_pos = robot + direction;
        let Some(next) = map.get(&next_pos) else {
            // bot can move
            let bot = map
                .remove(&robot)
                .expect("robot to exist when removing");
            map.insert(next_pos, bot);
            // println!("{}", debug_grid(&map).unwrap());

            continue;
        };

        match next {
            Object::Wall => {
                // println!("{}",
                // debug_grid(&map).unwrap());
                continue;
            }
            Object::Box => {
                // check all objects until wall or space
                let mut items = vec![next_pos];
                while Some(&Object::Box)
                    == map.get(
                        &(items.iter().last().unwrap()
                            + direction),
                    )
                {
                    items.push(
                        items.iter().last().unwrap()
                            + direction,
                    );
                }
                // println!("{:?}", items);
                // next is always a wall because of
                // other checks
                let Some(next) = map.get(
                    &(items.iter().last().unwrap()
                        + direction),
                ) else {
                    // bot *and* next item can move
                    let bot = map.remove(&robot).expect(
                        "robot to exist when removing",
                    );
                    let mut it = items.iter();
                    let next_item_location =
                        it.next().unwrap();
                    let next_ = map
                        .remove(next_item_location)
                        .expect(
                            "robot to exist when removing",
                        );
                    map.insert(*next_item_location, bot);
                    match it.last() {
                        Some(location) => {
                            // println!(
                            //     "inserting {}",
                            //     location
                            // );
                            map.insert(
                                *location + direction,
                                next_,
                            );
                        }
                        None => {
                            // println!(
                            //     "inserting2 {}",
                            //     next_item_location
                            //         + direction
                            // );
                            map.insert(
                                next_item_location
                                    + direction,
                                next_,
                            );
                        }
                    }
                    // next_item_location
                    // println!(
                    //     "{}",
                    //     debug_grid(&map).unwrap()
                    // );
                    continue;
                };
            }
            Object::Robot => {
                unreachable!(
                    "should never see a second robot"
                );
            }
        }
    }
    let result: i32 = map
        .iter()
        .filter(|(_, obj)| obj == &&Object::Box)
        .map(|(pos, _)| 100 * pos.y + pos.x)
        .sum();

    Ok(result.to_string())
}

fn debug_grid(
    objects: &HashMap<IVec2, Object>,
) -> Result<String, fmt::Error> {
    let map_size = IVec2::new(
        objects.keys().map(|pos| pos.x).max().unwrap(),
        objects.keys().map(|pos| pos.y).max().unwrap(),
    );
    let mut output = String::new();
    writeln!(&mut output, "")?;
    for y in 0..=map_size.y {
        for x in 0..=map_size.x {
            match objects.get(&IVec2::new(x, y)) {
                Some(obj) => {
                    write!(&mut output, "{obj}")?;
                }
                None => {
                    write!(&mut output, ".",)?;
                }
            }
        }
        writeln!(&mut output)?;
    }
    Ok(output)
}

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Object {
    Wall,
    Box,
    Robot,
}
impl Display for Object {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Object::Wall => "#",
                Object::Box => "O",
                Object::Robot => "@",
            }
        )
    }
}

fn object_pos(
    input: Span,
) -> IResult<Span, (IVec2, Object)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, obj) = alt((
        value(Object::Wall, complete::char('#')),
        value(Object::Box, complete::char('O')),
        value(Object::Robot, complete::char('@')),
    ))(input)?;
    Ok((input, (IVec2::new(x, y), obj)))
}
fn parse(
    input: Span,
) -> IResult<Span, (HashMap<IVec2, Object>, Vec<IVec2>)> {
    let (input, lines) = separated_list1(
        line_ending,
        many1(preceded(opt(is_a(".")), object_pos)),
    )(input)?;

    let (input, directions) = preceded(
        multispace1,
        separated_list1(
            line_ending,
            many1(alt((
                value(IVec2::NEG_Y, complete::char('^')),
                value(IVec2::Y, complete::char('v')),
                value(IVec2::X, complete::char('>')),
                value(IVec2::NEG_X, complete::char('<')),
            ))),
        ),
    )(input)?;

    let hashmap = lines.into_iter().flatten().collect();
    Ok((
        input,
        (
            hashmap,
            directions.into_iter().flatten().collect(),
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
        "2028"
    )]
    #[case("##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
"10092")]
    fn test_process(
        #[case] input: &str,
        #[case] result: &str,
    ) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
