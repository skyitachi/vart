use criterion::{criterion_group, criterion_main, Criterion, black_box};

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 10", |b| b.iter(|| fibonacci(black_box(10))));
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);