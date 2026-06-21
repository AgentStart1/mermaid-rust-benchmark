use criterion::{criterion_group, criterion_main, Criterion};
use rusty_mermaid::{to_svg, Theme};

const SIMPLE: &str = include_str!("../../test-data/simple_flowchart.mmd");
const COMPLEX: &str = include_str!("../../test-data/complex_flowchart.mmd");
const SEQUENCE: &str = include_str!("../../test-data/sequence_diagram.mmd");

fn bench_render(c: &mut Criterion) {
    let theme = Theme::default();
    let mut g = c.benchmark_group("rusty-mermaid");
    g.bench_function("simple_flowchart", |b| {
        b.iter(|| to_svg(SIMPLE, &theme).unwrap());
    });
    g.bench_function("complex_flowchart", |b| {
        b.iter(|| to_svg(COMPLEX, &theme).unwrap());
    });
    g.bench_function("sequence_diagram", |b| {
        b.iter(|| to_svg(SEQUENCE, &theme).unwrap());
    });
    g.finish();
}

criterion_group!(benches, bench_render);
criterion_main!(benches);
