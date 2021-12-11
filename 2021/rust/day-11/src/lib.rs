use ndarray::{s, Array2};
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone)]
enum Squid {
    NeedsToFlash,
    Flashed,
    Energy(u8),
    NoSquid,
}
// Squid can be constructed with u8 > 9, which
// breaks the assumptions of the board and impl
// functions
impl Squid {
    // fn should_flash(&self) -> bool {
    //     match self {
    //         Squid::Energy(x) if x > &9 => true,
    //         _ => false,
    //     }
    // }
    fn increment_energy(&mut self) {
        match self {
            Squid::Energy(x) => {
                if x == &9 {
                    *self = Squid::NeedsToFlash
                } else {
                    *x += 1
                }
            }
            _ => {}
        }
    }
    fn reset(&mut self) {
        match self {
            Squid::Flashed => *self = Squid::Energy(0),
            _ => {}
        }
    }
}

fn row(input: &str) -> IResult<&str, Vec<Squid>> {
    let (input, chars) =
        many1(one_of("0123456789"))(input)?;
    let nums = [Squid::NoSquid]
        .into_iter()
        .chain(chars.iter().map(|v| {
            Squid::Energy(
                v.to_digit(10).expect("to have succeeded")
                    as u8,
            )
        }))
        .chain([Squid::NoSquid].into_iter())
        .collect::<Vec<Squid>>();

    Ok((input, nums))
}
fn puzzle_input(
    input: &str,
) -> IResult<&str, Array2<Squid>> {
    let (input, outputs) =
        separated_list1(newline, row)(input)?;
    // dbg!(&outputs);
    let nrows = outputs.len();
    let ncols = outputs[0].len();

    let v = vec![Squid::NoSquid; ncols];
    let data = v
        .iter()
        .cloned()
        .chain(outputs.into_iter().flatten())
        .chain(v.iter().cloned())
        .collect::<Vec<Squid>>();

    let arr =
        Array2::from_shape_vec((nrows + 2, ncols), data)
            .unwrap();
    Ok((input, arr))
}
pub fn process_part1(input: &str) -> u64 {
    let (_, mut squid_map) = puzzle_input(input).unwrap();

    let mut flashes: u64 = 0;
    for _ in 0..100 {
        // Part A: Increment all squids
        for (_, squid) in squid_map.indexed_iter_mut() {
            squid.increment_energy();
        }
        // Part B: Flash all Squids
        loop {
            if !squid_map.indexed_iter().any(
                |(_, squid)| match squid {
                    Squid::NeedsToFlash => true,
                    _ => false,
                },
            ) {
                break;
            }
            let shape = squid_map.shape();
            let row_count = shape[0] - 2;
            let col_count = shape[1] - 2;

            // flash squids
            for row_idx in 0..row_count {
                for col_idx in 0..col_count {
                    let mut squids =
                        squid_map.slice_mut(s![
                            row_idx..=row_idx + 2,
                            col_idx..=col_idx + 2
                        ]);
                    let central_squid = &mut squids[(1, 1)];
                    match central_squid {
                        Squid::NeedsToFlash => {
                            *central_squid = Squid::Flashed;
                            flashes = flashes + 1;
                            for squid in squids.iter_mut() {
                                squid.increment_energy()
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        // Part C: Reset Flashed Squids
        for (_, squid) in squid_map.indexed_iter_mut() {
            squid.reset();
        }
    }

    flashes
}

pub fn process_part2(input: &str) -> u64 {
    let (_, mut squid_map) = puzzle_input(input).unwrap();

    let mut step: u64 = 0;
    // problem counts from "step 1", not "step 0"
    for i in 1.. {
        // Part A: Increment all squids
        for (_, squid) in squid_map.indexed_iter_mut() {
            squid.increment_energy();
        }

        // Part B: Flash all Squids
        loop {
            if !squid_map.indexed_iter().any(
                |(_, squid)| match squid {
                    Squid::NeedsToFlash => true,
                    _ => false,
                },
            ) {
                break;
            }
            let shape = squid_map.shape();
            let row_count = shape[0] - 2;
            let col_count = shape[1] - 2;

            // flash squids
            for row_idx in 0..row_count {
                for col_idx in 0..col_count {
                    let mut squids =
                        squid_map.slice_mut(s![
                            row_idx..=row_idx + 2,
                            col_idx..=col_idx + 2
                        ]);
                    let central_squid = &mut squids[(1, 1)];
                    match central_squid {
                        Squid::NeedsToFlash => {
                            *central_squid = Squid::Flashed;
                            // flashes = flashes + 1;
                            for squid in squids.iter_mut() {
                                squid.increment_energy()
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        if squid_map.indexed_iter().all(|(_, squid)| {
            match squid {
                Squid::Flashed => true,
                Squid::NoSquid => true,
                _ => false,
            }
        }) {
            step = i;
            break;
        }
        // Part C: Reset Flashed Squids
        for (_, squid) in squid_map.indexed_iter_mut() {
            squid.reset();
        }
    }

    step
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &'static str =
        include_str!("./test-input.txt");

    #[test]
    fn part1_test_demo_data() {
        assert_eq!(1656, process_part1(input));
    }
    #[test]
    fn part2_test_demo_data() {
        assert_eq!(195, process_part2(input));
    }
}
