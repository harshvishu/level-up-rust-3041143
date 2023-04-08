extern crate criterion;
extern crate hello_rust_learners;

use criterion::{criterion_group, criterion_main, Criterion};
use hello_rust_learners::unique;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut list: Vec<f32> = vec![1.0, 2.0, 3.0, 3.0, 4.0, 4.0, 5.0, 5.0, 6.0];
    c.bench_function("unique", |b| b.iter(|| unique(&mut list)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
