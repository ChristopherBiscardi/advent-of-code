use bitvec::prelude::*;
use itertools::Itertools;
fn main() {
    // let input =
    // vec![2,4,1,2,7,5,0,3,4,7,1,7,5,5,3,0];
    // let input = vec![0, 3, 5, 4, 3, 0];
    // // let mut n: i32 = 117440;
    // 'outer: for og_n in 10000..120000 {
    //     let mut n = og_n;
    //     for i in 0..input.len() {
    //         // println!("{n} {}", n%8);
    //         if i != (input.len() - 1) && n == 0 {
    //             continue 'outer;
    //         }
    //         n /= 8;
    //         if i == (input.len() - 1) && n == 0 {
    //             dbg!(og_n);
    //             println!("{n} {}", n % 8);
    //             break 'outer;
    //         }
    //     }
    // }

    // 6
    // 011100101011000000

    let t = (0..3)
        .map(|_| [true, false])
        .multi_cartesian_product()
        .collect::<Vec<_>>();
    dbg!(&t);
    for pattern in t {
        let mut bv = bitvec![];
        for i in pattern {
            bv.push(i);
        }
        let num = bv.as_raw_slice()[0];
        println!("{bv} {}", num);
    }

    let mut bv = bitvec![];
    for i in 0..(16 * 3) {
        bv.push(true);
    }
    let num = bv.as_raw_slice()[0];
    println!("{bv} {}", num);
}
