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

mod small {
    use super::*;

    #[divan::bench]
    fn nom_intro() {
        nom_intro::parse(divan::black_box(&INPUT[0]))
            .unwrap();
    }
    #[divan::bench]
    fn nom_intro_bytes() {
        nom_intro_bytes::parse(divan::black_box(
            &INPUT[0].as_bytes(),
        ))
        .unwrap();
    }
    #[divan::bench]
    fn nom_supreme_intro() {
        nom_supreme_intro::parse(divan::black_box(
            &INPUT[0],
        ))
        .unwrap();
    }
    #[divan::bench]
    fn winnow_intro() {
        let mut input = INPUT[0];
        winnow_intro::parse(divan::black_box(&mut input))
            .unwrap();
    }
    #[divan::bench]
    fn winnow_intro_bytes() {
        winnow_intro_bytes::parse(divan::black_box(
            &mut INPUT[0].as_bytes(),
        ))
        .unwrap();
    }
    #[divan::bench]
    fn regex_intro() {
        let mut input = INPUT[0];
        regex_intro::parse(divan::black_box(&mut input))
            .unwrap();
    }
    #[divan::bench]
    fn pest_intro() {
        let mut input = INPUT[0];
        pest_intro::parse(divan::black_box(&mut input))
            .unwrap();
    }
}
mod big {
    use super::*;
    #[divan::bench]
    fn nom_intro() {
        nom_intro::parse(divan::black_box(&INPUT[1]))
            .unwrap();
    }

    #[divan::bench]
    fn nom_intro_bytes() {
        nom_intro_bytes::parse(divan::black_box(
            &INPUT[1].as_bytes(),
        ))
        .unwrap();
    }

    #[divan::bench]
    fn nom_supreme_intro() {
        nom_supreme_intro::parse(divan::black_box(
            &INPUT[1],
        ))
        .unwrap();
    }

    #[divan::bench]
    fn winnow_intro() {
        let mut input = INPUT[1];
        winnow_intro::parse(divan::black_box(&mut input))
            .unwrap();
    }

    #[divan::bench]
    fn winnow_intro_bytes() {
        winnow_intro_bytes::parse(divan::black_box(
            &mut INPUT[1].as_bytes(),
        ))
        .unwrap();
    }

    #[divan::bench]
    fn regex_intro() {
        let mut input = INPUT[1];
        regex_intro::parse(divan::black_box(&mut input))
            .unwrap();
    }

    #[divan::bench]
    fn pest_intro() {
        let mut input = INPUT[1];
        pest_intro::parse(divan::black_box(&mut input))
            .unwrap();
    }
}
