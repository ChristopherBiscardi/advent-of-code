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
           
            [
                IVec2::new(1, 0),
                IVec2::new(-1, 0),
                IVec2::new(0, -1),
                IVec2::new(0, 1),
            ]
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
                    if new_deque.len() == 5 {
              
                        let dir = new_deque[1] - new_deque[0];
                        let a = new_deque[2] - new_deque[1];
                        let b = new_deque[3] - new_deque[2];
                        let c = new_deque[4] - new_deque[3];
                        // if we've moved in the same direction 4 times
                        let three_forward_check = [
                            a,
                            b,
                            c,
                        ].iter().all(|a_dir| a_dir == &dir);
                            
                        if three_forward_check {
                            None
                        } else {
                         
                            new_deque.pop_back();
                            Some((next_position, new_deque))
                        }
                    } else {
                        
                        Some((next_position, new_deque))
                    }
                    
                   
                } else {
                    None
                }
            })
            .map(|pos| {
            let next_cost =    *grid.get(&pos.0).unwrap();
                (pos, next_cost)
            })
            .collect::<Vec<((IVec2, VecDeque<IVec2>), u32)>>()
        },
        |(win, _deque)| {
            // todo: Not too far in a straight
            win == &goal
        },
    ).expect("should have a valid path");

 
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
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2413432311323
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
4322674655533";
        assert_eq!("102", process(input)?);
        Ok(())
    }
}
