use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_03::process_part1;

const INPUT: &'static str = include_str!("../input.txt");
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day-03:part-1", |b| {
        b.iter(|| process_part1(black_box(INPUT)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
