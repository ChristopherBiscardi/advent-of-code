use day_06::*;
use part1::Span;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn parser_part1() {
    part1::parse(divan::black_box(Span::new(
        include_str!("../input1.txt",),
    )))
    .unwrap()
    .0;
}

#[divan::bench]
fn parser_part1_bytes() {
    part1_bytes::parse(divan::black_box(
        part1_bytes::Span::new(include_bytes!(
            "../input1.txt"
        )),
    ))
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
fn part2() {
    part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}
