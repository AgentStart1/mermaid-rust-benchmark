use criterion::{criterion_group, criterion_main, Criterion};
use mermaid_rs_renderer::render;

const SIMPLE: &str = include_str!("../../test-data/simple_flowchart.mmd");
const COMPLEX: &str = include_str!("../../test-data/complex_flowchart.mmd");
const SEQUENCE: &str = include_str!("../../test-data/sequence_diagram.mmd");

fn bench_render(c: &mut Criterion) {
    let mut g = c.benchmark_group("mermaid-rs-renderer");
    g.bench_function("simple_flowchart", |b| {
        b.iter(|| render(SIMPLE).unwrap());
    });
    g.bench_function("complex_flowchart", |b| {
        b.iter(|| render(COMPLEX).unwrap());
    });
    g.bench_function("sequence_diagram", |b| {
        b.iter(|| render(SEQUENCE).unwrap());
    });
    g.finish();
}

criterion_group!(benches, bench_render);
criterion_main!(benches);
