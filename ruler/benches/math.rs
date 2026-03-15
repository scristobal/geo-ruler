//! Performance benchmarks comparing different atan2 implementations.
//!

use divan::black_box;
use ruler::math;

fn main() {
    divan::main();
}

mod atan2_f64 {
    use super::*;

    fn test_data() -> Vec<(f64, f64)> {
        vec![
            (1.0, 1.0),     // 45 degrees
            (0.0, 1.0),     // 90 degrees
            (-1.0, 1.0),    // 135 degrees
            (-1.0, 0.0),    // 180 degrees
            (-1.0, -1.0),   // 225 degrees
            (0.0, -1.0),    // 270 degrees
            (1.0, -1.0),    // 315 degrees
            (1.0, 0.0),     // 0 degrees
            (3.0, 4.0),     // Random values
            (0.5, 0.866),   // 60 degrees
            (0.866, 0.5),   // 30 degrees
            (100.0, 0.1),   // Large x, small y
            (0.1, 100.0),   // Small x, large y
        ]
    }

    #[divan::bench]
    fn default_atan2(bencher: divan::Bencher) {
        let data = test_data();
        bencher.bench_local(|| {
            for &(y, x) in &data {
                black_box(y.atan2(x));
            }
        });
    }

    #[divan::bench]
    fn atan2_deg3(bencher: divan::Bencher) {
        let data = test_data();
        bencher.bench_local(|| {
            for &(y, x) in &data {
                black_box(math::atan2(y, x));
            }
        });
    }
}

mod atan2_f32 {
    use super::*;

    fn test_data() -> Vec<(f32, f32)> {
        vec![
            (1.0, 1.0),     // 45 degrees
            (0.0, 1.0),     // 90 degrees
            (-1.0, 1.0),    // 135 degrees
            (-1.0, 0.0),    // 180 degrees
            (-1.0, -1.0),   // 225 degrees
            (0.0, -1.0),    // 270 degrees
            (1.0, -1.0),    // 315 degrees
            (1.0, 0.0),     // 0 degrees
            (3.0, 4.0),     // Random values
            (0.5, 0.866),   // 60 degrees
            (0.866, 0.5),   // 30 degrees
            (100.0, 0.1),   // Large x, small y
            (0.1, 100.0),   // Small x, large y
        ]
    }

    #[divan::bench]
    fn default_atan2(bencher: divan::Bencher) {
        let data = test_data();
        bencher.bench_local(|| {
            for &(y, x) in &data {
                black_box(y.atan2(x));
            }
        });
    }

    #[divan::bench]
    fn atan2_deg3(bencher: divan::Bencher) {
        let data = test_data();
        bencher.bench_local(|| {
            for &(y, x) in &data {
                black_box(math::atan2(y, x));
            }
        });
    }
}
