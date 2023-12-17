use crate::custom_error::AocError;
use glam::IVec2;
use itertools::Itertools;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};
use pathfinding::prelude::dijkstra;
use std::collections::{HashMap, VecDeque, HashSet};

type Span<'a> = LocatedSpan<&'a str>;
// type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

fn a_num(input: Span) -> IResult<Span, (IVec2, u32)> {
    let (input, pos) = position(input)?;
    let (input, n) = one_of("0123456789")(input)?;

    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;

    Ok((
        input,
        (
            IVec2::new(x, y),
            n.to_digit(10).unwrap(),
        ),
    ))
}
fn parse_grid(
    input: Span,
) -> IResult<Span, HashMap<IVec2, u32>> {
    let (input, output) =
        separated_list1(line_ending, many1(a_num))(input)?;
    let map = output
        .into_iter()
        .flat_map(|v| v.into_iter())
        .collect::<HashMap<IVec2, u32>>();

    Ok((input, map))
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let row_count = input.lines().count() as i32;
    let column_count =
        input.lines().next().unwrap().len() as i32;
    let (_, grid) =
        parse_grid(Span::new(input)).expect("should parse");

    let boundaries = IVec2::new(column_count, row_count);
    let goal = IVec2::new(column_count - 1, row_count - 1);
    let result: (
        Vec<(IVec2, VecDeque<IVec2>)>,
        u32,
    ) = dijkstra(
        &(IVec2::splat(0), VecDeque::from([IVec2::splat(0)])),
        |(position, deque)| {
           let diffs: Vec<IVec2> = deque.iter().tuple_windows().map(|(a,b)| *a - *b).collect();
           let last_diff = diffs.get(0);

           let maybe_first_diff_count = diffs.iter().dedup_with_count().next();
           let options = if let Some(diff_count) = maybe_first_diff_count {
               let num_consecutive_straight_diffs = diff_count.0;
               let must_turn = num_consecutive_straight_diffs == 10;
               let must_go_straight = num_consecutive_straight_diffs < 4;
            //    let can_go_anywhere = num_consecutive_straight_diffs >= 3 && num_consecutive_straight_diffs < 9;

             if must_turn {
                // "last_diff" is invalid
                [
                    IVec2::new(1, 0),
                    IVec2::new(-1, 0),
                    IVec2::new(0, -1),
                    IVec2::new(0, 1),
                ]
                .into_iter()
                .filter(|option| option != last_diff.unwrap())
                .collect::<Vec<IVec2>>()

               } else if must_go_straight {
                // "last_diff" is only direction to go
                vec![*last_diff.unwrap()]
               } else {
                // can go anywhere
                vec![
                    IVec2::new(1, 0),
                    IVec2::new(-1, 0),
                    IVec2::new(0, -1),
                    IVec2::new(0, 1),
                ]
               }
           } else {
                // runs exactly once to kickstart the process
                vec![
                    IVec2::new(1, 0),
                    IVec2::new(-1, 0),
                    IVec2::new(0, -1),
                    IVec2::new(0, 1),
                ]
           };

            options
            .into_iter()
            .filter_map(|pos_diff| {
             
                let next_position = pos_diff + *position;
                if (0..boundaries.x)
                    .contains(&next_position.x)
                    && (0..boundaries.y)
                        .contains(&next_position.y)
                {
                    if deque.len() > 2 && deque[1] == next_position {
                        return None;
                    }
               

                    let mut new_deque = deque.clone();
                    new_deque.push_front(next_position);

                    if new_deque.len() > 14 {
                        new_deque.pop_back();
                    }
                    Some((next_position, new_deque))
                    
                   
                } else {
                    None
                }
            })
            .map(|pos| {
                let next_cost = *grid.get(&pos.0).unwrap();
                (pos, next_cost)
            })
            .collect::<Vec<((IVec2, VecDeque<IVec2>), u32)>>()
            
        },
        |(win, deque)| {
            // todo: Not too far in a straight
            let diffs: Vec<IVec2> = deque.iter().tuple_windows().map(|(a,b)| *a - *b).collect();

            let maybe_first_diff_count = diffs.iter().dedup_with_count().next();

            maybe_first_diff_count.is_some_and(|(count, _)| count >= 4 ) &&  win == &goal
        },
    ).expect("should have a valid path");

 
    print_grid(&result.0.iter().map(|v|v.0).collect(), &boundaries);
    Ok(result.1.to_string())
}

#[allow(dead_code)]
fn print_grid(map: &HashSet<IVec2>, boundaries: &IVec2) {
    for y in 0..boundaries.y {
        for x in 0..boundaries.x {
            match map.get(&IVec2::new(x, y)) {
                Some(_) => {
                    print!("#");
                }
                None => print!("."),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533","94")]
    #[case("111111111111
999999999991
999999999991
999999999991
999999999991", "71")]
    fn test_process(
        #[case] input: &str,
        #[case] expected: &str
    ) -> miette::Result<()> {
       
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
