//! Performance benchmarks comparing different atan2 implementations.
//!

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use geo_ruler::math;

pub fn benchmark_atan2_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("atan2_f64");

    // Test data - various angle combinations to test different quadrants and edge cases
    let test_data = vec![
        (1.0f64, 1.0f64),   // 45 degrees
        (0.0f64, 1.0f64),   // 90 degrees
        (-1.0f64, 1.0f64),  // 135 degrees
        (-1.0f64, 0.0f64),  // 180 degrees
        (-1.0f64, -1.0f64), // 225 degrees
        (0.0f64, -1.0f64),  // 270 degrees
        (1.0f64, -1.0f64),  // 315 degrees
        (1.0f64, 0.0f64),   // 0 degrees
        (3.0f64, 4.0f64),   // Random values
        (0.5f64, 0.866f64), // 60 degrees
        (0.866f64, 0.5f64), // 30 degrees
        (100.0f64, 0.1f64), // Large x, small y
        (0.1f64, 100.0f64), // Small x, large y
    ];

    group.bench_function("default_atan2", |b| {
        b.iter(|| {
            for (y, x) in &test_data {
                black_box(y.atan2(*x));
            }
        });
    });

    group.bench_function("atan2_deg3", |b| {
        b.iter(|| {
            for (y, x) in &test_data {
                black_box(math::atan2(*y, *x));
            }
        });
    });

    group.finish();
}

pub fn benchmark_atan2_f32(c: &mut Criterion) {
    let mut group = c.benchmark_group("atan2_f32");

    // Test data - various angle combinations to test different quadrants and edge cases
    let test_data_f32 = vec![
        (1.0f32, 1.0f32),   // 45 degrees
        (0.0f32, 1.0f32),   // 90 degrees
        (-1.0f32, 1.0f32),  // 135 degrees
        (-1.0f32, 0.0f32),  // 180 degrees
        (-1.0f32, -1.0f32), // 225 degrees
        (0.0f32, -1.0f32),  // 270 degrees
        (1.0f32, -1.0f32),  // 315 degrees
        (1.0f32, 0.0f32),   // 0 degrees
        (3.0f32, 4.0f32),   // Random values
        (0.5f32, 0.866f32), // 60 degrees
        (0.866f32, 0.5f32), // 30 degrees
        (100.0f32, 0.1f32), // Large x, small y
        (0.1f32, 100.0f32), // Small x, large y
    ];

    // Benchmark Rust's default atan2 with f32
    group.bench_function("default_atan2", |b| {
        b.iter(|| {
            for (y, x) in &test_data_f32 {
                black_box(y.atan2(*x));
            }
        });
    });

    group.bench_function("atan2_deg3", |b| {
        b.iter(|| {
            for (y, x) in &test_data_f32 {
                black_box(math::atan2(*y, *x));
            }
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_atan2_f64, benchmark_atan2_f32);
criterion_main!(benches);
