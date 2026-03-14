//! Performance benchmarks for the geo-ruler crate.
//!
//! Compares performance against other geodesic calculation
//! methods for common operations at city-scale distances.

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use geo::InterpolatePoint;
use geo::{Bearing, Destination, Distance, Geodesic, Haversine, Rhumb, point};
use geo_ruler::geo::RulerMeasure;

macro_rules! bench_distance {
    ($group:ident, $label:expr, $measure:expr, $origin:expr, $destination:expr) => {{
        let measure = $measure;
        $group.bench_with_input(
            "distance",
            &($origin, $destination),
            |b, &(origin, destination)| {
                b.iter(|| measure.distance(black_box(origin), black_box(destination)));
            },
        );
    }};
}

macro_rules! bench_bearing {
    ($group:ident, $label:expr, $measure:expr, $origin:expr, $destination:expr) => {{
        let measure = $measure;
        $group.bench_with_input(
            "bearing",
            &($origin, $destination),
            |b, &(origin, destination)| {
                b.iter(|| measure.bearing(black_box(origin), black_box(destination)));
            },
        );
    }};
}

macro_rules! bench_destination {
    ($group:ident, $label:expr, $measure:expr, $origin:expr, $bearing:expr, $distance:expr) => {{
        let measure = $measure;
        $group.bench_with_input(
            "destination",
            &($origin, $bearing, $distance),
            |b, &(origin, bearing, distance)| {
                b.iter(|| {
                    measure.destination(black_box(origin), black_box(bearing), black_box(distance))
                });
            },
        );
    }};
}

macro_rules! bench_interpolate_distance {
    ($group:ident, $label:expr, $measure:expr, $origin:expr, $destination:expr, $distance:expr) => {{
        let measure = $measure;
        $group.bench_with_input(
            "interpolate_distance",
            &($origin, $destination, $distance),
            |b, &(origin, destination, distance)| {
                b.iter(|| {
                    measure.point_at_distance_between(
                        black_box(origin),
                        black_box(destination),
                        black_box(distance),
                    )
                });
            },
        );
    }};
}

macro_rules! bench_interpolate_ratio {
    ($group:ident, $label:expr, $measure:expr, $origin:expr, $destination:expr, $ratio:expr) => {{
        let measure = $measure;
        $group.bench_with_input(
            "interpolate_ratio",
            &($origin, $destination, $ratio),
            |b, &(origin, destination, ratio)| {
                b.iter(|| {
                    measure.point_at_ratio_between(
                        black_box(origin),
                        black_box(destination),
                        black_box(ratio),
                    )
                });
            },
        );
    }};
}

macro_rules! bench_interpolate_along {
    ($group:ident, $label:expr, $measure:expr, $origin:expr, $destination:expr, $max_distance:expr, $include_ends:expr) => {{
        let measure = $measure;
        $group.bench_with_input(
            "interpolate_along",
            &($origin, $destination, $max_distance, $include_ends),
            |b, &(origin, destination, max_distance, include_ends)| {
                b.iter(|| {
                    measure.points_along_line(
                        black_box(origin),
                        black_box(destination),
                        black_box(max_distance),
                        black_box(include_ends),
                    )
                });
            },
        );
    }};
}

macro_rules! bench_all_ops {
    ($c: ident, $label:expr, $measure:expr, $origin:expr, $destination:expr, $bearing:expr, $distance:expr, $ratio:expr, $include_ends:expr) => {{
        let mut group = $c.benchmark_group($label);

        bench_distance!(group, $label, $measure, $origin, $destination);
        bench_bearing!(group, $label, $measure, $origin, $destination);
        bench_destination!(group, $label, $measure, $origin, $bearing, $distance);
        bench_interpolate_distance!(group, $label, $measure, $origin, $destination, $distance);
        bench_interpolate_ratio!(group, $label, $measure, $origin, $destination, $ratio);
        bench_interpolate_along!(
            group,
            $label,
            $measure,
            $origin,
            $destination,
            $distance,
            $include_ends
        );

        group.finish();
    }};
}

macro_rules! bench_new_york {
    ($c:ident, $label:expr, $measure:expr) => {{
        let empire_state = point!(x: -73.9857, y: 40.7484); // Empire State
        let flatiron = point!(x: -73.9897, y: 40.7411); // Flatiron
        let bearing = 45.;
        let distance = 100.;
        let ratio = 0.25;
        let include_ends = false;

        bench_all_ops!($c, $label, $measure, empire_state, flatiron, bearing, distance, ratio, include_ends);
    }};
}

pub fn benchmark(c: &mut Criterion) {
    let ruler = RulerMeasure::<f32>::WGS84();

    bench_new_york!(c, "ruler", &ruler);
    bench_new_york!(c, "geodesic", &Geodesic);
    bench_new_york!(c, "haversine", &Haversine);
    bench_new_york!(c, "rhumb", &Rhumb);
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
