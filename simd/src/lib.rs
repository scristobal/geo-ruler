//! # geo-ruler-simd
//!
//! High-performance, limited accuracy, SIMD-accelerated geographic calculations.
//!
//! Provides vectorized implementations of geospatial operations using Rust's portable SIMD,
//! processing multiple coordinate pairs simultaneously with an ellipsoidal Earth model.
//!
//! Requires nightly Rust due to unstable `portable_simd` feature.

#![feature(portable_simd)]

use core::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};
use std::simd::{StdFloat as _, prelude::*};

const RE: f32 = 6_378_137f32.to_radians();
const E2: f32 = 0.006_694_38;

#[inline(always)]
fn coefs(lat: &f32x4) -> [f32x4; 2] {
    let c = cos(lat.to_radians());

    let w = f32x4::splat(1.) / (f32x4::splat(1.) - f32x4::splat(E2) * (f32x4::splat(1.) - c * c));
    let k = w.sqrt() * f32x4::splat(RE);

    let kx = k * c;
    let ky = k * w * (f32x4::splat(1.) - f32x4::splat(E2));

    [kx, ky]
}

#[inline(always)]
unsafe fn read(s: &[f32], offset: usize) -> f32x4 {
    unsafe { std::ptr::read(s.as_ptr().add(offset) as *const f32x4) }
}

#[inline(always)]
fn load(s: &[f32], offset: usize) -> f32x4 {
    f32x4::load_or_default(&s[offset..])
}

/// Calculates the total length of a polyline using SIMD vectorization.
///
/// Processes multiple coordinate pairs simultaneously with an ellipsoidal Earth model using
/// Rust's portable SIMD, which requires the `portable_simd` feature flag and hence a nightly Rust compiler.
///
/// Coordinates are expected in decimal degrees `[longitude_array, latitude_array]`.
///
/// Returns the total length in meters.
///
/// ```rust
/// # use simd_ruler::length;
/// let lons = [-73.9857, -73.9897, -73.9927];
/// let lats = [40.7484, 40.7411, 40.7394];
///
/// let points = [&lons[..], &lats[..]];
/// let distance = length(&points);
/// ```
pub fn length(points: &[&[f32]; 2]) -> f32 {
    let n = points[0].len();

    if n < 2 {
        return 0.0;
    }

    let mut total_length = 0.;

    let num_chunks = (n - 1) / 4;

    for offset in (0..num_chunks).step_by(4) {
        let origins = unsafe { [read(points[0], offset), read(points[1], offset)] };
        let destinations = unsafe { [read(points[0], 1 + offset), read(points[1], 1 + offset)] };

        total_length += distance(&origins, &destinations).reduce_sum();
    }

    let remaining_pairs = (n - 1) % 4;

    if remaining_pairs > 0 {
        let offset = num_chunks * 4;
        let origins = [load(&points[0], offset), load(&points[1], offset)];
        let destinations = [load(&points[0], offset + 1), load(&points[1], offset + 1)];

        let mask = mask32x4::from_bitmask((1 << remaining_pairs) - 1);

        total_length += mask
            .select(distance(&origins, &destinations), f32x4::splat(0.0))
            .reduce_sum();
    }

    total_length
}

// TODO: fn length<const N: usize>(points: &[&[f32,N]; 2]) -> f32 {}

#[inline(always)]
fn distance(origin: &[f32x4; 2], destination: &[f32x4; 2]) -> f32x4 {
    let [kx, ky] = coefs(&origin[1]);

    let dx = (destination[0] - origin[0]) * kx;
    let dy = (destination[1] - origin[1]) * ky;

    ((dx * dx) + (dy * dy)).sqrt()
}

#[allow(dead_code)] // not public API yet
#[inline(always)]
fn destination(origin: &[f32x4; 2], bearing: &f32, distance: &f32) -> [f32x4; 2] {
    let [kx, ky] = coefs(&origin[1]);

    let distance = f32x4::splat(*distance);

    let (sin, cos) = bearing.to_radians().sin_cos();

    let x = origin[0] + distance * f32x4::splat(sin) / kx;
    let y = origin[1] + distance * f32x4::splat(cos) / ky;

    [x, y]
}

#[allow(dead_code)] // not public API yet
#[inline(always)]
fn bearing(origin: &[f32x4; 2], destination: &[f32x4; 2]) -> f32x4 {
    let [kx, ky] = coefs(&origin[1]);

    let dx = (destination[0] - origin[0]) * kx;
    let dy = (destination[1] - origin[1]) * ky;

    atan2(dy, dx).to_degrees()
}

#[inline(always)]
fn atan2(y: f32x4, x: f32x4) -> f32x4 {
    let a1 = f32x4::splat(-0.9817f32);
    let a3 = f32x4::splat(0.1963f32);

    let abs_y = y.abs();

    let mut res = f32x4::splat(FRAC_PI_4);
    let mut r = (x - abs_y) / (x + abs_y);

    if x < f32x4::splat(0.) {
        res += f32x4::splat(FRAC_PI_2);
        r = f32x4::splat(-1.) / r;
    };

    res += r * (a1 + a3 * r * r);

    if y < f32x4::splat(0.) {
        res = -res;
    }

    res
}

#[inline(always)]
fn cos(mut x: f32x4) -> f32x4 {
    // reduce to [0, 2π) using periodicity
    if x < f32x4::splat(0.) {
        x = x + f32x4::splat(2. * PI);
    }

    // reduce to [0, π/2] using symmetry
    let mut sign = f32x4::splat(1.);

    if x > f32x4::splat(PI) {
        x = x - f32x4::splat(PI);
        sign = -sign;
    }

    if x > f32x4::splat(FRAC_PI_2) {
        x = f32x4::splat(PI) - x;
        sign = -sign;
    }

    // 4th degree polynomial approximation for cos(x) on [0, π/2]
    // Coefficients optimized for minimal error using Chebyshev approximation
    let a0 = f32x4::splat(1.);
    let a2 = f32x4::splat(-0.4999999);
    let a4 = f32x4::splat(0.04166368);

    let x_sq = x * x;
    sign * (a0 + x_sq * (a2 + x_sq * a4))
}
