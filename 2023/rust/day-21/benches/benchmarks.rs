use day_21::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(args = [
        6,
        64
    ])]
fn part1(n: usize) {
    part1::process(
        divan::black_box(include_str!("../input1.txt")),
        n,
    )
    .unwrap();
}

#[divan::bench(args = [
    6,
    64
])]
fn part2(n: usize) {
    part2::process(
        divan::black_box(include_str!("../input2.txt",)),
        n,
    )
    .unwrap();
}
