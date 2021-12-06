use day_05::puzzle_input;
#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use std::fs;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();

    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process(&file, 80));
}

fn process(input: &str, num_days_to_process: u32) -> u64 {
    let (_, mut fishes) = puzzle_input(input).unwrap();
    for _ in 0..num_days_to_process {
        fishes.rotate_left(1);

        let new_fishes = fishes.get(8).unwrap().clone();
        let old_fishes = fishes.get_mut(6).unwrap();
        *old_fishes += new_fishes;
    }
    fishes.iter().sum()
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
