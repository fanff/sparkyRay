use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rustyray::{load_scene_name, Scene};
use std::iter;

fn do_render(scene: &Scene, w: usize, depth: u64) {
    scene.render(w, w, 5, -1.0, 1.0, -1.0, 1.0);
}

fn criterion_benchmark(c: &mut Criterion) {
    static w: usize = 19;
    let scene = load_scene_name("scene1.json".to_string());

    let mut group = c.benchmark_group("do_render");

    group.sample_size(30);

    for size in [w, 2 * w, 4 * w, 8 * w, 16 * w].iter() {
        group.throughput(Throughput::Elements((*size as u64).pow(2)));
        group.bench_with_input(
            BenchmarkId::from_parameter((*size as u64)),
            size,
            |b, &size| {
                b.iter(|| do_render(&scene, size, 5));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
