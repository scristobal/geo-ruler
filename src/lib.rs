//! A fast, city-scale geodesic approximation library based on Mapbox's Cheap Ruler algorithm.
//!
//! This crate provides the `CheapRuler` struct that implements fast approximations for
//! common geodesic calculations on Earth's surface, designed for city-scale distances
//! where performance is more important than absolute precision.
//!
//! The core algorithm uses a locally flat Earth approximation with latitude-dependent
//! scaling factors, offering 20-100x performance improvements over traditional methods
//! like Haversine or Vincenty's formulas while maintaining typically <0.1% error for
//! distances up to 500 km.
//!
//! # Examples
//!
//! Basic distance calculation:
//!
//! ```
//! use geo_ruler::CheapRuler;
//!
//! let ruler = CheapRuler::<f64>::WGS84();
//! let empire_state = [-73.9857, 40.7484];
//! let flatiron = [-73.9897, 40.7411];
//!
//! let distance = ruler.distance(&empire_state, &flatiron);
//! println!("Distance: {:.1} meters", distance);
//! ```
//!
//! # Feature Flags
//!
//! - `geo`: Integration with the geo-rs crate ecosystem
//! - `wasm`: WebAssembly bindings for JavaScript interop
//! - `atan2_deg3`: Use 3rd degree polynomial approximation for `atan2` (faster)
//! - `atan2_deg5`: Use 5th degree polynomial approximation for `atan2` (more accurate)

#![no_std]

mod constants;

#[cfg(feature = "wasm")]
mod wasm;

#[cfg(any(feature = "atan2_deg3", feature = "atan2_deg5"))]
pub mod math;

#[cfg(feature = "geo")]
pub mod geo;

use constants::{WGS84_E2, WGS84_RE};
use core::convert::From;
use core::fmt::Debug;
use num_traits::{Float, FloatConst};

/// A fast geodesic approximation calculator using latitude-dependent scaling.
///
/// `CheapRuler` provides efficient approximations for common geodesic operations
/// by using a locally flat Earth model with scaling factors computed for a specific
/// latitude. This approach trades some accuracy for significant performance gains.
///
/// The ruler works with any floating-point type that implements the required traits.
pub struct CheapRuler<T: Float> {
    re: T,
    e2: T,
}

impl<T: Float + FloatConst + Debug + From<f32>> CheapRuler<T> {
    /// Creates a new `CheapRuler` using the WGS84 ellipsoid parameters.
    ///
    /// This is the most common constructor for Earth-based calculations.
    #[allow(non_snake_case)]
    pub fn WGS84() -> CheapRuler<T> {
        CheapRuler {
            re: WGS84_RE.into(),
            e2: WGS84_E2.into(),
        }
    }
}

impl<T: Float + FloatConst + Debug> CheapRuler<T> {
    /// Creates a new `CheapRuler` with custom ellipsoid parameters.
    ///
    /// This constructor allows using different ellipsoid models or even
    /// calculations for other celestial bodies.
    ///
    /// # Parameters
    ///
    /// - `major`: Semi-major axis of the ellipsoid (in meters)
    /// - `minor`: Semi-minor axis of the ellipsoid (in meters)
    pub fn new(major: &T, minor: &T) -> Self {
        let e2 = T::one() - (minor.powi(2) / major.powi(2));
        Self { re: *major, e2 }
    }

    /// Calculates the latitude-dependent coefficients for distance calculations.
    ///
    /// This is an internal method that computes scaling factors to convert longitude and
    /// latitude differences into actual distances at a specific latitude.
    ///
    /// # Parameters
    ///
    /// - `origin`: The reference point as `[longitude, latitude]` in degrees
    ///
    /// # Returns
    ///
    /// An array of two scaling factors:
    /// - `kx`: The longitude scaling factor (converts longitude degrees to meters)
    /// - `ky`: The latitude scaling factor (converts latitude degrees to meters)
    fn coefs(&self, origin: &[T; 2]) -> [T; 2] {
        let c = origin[1].to_radians().cos();

        let w = T::one() / (T::one() - self.e2 * (T::one() - c.powi(2)));
        let k = w.sqrt() * self.re.to_radians();

        let kx = k * c;
        let ky = k * w * (T::one() - self.e2);

        [kx, ky]
    }

