//! Python bindings for ruler geodesic calculations.
//!
//! This module provides Python bindings for the ruler library,
//! enabling fast geodesic calculations to be used from Python.
//!
//! The Python interface is designed to be simple and efficient, providing a
//! `Coords` class that represents a geographic coordinate (longitude, latitude)
//! and includes methods for common geodesic operations.
//!
//! # Performance
//!
//! The Python bindings use the same fast approximation algorithm as the core
//! ruler library, providing 20-100x performance improvements over traditional
//! geodesic calculations while maintaining typically <0.1% error for city-scale
//! distances (up to 500 km).
//!
//! # Python Usage
//!
//! ```python
//! from geo_ruler import Coords
//!
//! # Create coordinate points (Empire State Building and Flatiron Building)
//! empire_state = Coords(-73.9857, 40.7484)
//! flatiron = Coords(-73.9897, 40.7411)
//!
//! # Calculate distance in meters
//! distance = empire_state.distance(flatiron)
//! print(f"Distance: {distance:.1f} meters")
//!
//! # Calculate bearing in degrees
//! bearing = empire_state.bearing(flatiron)
//! print(f"Bearing: {bearing:.1f} degrees")
//!
//! # Find destination point 100m away at 45 degree bearing
//! destination = empire_state.destination(45.0, 100.0)
//! print(f"Destination: {destination.x}, {destination.y}")
//! ```
//!
//! # Coordinate System
//!
//! All coordinates use the WGS84 datum with longitude/latitude in decimal degrees.
//! The `x` field represents longitude and the `y` field represents latitude.

use ::ruler::CheapRuler;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use pyo3_stub_gen::define_stub_info_gatherer;

/// A geographic coordinate point with longitude and latitude.
///
/// This class represents a point on Earth's surface using the WGS84 coordinate system.
/// It provides methods for calculating distances, bearings, and destinations between
/// coordinate points using the fast Cheap Ruler approximation algorithm.
///
/// # Fields
///
/// - `x`: Longitude in decimal degrees (range: -180 to 180)
/// - `y`: Latitude in decimal degrees (range: -90 to 90)
///
/// # Examples
///
/// ```python
/// # Create a coordinate point for New York City
/// nyc = Coords(-74.0059, 40.7128)
///
/// # Create another point for Philadelphia
/// philly = Coords(-75.1652, 39.9526)
///
/// # Calculate distance between cities
/// distance = nyc.distance(philly)
/// ```
#[gen_stub_pyclass]
#[pyclass(get_all, set_all)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

#[gen_stub_pymethods]
#[pymethods]
impl Coords {
    /// Creates a new coordinate point with the given longitude and latitude.
    ///
    /// # Parameters
    ///
    /// - `x`: Longitude in decimal degrees (range: -180 to 180)
    /// - `y`: Latitude in decimal degrees (range: -90 to 90)
    ///
    /// # Returns
    ///
    /// A new `Coords` instance representing the specified geographic point.
    ///
    /// # Examples
    ///
    /// ```python
    /// # Create a coordinate for the Empire State Building
    /// empire_state = Coords(-73.9857, 40.7484)
    /// ```
    #[new]
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Calculates the distance between this coordinate and another coordinate.
    ///
    /// Uses the Cheap Ruler algorithm to compute an approximate distance between
    /// two points on Earth's surface. The result is optimized for city-scale
    /// distances (up to ~500 km) where speed is more important than absolute precision.
    ///
    /// # Parameters
    ///
    /// - `destination`: The destination coordinate point
    ///
    /// # Returns
    ///
    /// The distance between the two points in meters.
    ///
    /// # Accuracy
    ///
    /// Typical error is <0.1% for distances up to 500 km. For longer distances,
    /// consider using more precise geodesic calculations.
    ///
    /// # Examples
    ///
    /// ```python
    /// point1 = Coords(-73.9857, 40.7484)  # Empire State Building
    /// point2 = Coords(-73.9897, 40.7411)  # Flatiron Building
    /// distance = point1.distance(point2)
    /// print(f"Distance: {distance:.1f} meters")
    /// ```
    fn distance(&self, destination: &Coords) -> f32 {
        CheapRuler::WGS84().distance(&[self.x, self.y], &[destination.x, destination.y])
    }

    /// Calculates the bearing (direction) from this coordinate to another coordinate.
    ///
    /// The bearing is measured clockwise from north (0°) and represents the initial
    /// direction of travel when moving from this point to the destination point.
    ///
    /// # Parameters
    ///
    /// - `destination`: The destination coordinate point
    ///
    /// # Returns
    ///
    /// The bearing in degrees (range: 0 to 360), where:
    /// - 0° = North
    /// - 90° = East
    /// - 180° = South
    /// - 270° = West
    ///
    /// # Examples
    ///
    /// ```python
    /// start = Coords(-73.9857, 40.7484)
    /// end = Coords(-73.9897, 40.7411)
    /// bearing = start.bearing(end)
    /// print(f"Bearing: {bearing:.1f} degrees")
    /// ```
    fn bearing(&self, destination: &Coords) -> f32 {
        CheapRuler::WGS84().bearing(&[self.x, self.y], &[destination.x, destination.y])
    }

    /// Calculates the destination coordinate when traveling from this point
    /// at a given bearing and distance.
    ///
    /// This method computes where you would end up if you started from this
    /// coordinate and traveled in a straight line (great circle) for the
    /// specified distance at the given bearing.
    ///
    /// # Parameters
    ///
    /// - `bearing`: The direction of travel in degrees (0-360), where 0° is north
    /// - `distance`: The distance to travel in meters
    ///
    /// # Returns
    ///
    /// A new `Coords` instance representing the destination point.
    ///
    /// # Examples
    ///
    /// ```python
    /// start = Coords(-73.9857, 40.7484)  # Empire State Building
    /// # Travel 1000 meters northeast (45 degrees)
    /// destination = start.destination(45.0, 1000.0)
    /// print(f"Destination: {destination.x}, {destination.y}")
    /// ```
    fn destination(&self, bearing: f32, distance: f32) -> Coords {
        let [x, y] = CheapRuler::WGS84().destination(&[self.x, self.y], &bearing, &distance);
        Coords { x, y }
    }

    fn __repr__(&self) -> String {
        format!("Coords(x={}, y={})", self.x, self.y)
    }
}

define_stub_info_gatherer!(stub_info);

#[pymodule]
fn geo_ruler(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Coords>()?;
    Ok(())
}
