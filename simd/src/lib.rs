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
use std::simd::{LaneCount, StdFloat as _, SupportedLaneCount, prelude::*};

const RE: f32 = 6_378_137f32.to_radians();
const E2: f32 = 0.006_694_38;

#[inline(always)]
fn coefs<const N: usize>(lat: &Simd<f32, N>) -> [Simd<f32, N>; 2]
where
    LaneCount<N>: SupportedLaneCount,
{
    let c = cos(lat.to_radians());

    let w = Simd::<f32, N>::splat(1.)
        / (Simd::<f32, N>::splat(1.)
            - Simd::<f32, N>::splat(E2) * (Simd::<f32, N>::splat(1.) - c * c));
    let k = w.sqrt() * Simd::<f32, N>::splat(RE);

    let kx = k * c;
    let ky = k * w * (Simd::<f32, N>::splat(1.) - Simd::<f32, N>::splat(E2));

    [kx, ky]
}

#[inline(always)]
unsafe fn read<const N: usize>(s: &[f32], offset: usize) -> Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[cfg(target_arch = "aarch64")]
    return unsafe { std::ptr::read(s.as_ptr().add(offset) as *const Simd<f32, N>) };

    #[cfg(not(target_arch = "aarch64"))]
    return load(s, offset);
}

#[inline(always)]
fn load<const N: usize>(s: &[f32], offset: usize) -> Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    Simd::<f32, N>::load_or_default(&s[offset..])
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
/// let distance = length::<4>(&points);
/// ```
pub fn length<const N: usize>(points: &[&[f32]; 2]) -> f32
where
    LaneCount<N>: SupportedLaneCount,
{
    assert_eq!(points[0].len(), points[1].len());

    let n = points[0].len();

    if n < 2 {
        return 0.0;
    }

    let mut total_length = 0.;

    let num_chunks = (n - 1) / N;

    for offset in (0..num_chunks).step_by(N) {
        let origins = unsafe { [read(points[0], offset), read(points[1], offset)] };
        let destinations = unsafe { [read(points[0], 1 + offset), read(points[1], 1 + offset)] };

        total_length += distance(&origins, &destinations).reduce_sum();
    }

    let remaining_pairs = (n - 1) % N;

    if remaining_pairs > 0 {
        let offset = num_chunks * N;
        let origins = [load(&points[0], offset), load(&points[1], offset)];
        let destinations = [load(&points[0], offset + 1), load(&points[1], offset + 1)];

        let mask = Mask::<i32, N>::from_bitmask((1 << remaining_pairs) - 1);

        total_length += mask
            .select(
                distance(&origins, &destinations),
                Simd::<f32, N>::splat(0.0),
            )
            .reduce_sum();
    }

    total_length
}

#[inline(always)]
fn distance<const N: usize>(
    origin: &[Simd<f32, N>; 2],
    destination: &[Simd<f32, N>; 2],
) -> Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    let [kx, ky] = coefs(&origin[1]);

    let dx = (destination[0] - origin[0]) * kx;
    let dy = (destination[1] - origin[1]) * ky;

    ((dx * dx) + (dy * dy)).sqrt()
}

#[allow(dead_code)] // not public API yet
#[inline(always)]
fn destination<const N: usize>(
    origin: &[Simd<f32, N>; 2],
    bearing: &f32,
    distance: &f32,
) -> [Simd<f32, N>; 2]
where
    LaneCount<N>: SupportedLaneCount,
{
    let [kx, ky] = coefs(&origin[1]);

    let distance = Simd::<f32, N>::splat(*distance);

    let (sin, cos) = bearing.to_radians().sin_cos();

    let x = origin[0] + distance * Simd::<f32, N>::splat(sin) / kx;
    let y = origin[1] + distance * Simd::<f32, N>::splat(cos) / ky;

    [x, y]
}

#[allow(dead_code)] // not public API yet
#[inline(always)]
fn bearing<const N: usize>(
    origin: &[Simd<f32, N>; 2],
    destination: &[Simd<f32, N>; 2],
) -> Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    let [kx, ky] = coefs(&origin[1]);

    let dx = (destination[0] - origin[0]) * kx;
    let dy = (destination[1] - origin[1]) * ky;

    atan2(dy, dx).to_degrees()
}

#[inline(always)]
fn atan2<const N: usize>(y: Simd<f32, N>, x: Simd<f32, N>) -> Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    let a1 = Simd::<f32, N>::splat(-0.9817f32);
    let a3 = Simd::<f32, N>::splat(0.1963f32);

    let abs_y = y.abs();

    let res = Simd::<f32, N>::splat(FRAC_PI_4);
    let r = (x - abs_y) / (x + abs_y);

    // if x < Simd::<f32, N>::splat(0.) {
    //     res += Simd::<f32, N>::splat(FRAC_PI_2);
    //     r = Simd::<f32, N>::splat(-1.) / r;
    // };
    let mask = x.simd_lt(Simd::<f32, N>::splat(0.));
    let mut res = mask.select(res + Simd::<f32, N>::splat(FRAC_PI_2), res);
    let r = mask.select(Simd::<f32, N>::splat(-1.) / r, r);

    res += r * (a1 + a3 * r * r);

    // if y < Simd::<f32, N>::splat(0.) {
    //     res = -res;
    // }
    let mask = y.simd_lt(Simd::<f32, N>::splat(0.));
    mask.select(-res, res)
}

#[inline(always)]
fn cos<const N: usize>(mut x: Simd<f32, N>) -> Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    // reduce to [0, 2π) using periodicity

    // if x < Simd::<f32, N>::splat(0.) {
    //     x = x + Simd::<f32, N>::splat(2. * PI);
    // }
    let mut mask = x.simd_lt(Simd::<f32, N>::splat(0.));
    x = mask.select(x + Simd::<f32, N>::splat(2. * PI), x);

    // reduce to [0, π/2] using symmetry

    //  let mut sign = Simd::<f32, N>::splat(1.);
    //
    //  if x > Simd::<f32, N>::splat(PI) {
    //      x = x - Simd::<f32, N>::splat(PI);
    //      sign = -sign;
    //  }
    mask = x.simd_gt(Simd::<f32, N>::splat(PI));
    x = mask.select(x - Simd::<f32, N>::splat(PI), x);
    let mut sign = mask.select(Simd::<f32, N>::splat(1.), Simd::<f32, N>::splat(-1.));

    // if x > Simd::<f32, N>::splat(FRAC_PI_2) {
    //     x = Simd::<f32, N>::splat(PI) - x;
    //     sign = -sign;
    // }
    mask = x.simd_gt(Simd::<f32, N>::splat(FRAC_PI_2));
    x = mask.select(Simd::<f32, N>::splat(PI) - x, x);
    sign = mask.select(-sign, sign);

    // 4th degree Chebyshev approximation polynomial approximation for cos(x) on [0, π/2]
    let a0 = Simd::<f32, N>::splat(1.);
    let a2 = Simd::<f32, N>::splat(-0.4999999);
    let a4 = Simd::<f32, N>::splat(0.04166368);

    // Horner method for polynomial evaluation
    let x_sq = x * x;
    sign * (a0 + x_sq * (a2 + x_sq * a4))
}
