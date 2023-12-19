use day_02_parsing::*;
// use divan::AllocProfiler;

// #[global_allocator]
// static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    // Run registered benchmarks.
    divan::main();
}

const INPUT: [&str; 2] = [
    include_str!("../small-input.txt"),
    include_str!("../big-input.txt"),
];

#[divan::bench]
fn small_nom_intro() {
    nom_intro::parse(divan::black_box(&INPUT[0])).unwrap();
}
#[divan::bench]
fn big_nom_intro() {
    nom_intro::parse(divan::black_box(&INPUT[1])).unwrap();
}

#[divan::bench]
fn small_nom_intro_bytes() {
    nom_intro_bytes::parse(divan::black_box(
        &INPUT[0].as_bytes(),
    ))
    .unwrap();
}
#[divan::bench]
fn big_nom_intro_bytes() {
    nom_intro_bytes::parse(divan::black_box(
        &INPUT[1].as_bytes(),
    ))
    .unwrap();
}

#[divan::bench]
fn small_nom_supreme_intro() {
    nom_supreme_intro::parse(divan::black_box(&INPUT[0]))
        .unwrap();
}
#[divan::bench]
fn big_nom_supreme_intro() {
    nom_supreme_intro::parse(divan::black_box(&INPUT[1]))
        .unwrap();
}

#[divan::bench]
fn small_winnow_intro() {
    winnow_intro::parse(divan::black_box(&INPUT[0]))
        .unwrap();
}
#[divan::bench]
fn big_winnow_intro() {
    winnow_intro::parse(divan::black_box(&INPUT[1]))
        .unwrap();
}
