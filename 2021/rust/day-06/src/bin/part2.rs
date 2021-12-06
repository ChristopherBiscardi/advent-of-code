use day_05::puzzle_input;
#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use std::collections::BTreeMap;
use std::fs;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();

    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process(&file, 256));
}

fn process(input: &str, num_days_to_process: u32) -> usize {
    let (_, mut fishes) = puzzle_input(input).unwrap();
    for _ in 0..num_days_to_process {
        let mut new_fish = vec![];
        for fish in fishes.iter_mut() {
            match fish {
                0 => {
                    *fish = 6;
                    new_fish.push(8);
                }
                1..=8 => {
                    *fish -= 1;
                }
                _ => panic!("immortal fish. run."),
            }
        }
        fishes.extend_from_slice(&new_fish);
    }
    fishes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &'static str = "3,4,3,1,2";

    #[test]
    fn test_demo_data() {
        assert_eq!(26, process(input, 18));
    }
    #[test]
    fn test_demo_data_2() {
        assert_eq!(5934, process(input, 80));
    }
}
