use criterion::{criterion_group, criterion_main, Criterion};

const SIMPLE: &str = include_str!("../../test-data/simple_flowchart.mmd");
const COMPLEX: &str = include_str!("../../test-data/complex_flowchart.mmd");
const SEQUENCE: &str = include_str!("../../test-data/sequence_diagram.mmd");

fn bench_render_svg(c: &mut Criterion) {
    let mut g = c.benchmark_group("selkie");
    g.bench_function("simple_flowchart", |b| {
        b.iter(|| {
            let diagram = selkie::parse(SIMPLE).unwrap();
            selkie::render::render(&diagram).unwrap()
        });
    });
    g.bench_function("complex_flowchart", |b| {
        b.iter(|| {
            let diagram = selkie::parse(COMPLEX).unwrap();
            selkie::render::render(&diagram).unwrap()
        });
    });
    g.bench_function("sequence_diagram", |b| {
        b.iter(|| {
            let diagram = selkie::parse(SEQUENCE).unwrap();
            selkie::render::render(&diagram).unwrap()
        });
    });
    g.finish();
}

criterion_group!(benches, bench_render_svg);
criterion_main!(benches);
