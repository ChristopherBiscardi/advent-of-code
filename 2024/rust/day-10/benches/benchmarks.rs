use day_10::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn parser_part1() {
    part1::parse(divan::black_box(part1::Span::new(
        include_str!("../input1.txt",),
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
fn part1_pathfinding() {
    part1_pathfinding::process(divan::black_box(
        include_str!("../input1.txt",),
    ))
    .unwrap();
}

#[divan::bench]
fn part1_pathfinding_rayon() {
    part1_pathfinding_rayon::process(divan::black_box(
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
