//! WebAssembly bindings for geo-ruler geodesic calculations.
//!
//! This module provides WebAssembly (WASM) bindings for the geo-ruler library,
//! enabling fast geodesic calculations to be used from JavaScript and other
//! WebAssembly environments.
//!
//! The WASM interface is designed to be simple and efficient, providing a
//! `Coords` struct that represents a geographic coordinate (longitude, latitude)
//! and includes methods for common geodesic operations.
//!
//! # Performance
//!
//! The WASM bindings use the same fast approximation algorithm as the core
//! geo-ruler library, providing 20-100x performance improvements over traditional
//! geodesic calculations while maintaining typically <0.1% error for city-scale
//! distances (up to 500 km).
//!
//! # JavaScript Usage
//!
//! ```javascript
//! import { Coords } from 'geo-ruler';
//!
//! // Create coordinate points (Empire State Building and Flatiron Building)
//! const empireState = new Coords(-73.9857, 40.7484);
//! const flatiron = new Coords(-73.9897, 40.7411);
//!
//! // Calculate distance in meters
//! const distance = empireState.distance(flatiron);
//! console.log(`Distance: ${distance.toFixed(1)} meters`);
//!
//! // Calculate bearing in degrees
//! const bearing = empireState.bearing(flatiron);
//! console.log(`Bearing: ${bearing.toFixed(1)} degrees`);
//!
//! // Find destination point 100m away at 45 degree bearing
//! const destination = empireState.destination(45.0, 100.0);
//! console.log(`Destination: ${destination.x}, ${destination.y}`);
//! ```
//!
//! # Feature Flag
//!
//! This module is available when the `wasm` feature flag is enabled.
//!
//! # Coordinate System
//!
//! All coordinates use the WGS84 datum with longitude/latitude in decimal degrees.
//! The `x` field represents longitude and the `y` field represents latitude.

use crate::CheapRuler;
use wasm_bindgen::prelude::*;

/// A geographic coordinate point with longitude and latitude.
///
/// This struct represents a point on Earth's surface using the WGS84 coordinate system.
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
/// ```javascript
/// // Create a coordinate point for New York City
/// const nyc = new Coords(-74.0059, 40.7128);
///
/// // Create another point for Philadelphia
/// const philly = new Coords(-75.1652, 39.9526);
///
/// // Calculate distance between cities
/// const distance = nyc.distance(philly);
/// ```
#[wasm_bindgen]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
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
    /// ```javascript
    /// // Create a coordinate for the Empire State Building
    /// const empireState = new Coords(-73.9857, 40.7484);
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Coords {
        Coords { x, y }
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
    /// ```javascript
    /// const point1 = new Coords(-73.9857, 40.7484); // Empire State Building
    /// const point2 = new Coords(-73.9897, 40.7411); // Flatiron Building
    /// const distance = point1.distance(point2);
    /// console.log(`Distance: ${distance.toFixed(1)} meters`);
    /// ```
    pub fn distance(&self, destination: &Coords) -> f32 {
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
    /// ```javascript
    /// const start = new Coords(-73.9857, 40.7484);
    /// const end = new Coords(-73.9897, 40.7411);
    /// const bearing = start.bearing(end);
    /// console.log(`Bearing: ${bearing.toFixed(1)} degrees`);
    /// ```
    pub fn bearing(&self, destination: &Coords) -> f32 {
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
    /// ```javascript
    /// const start = new Coords(-73.9857, 40.7484); // Empire State Building
    /// // Travel 1000 meters northeast (45 degrees)
    /// const destination = start.destination(45.0, 1000.0);
    /// console.log(`Destination: ${destination.x}, ${destination.y}`);
    /// ```
    pub fn destination(&self, bearing: f32, distance: f32) -> Coords {
        let [x, y] = CheapRuler::WGS84().destination(&[self.x, self.y], &bearing, &distance);
        Coords { x, y }
    }
}
