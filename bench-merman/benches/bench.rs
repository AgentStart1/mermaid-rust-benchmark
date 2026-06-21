use criterion::{criterion_group, criterion_main, Criterion};
use merman::render::HeadlessRenderer;

const SIMPLE: &str = include_str!("../../test-data/simple_flowchart.mmd");
const COMPLEX: &str = include_str!("../../test-data/complex_flowchart.mmd");
const SEQUENCE: &str = include_str!("../../test-data/sequence_diagram.mmd");

fn bench_render(c: &mut Criterion) {
    let renderer = HeadlessRenderer::new();
    let mut g = c.benchmark_group("merman");
    g.bench_function("simple_flowchart", |b| {
        b.iter(|| renderer.render_svg_sync(SIMPLE).unwrap());
    });
    g.bench_function("complex_flowchart", |b| {
        b.iter(|| renderer.render_svg_sync(COMPLEX).unwrap());
    });
    g.bench_function("sequence_diagram", |b| {
        b.iter(|| renderer.render_svg_sync(SEQUENCE).unwrap());
    });
    g.finish();
}

criterion_group!(benches, bench_render);
criterion_main!(benches);
