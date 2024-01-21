use day_11::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench(args = [
    2,
    10,
    100,
    1000,
    100000,
    1000000
])]
fn part2(n: i64) {
    part2::process(
        divan::black_box(include_str!("../input2.txt")),
        n,
    )
    .unwrap();
}
