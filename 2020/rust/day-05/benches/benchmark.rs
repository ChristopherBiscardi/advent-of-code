use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_05::process_part2;

const INPUT: &'static str = include_str!("../input.txt");
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day-04:part-2", |b| {
        b.iter(|| process_part2(black_box(INPUT)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
