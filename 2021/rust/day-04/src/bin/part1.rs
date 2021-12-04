use bitvec::prelude::*;
use day_04::board;
use day_04::puzzle_input;
#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use ndarray::{Array1, Array2, Axis, Zip};
use std::fs;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();

    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process(&file));
}

fn process(input: &str) -> u32 {
    let (_, (callouts, mut boards)) =
        puzzle_input(input).unwrap();
    let winning_number =
        callouts.iter().find_map(|callout| {
            let mut fin = None;
            for board in boards.iter_mut() {
                let result = board.mark(callout);
                match result {
                    day_04::BoardState::Finished(
                        winning_number,
                    ) => {
                        fin = Some(winning_number);
                        break;
                    }
                    day_04::BoardState::Playing => {}
                };
            }
            fin
        });
    winning_number.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &'static str =
        include_str!("./test-input.txt");

    #[test]
    fn test_demo_data() {
        assert_eq!(4512, process(input));
    }
}
