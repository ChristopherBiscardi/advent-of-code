use day_06::*;

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
fn part2() {
    part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2_inflight_parse() {
    let (_, (lines, ops)) = part2_inflight::parse(
        divan::black_box(include_bytes!("../input2.txt",)),
    )
    .unwrap();
    for line in lines {
        line.collect::<Vec<_>>();
    }
}

#[divan::bench]
fn part2_inflight() {
    part2_inflight::process(divan::black_box(
        include_str!("../input2.txt",),
    ))
    .unwrap();
}
