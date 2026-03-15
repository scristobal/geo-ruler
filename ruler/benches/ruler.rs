//! Performance benchmarks for the cheap-ruler crate.
//!

use divan::black_box;
use ruler::CheapRuler;

fn main() {
    divan::main();
}

mod cheap_ruler {
    use super::*;

    #[divan::bench]
    fn distance(bencher: divan::Bencher) {
        let origin = [-73.9857, 40.7484];
        let destination = [-73.9897, 40.7411];
        let ruler = CheapRuler::<f32>::WGS84();
        bencher.bench_local(|| ruler.distance(black_box(&origin), black_box(&destination)));
    }

    #[divan::bench]
    fn bearing(bencher: divan::Bencher) {
        let origin = [-73.9857, 40.7484];
        let destination = [-73.9897, 40.7411];
        let ruler = CheapRuler::<f32>::WGS84();
        bencher.bench_local(|| ruler.bearing(black_box(&origin), black_box(&destination)));
    }

    #[divan::bench]
    fn destination(bencher: divan::Bencher) {
        let origin = [-73.9857, 40.7484];
        let bearing = 45.;
        let distance = 100.;
        let ruler = CheapRuler::<f32>::WGS84();
        bencher.bench_local(|| {
            ruler.destination(black_box(&origin), black_box(&bearing), black_box(&distance))
        });
    }
}