    /// Calculates the destination point given an origin, bearing, and distance.
    ///
    /// Uses the flat Earth approximation with latitude-dependent scaling to compute
    /// the destination coordinates efficiently.
    ///
    /// # Parameters
    ///
    /// - `origin`: Starting point as `[longitude, latitude]` in degrees
    /// - `bearing`: Direction of travel in degrees (0° = North, 90° = East)
    /// - `distance`: Distance to travel in meters
    ///
    /// # Returns
    ///
    /// Destination point as `[longitude, latitude]` in degrees
    pub fn destination(&self, origin: &[T; 2], bearing: &T, distance: &T) -> [T; 2] {
        let [kx, ky] = self.coefs(origin);

        let (sin, cos) = bearing.to_radians().sin_cos();

        let x = origin[0] + *distance * sin / kx;
        let y = origin[1] + *distance * cos / ky;

        [x, y]
    }

    /// Calculates the distance between two points.
    ///
    /// Uses the flat Earth approximation with latitude-dependent scaling computed
    /// at the origin point for efficient distance calculation.
    ///
    /// # Parameters
    ///
    /// - `origin`: First point as `[longitude, latitude]` in degrees
    /// - `destination`: Second point as `[longitude, latitude]` in degrees
    ///
    /// # Returns
    ///
    /// Distance between the points in meters
    pub fn distance(&self, origin: &[T; 2], destination: &[T; 2]) -> T {
        let [kx, ky] = self.coefs(origin);

        let dx = (destination[0] - origin[0]) * kx;
        let dy = (destination[1] - origin[1]) * ky;

        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

/// Trait for types that may implement `From<f32>` depending on feature flags.
///
/// This trait is used to conditionally require `From<f32>` when using alternative
/// `atan2` implementations that need to work with f32 constants.
#[cfg(any(feature = "atan2_deg5", feature = "atan2_deg3"))]
pub trait MaybeFromf32: From<f32> {}

/// Trait for types that may implement `From<f32>` depending on feature flags.
#[cfg(not(any(feature = "atan2_deg5", feature = "atan2_deg3")))]
pub trait MaybeFromf32 {}

#[cfg(any(feature = "atan2_deg5", feature = "atan2_deg3"))]
impl<T: From<f32>> MaybeFromf32 for T {}
#[cfg(not(any(feature = "atan2_deg5", feature = "atan2_deg3")))]
impl<T> MaybeFromf32 for T {}

impl<T: Float + FloatConst + Debug + MaybeFromf32> CheapRuler<T> {
    /// Calculates the bearing from one point to another.
    ///
    /// Uses the flat Earth approximation with latitude-dependent scaling to compute
    /// the initial bearing (forward azimuth) from the origin to the destination.
    ///
    /// # Parameters
    ///
    /// - `origin`: Starting point as `[longitude, latitude]` in degrees
    /// - `destination`: Target point as `[longitude, latitude]` in degrees
    ///
    /// # Returns
    ///
    /// Initial bearing in degrees (0° = North, 90° = East)
    pub fn bearing(&self, origin: &[T; 2], destination: &[T; 2]) -> T {
        let [kx, ky] = self.coefs(origin);

        let dx = (destination[0] - origin[0]) * kx;
        let dy = (destination[1] - origin[1]) * ky;

        #[cfg(not(any(feature = "atan2_deg3", feature = "atan2_deg5")))]
        return dx.atan2(dy).to_degrees();

        #[cfg(any(feature = "atan2_deg5", feature = "atan2_deg3"))]
        return math::atan2(dx, dy).to_degrees();
    }
}
