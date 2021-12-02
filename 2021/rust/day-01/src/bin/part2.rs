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
    let seafloor: Vec<u16> = file
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u16>>();

    let mut i = 0;

    for items in seafloor.windows(4) {
        if let [one, two, three, four] = items {
            let i_one = one + two + three;
            let i_two = two + three + four;
            if i_two > i_one {
                i += 1;
            }
        } else {
            panic!("slice wasn't length 4");
        }
    }

    dbg!(i);
}
