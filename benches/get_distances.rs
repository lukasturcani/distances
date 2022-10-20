use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use distances::Vector3;

fn benchmark_get_distances(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_distances");
    let sizes = vec![5, 10, 100, 1_000];
    for size in sizes {
        let points = get_points(size);
        group.throughput(Throughput::Elements(size));
        group.bench_with_input(BenchmarkId::from_parameter(size), &points, |b, points| {
            b.iter(|| {
                let mut distances: Vec<f32> = Vec::with_capacity((size * (size - 1) / 2) as usize);
                distances.extend(distances::get_distances(points));
            });
        });
    }
    group.finish();
}

fn get_points(num_points: u64) -> Vec<Vector3<f32>> {
    (0..num_points)
        .map(|i| {
            let i = i as f32;
            Vector3 { x: i, y: i, z: i }
        })
        .collect()
}

criterion_group!(benches, benchmark_get_distances);
criterion_main!(benches);
