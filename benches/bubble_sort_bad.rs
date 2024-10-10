use algorithms::{bubble_sort::bubble_sort, bubble_sort_bad::bubble_sort_bad};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bubble_sort_good", |b| {
        let mut data = [2, 1, 3, 4, 5];
        b.iter(|| {
            bubble_sort(&mut data);
        });
    });

    c.bench_function("bubble_sort_bad", |b| {
        let mut data = [2, 1, 3, 4, 5];
        b.iter(|| {
            bubble_sort_bad(&mut data);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);