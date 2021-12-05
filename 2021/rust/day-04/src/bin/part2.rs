use bitvec::prelude::*;
use day_04::{board, Board};
use day_04::{puzzle_input, BoardState};
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
    // let inititalized_boards =
    let (_, (callouts, mut boards)) =
        puzzle_input(input).unwrap();
    let mut playing_boards = boards
        .iter_mut()
        .map(|board| (board, BoardState::Playing))
        .collect::<Vec<(&mut Board, BoardState)>>();
    let mut winning_boards: Vec<u32> = vec![];
    for callout in callouts.iter() {
        for (board, state) in playing_boards
            .iter_mut()
            .filter(|(_, state)| match state {
                BoardState::Finished(_) => false,
                BoardState::Playing => true,
            })
        {
            *state = board.mark(callout);
            match state {
                BoardState::Finished(fin) => {
                    winning_boards.push(*fin);
                }
                BoardState::Playing => {}
            }
        }
    }
    *winning_boards.iter().last().unwrap()
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
