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

    let mut gamma_vecs = stuff.clone();
    let mut epsilon_vecs = stuff.clone();
    for i in 0..stuff[0].len() {
        // if everything doesn't match, break;
        let (gamma, _) = calc_gamma_ep(&gamma_vecs);
        // println!("input: {:?}", gamma_vecs);
        let vecs = gamma_vecs
            .iter()
            .filter(|v| {
                v[i] == match gamma[i] {
                    None => 1,
                    Some(true) => 1,
                    Some(false) => 0,
                }
            })
            .cloned()
            .collect::<Vec<Vec<i32>>>();
        // println!("gamma: {}, {:?}", i, &gamma[i]);
        // // println!("gamma: {:#?}", &gamma);
        // println!("output: {:?}", vecs);
        // println!("\n\n");

        if vecs.len() == 0 {
            break;
        } else {
            gamma_vecs = vecs;
        }
    }
    for i in 0..stuff[0].len() {
        // if everything doesn't match, break;
        let (_, epsilon) = calc_gamma_ep(&epsilon_vecs);
        let vecs = epsilon_vecs
            .iter()
            .filter(|v| {
                v[i] == match epsilon[i] {
                    None => 0,
                    Some(true) => 1,
                    Some(false) => 0,
                }
            })
            .cloned()
            .collect::<Vec<Vec<i32>>>();
        if vecs.len() == 0 {
            break;
        } else {
            epsilon_vecs = vecs;
        }
    }
    let oxygen = gamma_vecs[0]
        .iter()
        .map(|v| match v {
            0 => false,
            1 => true,
            _ => panic!("Asfklj"),
        })
        .collect::<BitVec<Msb0>>();
    let co2 = epsilon_vecs[0]
        .iter()
        .map(|v| match v {
            0 => false,
            1 => true,
            _ => panic!("Asfklj"),
        })
        .collect::<BitVec<Msb0>>();

    let g = oxygen[0..].load::<u32>();
    let e = co2[0..].load::<u32>();
    dbg!(g, e);
    g * e
}

// fn num_common_bits(
//     gamma: &BitVec<Msb0>,
//     bits: &BitVec<Msb0>,
// ) -> u32 {
//     zip(gamma, bits)
//         .map(|(a, b)| if a == b { 1 } else { 0 })
//         .sum()
// }

fn calc_gamma_ep(
    stuff: &[Vec<i32>],
) -> (Vec<Option<bool>>, Vec<Option<bool>>) {
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
    let mut gamma: Vec<Option<bool>> = vec![];
    // // uncommon
    let mut epsilon: Vec<Option<bool>> = vec![];
    for v in totals.iter() {
        let frac = nrows / 2;
        dbg!(v, frac, nrows);
        if nrows % 2 == 0 && (*v as usize) == frac {
            gamma.push(None);
            epsilon.push(None);
        } else if (*v as usize) > frac {
            gamma.push(Some(true));
            epsilon.push(Some(false));
        } else {
            gamma.push(Some(false));
            epsilon.push(Some(true));
        }
    }
    (gamma, epsilon)
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
        assert_eq!(230, process(input));
    }
}
