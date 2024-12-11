use day_11::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(consts = [
    1,
    10,
    25,
    32,
    // 40 is too high for original implementation bench
    // 40,
])]
fn part1<const N: usize>() {
    part1::process(
        divan::black_box(include_str!("../input1.txt",)),
        N as u64,
    )
    .unwrap();
}

#[divan::bench(consts = [
    1,
    10,
    25,
    32,
    40,
    50,
    60,
    70,
    75,
])]
fn part2<const N: usize>() {
    part2::process(
        divan::black_box(include_str!("../input2.txt",)),
        N as u64,
    )
    .unwrap();
}
