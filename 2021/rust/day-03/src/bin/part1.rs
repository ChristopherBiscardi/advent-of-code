use bitvec::prelude::*;
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
    let stuff = input
        .lines()
        .map(|row| {
            row.split("")
                .filter(|v| v != &"")
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let nrows = stuff.len();
    let ncols = stuff[0].len();
    let mut data = vec![];
    for bits in stuff {
        data.extend_from_slice(&bits);
    }
    let arr = Array2::from_shape_vec((nrows, ncols), data)
        .unwrap();
    let tarr = arr.t();
    let mut totals = Array1::zeros(tarr.nrows());

    Zip::from(&mut totals)
        .and(tarr.rows())
        .for_each(|totals, row| *totals = row.sum());
    // // common
    let mut gamma: BitVec<Msb0> = BitVec::new();
    // // uncommon
    let mut epsilon: BitVec<Msb0> = BitVec::new();
    for v in totals.iter() {
        if (*v as usize) > nrows / 2 {
            gamma.push(true);
            epsilon.push(false);
        } else {
            gamma.push(false);
            epsilon.push(true);
        }
    }

    let g = gamma[0..].load::<u32>();
    let e = epsilon[0..].load::<u32>();

    g * e
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_demo_data() {
        assert_eq!(198, process(input));
    }
}
