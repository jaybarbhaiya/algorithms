use algorithms::bubble_sort::bubble_sort;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bubble_sort", |b| {
        let mut data = [2, 1, 3, 4, 5];
        b.iter(|| {
            bubble_sort(&mut data);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);