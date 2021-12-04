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
            row.chars()
                .map(|c| match c {
                    '0' => false,
                    '1' => true,
                    _ => panic!("invalid input"),
                })
                .collect::<BitVec<Msb0>>()
        })
        .collect::<Vec<BitVec<Msb0>>>();
    let nrows = stuff.len();
    let ncols = stuff[0].len();
    dbg!(ncols);

    let mut counts = vec![0; ncols];
    for bits in stuff {
        for (i, bit) in bits.iter().enumerate() {
            if *bit {
                counts[i] += 1;
            }
        }
    }
    // // common
    let mut gamma: BitVec<Msb0> = BitVec::new();
    // // uncommon
    let mut epsilon: BitVec<Msb0> = BitVec::new();
    for v in counts.iter() {
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
