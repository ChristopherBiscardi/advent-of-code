use day_05::process_part1;
#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};
use std::fs;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();

    let file = fs::read("./input.txt").unwrap();
    println!("{}", process_part1(&file));
}
