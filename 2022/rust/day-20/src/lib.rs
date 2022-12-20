use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete,
    character::complete::line_ending,
    combinator::{eof, iterator},
    multi::separated_list1,
    sequence::terminated,
    *,
};
use tracing::*;
use tracing_subscriber;

#[instrument(skip(input))]
fn numbers(
    input: &str,
) -> IResult<&str, Vec<(usize, i64)>> {
    let mut it = iterator(
        input,
        terminated(complete::i64, alt((line_ending, eof))),
    );
    let numbers = it.enumerate().collect::<Vec<_>>();
    info!(?numbers);
    let (input, _) = it.finish()?;
    Ok((input, numbers))
}

#[instrument(skip(input))]
pub fn process_part1(input: &str) -> String {
    let (_, numbers) = numbers(input).unwrap();
    let mut state = numbers.clone();
    info!(?state);
    for (id, value) in numbers.iter() {
        info!(?value, "moving");
        let index = state
            .iter()
            .position(|state_value| state_value.0 == *id)
            .unwrap();

        let current = state.remove(index);
        let added = index as i64 + current.1;
        let new_index =
            added.rem_euclid(state.len() as i64);

        info!(index, new_index);

        state.insert(new_index as usize, current);

        info!(
            "{:?}",
            state.iter().map(|v| v.1).collect::<Vec<_>>()
        );
    }

    let zero_pos =
        state.iter().position(|v| v.1 == 0).unwrap();
    let a = state[(1000 + zero_pos) % state.len()].1;
    let b = state[(2000 + zero_pos) % state.len()].1;
    let c = state[(3000 + zero_pos) % state.len()].1;
    info!(a, b, c, "ABC");
    (a + b + c).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut numbers) = numbers(input).unwrap();
    numbers
        .iter_mut()
        .for_each(|tuple| tuple.1 *= 811589153);
    let mut state = numbers.clone();
    info!(?state);
    for _ in 0..10 {
        for (id, value) in numbers.iter() {
            info!(?value, "moving");
            let index = state
                .iter()
                .position(|state_value| {
                    state_value.0 == *id
                })
                .unwrap();

            let current = state.remove(index);
            let added = index as i64 + current.1;
            let new_index =
                added.rem_euclid(state.len() as i64);

            info!(index, new_index);

            state.insert(new_index as usize, current);

            info!(
                "{:?}",
                state
                    .iter()
                    .map(|v| v.1)
                    .collect::<Vec<_>>()
            );
        }
    }

    let zero_pos =
        state.iter().position(|v| v.1 == 0).unwrap();
    let a = state[(1000 + zero_pos) % state.len()].1;
    let b = state[(2000 + zero_pos) % state.len()].1;
    let c = state[(3000 + zero_pos) % state.len()].1;
    info!(a, b, c, "ABC");
    (a + b + c).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    #[ignore]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part1(INPUT), "3");
    }

    #[test]
    fn part2_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part2(INPUT), "1623178306");
    }
}
