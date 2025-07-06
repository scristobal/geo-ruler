#![feature(iter_map_windows)]

use criterion::{criterion_group, criterion_main, Criterion};
use geo_ruler::CheapRuler;
use simd_ruler;
use std::hint::black_box;

fn generate_test_data(size: usize) -> Vec<[f32; 2]> {
    let ruler = CheapRuler::WGS84();

    let center = [40.7484, -73.9857];
    let distance = 10_000.0;

    let mut bearing = 0.0;
    let delta_bearing = 360. / size as f32;

    let mut points = Vec::with_capacity(size);

    for _ in 0..size {
        bearing += delta_bearing;
        points.push(ruler.destination(&center, &bearing, &distance));
    }

    points
}

pub fn benchmark(c: &mut Criterion) {
    let data = generate_test_data(1_000 * 4 + 1);

    let mut g = c.benchmark_group("simd");

    let lats: Vec<f32> = data.iter().map(|p| p[0]).collect();
    let lons: Vec<f32> = data.iter().map(|p| p[1]).collect();

    let points = [&lats[..], &lons[..]];

    g.bench_with_input("length", &points, |b, points| {
        b.iter(|| simd_ruler::length(black_box(points)))
    });

    g.finish();

    let mut g = c.benchmark_group("iter");

    let points: Vec<&[f32; 2]> = data.iter().collect();

    let ruler = CheapRuler::WGS84();

    g.bench_with_input("length", &points, |b, points| {
        b.iter(|| {
            black_box(points)
                .iter()
                .map_windows(|[p, q]| ruler.distance(p, q))
                .sum::<f32>()
        })
    });

    g.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
