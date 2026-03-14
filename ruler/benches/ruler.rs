//! Performance benchmarks for the cheap-ruler crate.
//!

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use geo_ruler::CheapRuler;

pub fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("cheap-ruler");

    let origin = [-73.9857, 40.7484];
    let destination = [-73.9897, 40.7411];

    let bearing = 45.;
    let distance = 100.;

    let ruler = CheapRuler::<f32>::WGS84();

    g.bench_with_input("distance", &(origin, destination), |b, (x, y)| {
        b.iter(|| ruler.distance(black_box(x), black_box(y)));
    });

    g.bench_with_input("bearing", &(origin, destination), |b, (x, y)| {
        b.iter(|| ruler.bearing(black_box(x), black_box(y)));
    });

    g.bench_with_input(
        "destination",
        &(origin, bearing, distance),
        |b, (x, a, d)| {
            b.iter(|| ruler.destination(black_box(x), black_box(a), black_box(d)));
        },
    );

    g.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
