use day_02::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn parser() {
    part1::ranges(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part1_ilog() {
    part1_ilog::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part1_ilog_rayon() {
    part1_ilog_rayon::process(divan::black_box(
        include_str!("../input1.txt",),
    ))
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}
