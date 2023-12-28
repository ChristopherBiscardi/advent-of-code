use criterion::{
    criterion_group, criterion_main, BatchSize,
    BenchmarkId, Criterion,
};
use day_02_parsing::*;

const INPUT: [&str; 2] = [
    include_str!("../small-input.txt"),
    include_str!("../big-input.txt"),
];

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsers");
    for (i, input) in INPUT.iter().enumerate() {
        let size = match i {
            0 => "small",
            1 => "large",
            _ => unreachable!(
                "configure name for input size"
            ),
        };
        group.bench_with_input(
            BenchmarkId::new("nom_intro", size),
            input,
            |b, input| b.iter(|| nom_intro::parse(input)),
        );
        group.bench_with_input(
            BenchmarkId::new("nom_intro_bytes", size),
            input,
            |b, input| {
                b.iter(|| {
                    nom_intro_bytes::parse(input.as_bytes())
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("nom_supreme_intro", size),
            input,
            |b, input| {
                b.iter(|| nom_supreme_intro::parse(input))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("pest_intro", size),
            input,
            |b, input| b.iter(|| pest_intro::parse(input)),
        );

        group.bench_with_input(
            BenchmarkId::new("winnow_intro", size),
            input,
            |b, mut input| {
                b.iter_batched(
                    || input.clone(),
                    |mut input| {
                        winnow_intro::parse(&mut input)
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
    c.bench_with_input(
        BenchmarkId::new("regex_intro", "small"),
        &INPUT[0],
        |b, input| b.iter(|| regex_intro::parse(input)),
    );
    c.bench_with_input(
        BenchmarkId::new("regex_intro", "large"),
        &INPUT[1],
        |b, input| b.iter(|| regex_intro::parse(input)),
    );
}

criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);
