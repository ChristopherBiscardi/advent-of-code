use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use day_05::{
    process_part1, process_part1_opt2, process_part2,
};

const INPUT: &'static str = include_str!("../input.txt");

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part1", |b| {
        b.iter(|| process_part1(black_box(INPUT)))
    });
    c.bench_function("part1_opt2", |b| {
        b.iter(|| process_part1_opt2(black_box(INPUT)))
    });
    c.bench_function("part2", |b| {
        b.iter(|| process_part2(black_box(INPUT)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
