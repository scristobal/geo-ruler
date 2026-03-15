use divan::black_box;
use ruler::CheapRuler;

fn main() {
    divan::main();
}

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

mod simd {
    use super::*;

    #[divan::bench]
    fn length(bencher: divan::Bencher) {
        // prime number of data points, not divisible by any number of lanes
        let data = generate_test_data(1019);

        let lats: Vec<f32> = data.iter().map(|p| p[0]).collect();
        let lons: Vec<f32> = data.iter().map(|p| p[1]).collect();

        let points = [&lats[..], &lons[..]];

        bencher.bench_local(|| ruler_simd::length(black_box(&points)));
    }
}

mod iter {
    use super::*;

    #[divan::bench]
    fn length(bencher: divan::Bencher) {
        let data = generate_test_data(1019);
        let points: Vec<&[f32; 2]> = data.iter().collect();
        let ruler = CheapRuler::WGS84();

        bencher.bench_local(|| {
            let points = black_box(&points);
            let mut distance = 0.;

            for i in 1..points.len() {
                distance += ruler.distance(points[i - 1], points[i])
            }

            distance
        });
    }
}
