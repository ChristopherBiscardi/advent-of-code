use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_01::process;

const INPUT: &'static str = include_str!("../input.txt");
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part2", |b| b.iter(|| crate::process(black_box(INPUT))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
