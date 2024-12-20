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
    collections::{HashMap, HashSet},
    fmt::{self, Display, Write},
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let new_input = input
        .chars()
        .map(|c| match c {
            '#' => "##".to_string(),
            'O' => "[]".to_string(),
            '.' => "..".to_string(),
            '@' => "@.".to_string(),
            other => other.to_string(),
        })
        .collect::<String>();
    let (_, (mut map, directions)) =
        parse(Span::new(&new_input)).map_err(|e| {
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
            Object::BoxLeft => {
                // println!("BoxLeft");
                if direction.x == 0 {
                    // vertical movement
                    vertical(
                        &Object::BoxLeft,
                        &mut map,
                        next_pos,
                        direction,
                        robot,
                    );
                } else {
                    // horizontal movement
                    // always moving East
                    horizontal(
                        &mut map, next_pos, direction,
                        robot,
                    );
                }
            }
            Object::BoxRight => {
                // println!("BoxRight");
                if direction.x == 0 {
                    // println!("vertical");
                    // vertical movement
                    vertical(
                        &Object::BoxRight,
                        &mut map,
                        next_pos,
                        direction,
                        robot,
                    );
                } else {
                    // println!("horizontal");
                    // horizontal movement
                    // always moving West
                    horizontal(
                        &mut map, next_pos, direction,
                        robot,
                    );
                }
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
        .filter(|(_, obj)| obj == &&Object::BoxLeft)
        .map(|(pos, _)| 100 * pos.y + pos.x)
        .sum();

    Ok(result.to_string())
}

fn vertical(
    object: &Object,
    map: &mut HashMap<IVec2, Object>,
    next_pos: IVec2,
    direction: IVec2,
    robot: IVec2,
) {
    // println!("vertical");
    let mut seen_items: HashSet<IVec2> =
        HashSet::from([robot]);
    let mut next_items: Vec<IVec2> = match object {
        Object::BoxLeft => {
            vec![next_pos, next_pos + IVec2::X]
        }
        Object::BoxRight => {
            vec![next_pos + IVec2::NEG_X, next_pos]
        }
        _ => unreachable!(""),
    };
    // check all objects until wall

    while !next_items.is_empty() {
        let mut new_items: HashSet<IVec2> =
            HashSet::default();
        for item in &next_items {
            // check in direction
            let next_pos = *item + direction;
            match map.get(&next_pos) {
                Some(Object::Wall) => {
                    return;
                }
                Some(Object::BoxLeft) => {
                    // get box right
                    new_items.insert(next_pos);
                    new_items.insert(next_pos + IVec2::X);
                }
                Some(Object::BoxRight) => {
                    // get box left
                    new_items.insert(next_pos);
                    new_items
                        .insert(next_pos + IVec2::NEG_X);
                }
                Some(_) => {
                    unreachable!("");
                }
                None => {}
            }
        }
        for item in &next_items {
            seen_items.insert(*item);
        }
        next_items = vec![];
        for item in &new_items {
            next_items.push(*item);
        }
    }

    let mut items: Vec<IVec2> =
        seen_items.into_iter().collect();
    items.sort_by(|a, b| {
        if direction.y > 0 {
            b.y.cmp(&a.y)
        } else {
            a.y.cmp(&b.y)
        }
    });

    for item in items {
        let v = map.remove(&item).unwrap();
        map.insert(item + direction, v);
    }
    // println!("{}", debug_grid(&map).unwrap());
}

fn horizontal(
    map: &mut HashMap<IVec2, Object>,
    next_pos: IVec2,
    direction: IVec2,
    robot: IVec2,
) {
    // check all objects until wall or space
    let mut items = vec![next_pos];
    while [Some(&Object::BoxLeft), Some(&Object::BoxRight)]
        .contains(&map.get(
            &(items.iter().last().unwrap() + direction),
        ))
    {
        items
            .push(items.iter().last().unwrap() + direction);
    }
    // println!("{:?}", items);
    // next is always a wall because of
    // other checks
    let Some(next) = map
        .get(&(items.iter().last().unwrap() + direction))
    else {
        // bot *and* next item can move
        let mut last_item = map
            .remove(&robot)
            .expect("robot to exist when removing");
        for item_location in &items {
            let item_to_insert = last_item;
            // remove the item
            last_item = map
                .remove(&item_location)
                .expect("robot to exist when removing");
            map.insert(*item_location, item_to_insert);
        }

        let location = items.iter().last().unwrap();
        map.insert(*location + direction, last_item);

        // println!("{}", debug_grid(&map).unwrap());
        return;
    };
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
    BoxLeft,
    BoxRight,
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
                Object::BoxLeft => "[",
                Object::BoxRight => "]",
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
        value(Object::BoxLeft, complete::char('[')),
        value(Object::BoxRight, complete::char(']')),
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
        "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^",
        "no answer"
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
"9021")]
    fn test_process(
        #[case] input: &str,
        #[case] result: &str,
    ) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
