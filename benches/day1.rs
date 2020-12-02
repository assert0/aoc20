use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc20::days;

static EXAMPLE: [i32; 6] = [1721, 979, 366, 299, 675, 1456];

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("nested find", |b| b.iter(|| 
        days::day1::find2(&EXAMPLE.to_vec(), black_box(2020))));
    c.bench_function("itertools find", |b| b.iter(||
        days::day1::find(&EXAMPLE.to_vec(), black_box(2020), black_box(3))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);