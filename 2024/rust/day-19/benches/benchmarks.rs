use day_19::*;

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
fn part1_nom() {
    part1_nom::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part1_nom_is_ok() {
    part1_nom_is_ok::process(divan::black_box(
        include_str!("../input1.txt",),
    ))
    .unwrap();
}

#[divan::bench]
fn part1_strip_prefix() {
    part1_strip_prefix::process(divan::black_box(
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
