use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_07::{process_part1, process_part2};

const INPUT: &'static str = include_str!("../input.txt");
fn bench_1(c: &mut Criterion) {
    c.bench_function("day-07:part-1", |b| {
        b.iter(|| process_part1(black_box(INPUT)))
    });
}

fn bench_2(c: &mut Criterion) {
    c.bench_function("day-07:part-2", |b| {
        b.iter(|| process_part2(black_box(INPUT)))
    });
}

criterion_group!(benches, bench_1, bench_2);
criterion_main!(benches);
