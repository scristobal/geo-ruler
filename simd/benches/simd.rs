use criterion::{Criterion, criterion_group, criterion_main};
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
    // prime number of data points, not divisible by any number of lanes
    let data = generate_test_data(1019);

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
            let points = black_box(points);
            let mut distance = 0.;

            for i in 1..points.len() {
                distance += ruler.distance(points[i - 1], points[i])
            }

            distance
        })
    });

    g.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
