//! Performance benchmarks for the ruler-geo crate.
//!
//! Compares performance against other geodesic calculation
//! methods for common operations at city-scale distances.

use divan::black_box;
use geo::InterpolatePoint;
use geo::{Bearing, Destination, Distance, Geodesic, Haversine, Rhumb, point};
use ruler_geo::RulerMeasure;

fn main() {
    divan::main();
}

macro_rules! bench_group {
    ($mod_name:ident, $measure:expr) => {
        mod $mod_name {
            use super::*;

            #[divan::bench]
            fn distance(bencher: divan::Bencher) {
                let origin = point!(x: -73.9857, y: 40.7484);
                let dest = point!(x: -73.9897, y: 40.7411);
                let measure = $measure;
                bencher.bench(|| {
                    measure.distance(black_box(origin), black_box(dest))
                });
            }

            #[divan::bench]
            fn bearing(bencher: divan::Bencher) {
                let origin = point!(x: -73.9857, y: 40.7484);
                let dest = point!(x: -73.9897, y: 40.7411);
                let measure = $measure;
                bencher.bench(|| {
                    measure.bearing(black_box(origin), black_box(dest))
                });
            }

            #[divan::bench]
            fn destination(bencher: divan::Bencher) {
                let origin = point!(x: -73.9857, y: 40.7484);
                let bearing = 45.;
                let distance = 100.;
                let measure = $measure;
                bencher.bench(|| {
                    measure.destination(
                        black_box(origin),
                        black_box(bearing),
                        black_box(distance),
                    )
                });
            }

            #[divan::bench]
            fn interpolate_distance(bencher: divan::Bencher) {
                let origin = point!(x: -73.9857, y: 40.7484);
                let dest = point!(x: -73.9897, y: 40.7411);
                let distance = 100.;
                let measure = $measure;
                bencher.bench(|| {
                    measure.point_at_distance_between(
                        black_box(origin),
                        black_box(dest),
                        black_box(distance),
                    )
                });
            }

            #[divan::bench]
            fn interpolate_ratio(bencher: divan::Bencher) {
                let origin = point!(x: -73.9857, y: 40.7484);
                let dest = point!(x: -73.9897, y: 40.7411);
                let ratio = 0.25;
                let measure = $measure;
                bencher.bench(|| {
                    measure.point_at_ratio_between(
                        black_box(origin),
                        black_box(dest),
                        black_box(ratio),
                    )
                });
            }

            #[divan::bench]
            fn interpolate_along(bencher: divan::Bencher) {
                let origin = point!(x: -73.9857, y: 40.7484);
                let dest = point!(x: -73.9897, y: 40.7411);
                let max_distance = 100.;
                let include_ends = false;
                let measure = $measure;
                bencher.bench(|| {
                    measure.points_along_line(
                        black_box(origin),
                        black_box(dest),
                        black_box(max_distance),
                        black_box(include_ends),
                    )
                });
            }
        }
    };
}

bench_group!(ruler, &RulerMeasure::<f32>::WGS84());
bench_group!(geodesic, &Geodesic);
bench_group!(haversine, &Haversine);
bench_group!(rhumb, &Rhumb);
