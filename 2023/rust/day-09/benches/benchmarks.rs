use day_09::*;

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
#[divan::bench]
fn part1_successors() {
    part1_successors::process(divan::black_box(
        include_str!("../input1.txt",),
    ))
    .unwrap();
}
#[divan::bench]
fn part1_one_vec() {
    part1_one_vec::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}
#[divan::bench]
fn part2_successors() {
    part2_successors::process(divan::black_box(
        include_str!("../input1.txt",),
    ))
    .unwrap();
}
#[divan::bench]
fn part2_one_vec() {
    part2_one_vec::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}
