use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

fn bench_placeholder(c: &mut Criterion) {
    c.bench_function("placeholder", |b| {
        b.iter(|| {
            // TODO: add histogram benchmarks
        });
    });
}

criterion_group!(benches, bench_placeholder);
criterion_main!(benches);
