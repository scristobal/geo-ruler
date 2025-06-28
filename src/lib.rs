//! This module provides an implementation of the Cheap Ruler measure model
//!
//! The `Ruler` implements a fast approximation for distance, bearing, and point
//! interpolation calculations on the Earth's surface, designed for city-scale
//! distances (up to a few hundred miles) where performance is more important than
//! absolute precision.
//!
//! Based on [Mapbox's Cheap Ruler algorithm](https://blog.mapbox.com/fast-geodesic-approximations-with-cheap-ruler-106f229ad016),
//! this implementation offers:
//!
//! - Speed: Significantly faster than Haversine and Geodesic calculations (often 20-100x)
//! - Reasonable accuracy: For distances under a few hundred miles, the error is typically <0.5%
//! - Simple calculations: Uses a flat-Earth approximation with latitude-dependent scaling
//!
//! # Examples
//!
//! ```
//! use geo::{point, Bearing, Distance, Destination};
//! use geo_ruler::Ruler;
//!
//! // Create two points (Empire State Building and Flatiron Building)
//! let empire_state = point!(x: -73.9857, y: 40.7484);
//! let flatiron = point!(x: -73.9897, y: 40.7411);
//!
//! // Calculate distance using the WGS84 ruler (returns meters)
//! let distance = Ruler::WGS84.distance(empire_state, flatiron);
//!
//! // Calculate bearing (in degrees)
//! let bearing = Ruler::WGS84.bearing(empire_state, flatiron);
//!
//! // Find destination point 100m from Empire State Building at 45 degree bearing
//! let destination = Ruler::WGS84.destination(empire_state, 45.0, 100.0);
//! ```
//!
//! # Feature Flags
//!
//! This crate provides optional feature flags to use alternative `atan2` implementations:
//!
//! - `atan2_deg3`: Use a 3rd degree polynomial approximation (faster but less accurate)
//! - `atan2_deg5`: Use a 5th degree polynomial approximation (better accuracy with slight performance cost)
//!
//! Without any features, Rust's default `atan2` implementation is used.
#![no_std]

use core::marker::PhantomData;
use geo::{Bearing, CoordFloat, Destination, Distance, InterpolatePoint, Point, point};
use num_traits::FloatConst;

#[cfg(any(feature = "atan2_deg3", feature = "atan2_deg5"))]
mod atan;

/// A fast approximation algorithm for geodesic calculations on Earth's surface.
///
/// `Ruler<F>` implements distance, bearing, and point interpolation calculations
/// using a flat-Earth approximation with latitude-dependent scaling.
///
/// It is designed for city-scale distances (up to a few hundred miles) where performance
/// is more important than absolute precision.
///
/// Based on [Mapbox's Cheap Ruler](https://blog.mapbox.com/fast-geodesic-approximations-with-cheap-ruler-106f229ad016).
pub struct Ruler<F> {
    re: f64,
    e2: f64,
    float: PhantomData<F>,
}

impl<F> Ruler<F> {
    /// The standard WGS84 ellipsoid parameters for Earth measurements.
    ///
    /// This constant provides a pre-configured `Ruler` using the WGS84 reference ellipsoid,
    /// which is the standard used in GPS and most modern mapping applications.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo::{point, Distance};
    /// use geo_ruler::Ruler;
    ///
    /// let empire_state = point!(x: -73.9857, y: 40.7484);
    /// let flatiron = point!(x: -73.9897, y: 40.7411);
    ///
    /// // Using the WGS84 constant
    /// let distance = Ruler::WGS84.distance(empire_state, flatiron);
    /// ```
    ///
    /// See also: [`new`](#method.new)
    pub const WGS84: Self = Self {
        re: 6_378_137.,    // WGS84 semimajor axis in meters
        e2: 0.00669437999, // WGS84 eccentricity squared
        float: PhantomData,
    };

    /// Creates a new `Ruler` with custom ellipsoid parameters.
    ///
    /// This method allows you to create a ruler for measuring distances on planets
    /// or celestial bodies other than Earth, or when using a different Earth model.
    ///
    /// # Parameters
    ///
    /// - `major`: The semi-major axis (equatorial radius) of the ellipsoid in meters
    /// - `minor`: The semi-minor axis (polar radius) of the ellipsoid in meters
    ///
    /// # Examples
    ///
    /// ```
    /// use geo::{point, Distance};
    /// use geo_ruler::Ruler;
    ///
    /// // Mars has different dimensions than Earth
    /// let mars_equatorial_radius = 3_396_200.0; // meters
    /// let mars_polar_radius = 3_376_200.0; // meters
    ///
    /// let mars_ruler = Ruler::<f64>::new(mars_equatorial_radius, mars_polar_radius);
    ///
    /// let olympus_mons = point!(x: -226.2, y: 18.65);
    /// let karzok_crater = point!(x: -226.192, y: 18.2292);
    ///
    /// let distance = mars_ruler.distance(olympus_mons, karzok_crater); // distance in meters
    /// ```
    pub fn new(major: f64, minor: f64) -> Self {
        let re = major;
        let e2 = 1. - (minor.powi(2) / major.powi(2));

        Self {
            re,
            e2,
            float: PhantomData,
        }
    }
}

impl<F> Default for Ruler<F> {
    fn default() -> Self {
        Ruler::WGS84
    }
}

impl<F: CoordFloat> Ruler<F> {
    /// Calculates the latitude-dependent coefficients for distance calculations.
    ///
    /// This is an internal method that computes scaling factors to convert longitude and
    /// latitude differences into actual distances at a specific latitude.
    ///
    /// # Parameters
    ///
    /// - `lat`: The latitude at which to compute the coefficients (in degrees)
    ///
    /// # Returns
    ///
    /// An array of two scaling factors:
    /// - `kx`: The longitude scaling factor (converts longitude degrees to meters)
    /// - `ky`: The latitude scaling factor (converts latitude degrees to meters)
    fn coefs(&self, lat: F) -> [F; 2] {
        let re = F::from(self.re).unwrap();
        let e2 = F::from(self.e2).unwrap();

        let clat = F::from(lat.to_radians().cos()).unwrap();

        let w = F::one() / (F::one() - e2 * (F::one() - clat * clat));
        let k = w.sqrt() * re.to_radians();

        let kx = k * clat;
        let ky = k * w * (F::one() - e2);

        [kx, ky]
    }

    /// Calculates the coefficients at the average latitude between two points.
    ///
    /// This is more efficient than calling `coefs()` repeatedly and provides
    /// a good approximation for distance calculations between two points.
    ///
    /// # Parameters
    ///
    /// - `origin`: The latitude of the starting point (in degrees)
    /// - `destination`: The latitude of the ending point (in degrees)
    ///
    /// # Returns
    ///
    /// An array of two scaling factors as described in `coefs()`.
    fn coefs_avg(&self, origin: F, destination: F) -> [F; 2] {
        self.coefs((origin + destination) / (F::one() + F::one()))
    }
}

impl<F: CoordFloat> Destination<F> for Ruler<F> {
    /// Returns a new point having traveled the given distance along the given bearing from origin.
    ///
    /// This uses the Cheap Ruler approximation which is fast but best suited for
    /// city-scale distances (up to a few hundred miles).
    ///
    /// # Parameters
    ///
    /// - `origin`: Starting point with coordinates in degrees (longitude, latitude)
    /// - `bearing`: Direction to travel in degrees (0° = North, 90° = East, etc.)
    /// - `distance`: Distance to travel in meters
    ///
    /// # Returns
    ///
    /// A new Point with coordinates in degrees (longitude, latitude)
    ///
    /// # Examples
    ///
    /// ```
    /// use geo::{point, Destination};
    /// use geo_ruler::Ruler;
    ///
    /// let empire_state = point!(x: -73.9857, y: 40.7484); // Empire State Building
    /// let distance = 100.0; // 100 meters
    /// let bearing = 45.0; // Northeast
    ///
    /// // Find a point 100m northeast of Empire State Building
    /// let destination = Ruler::WGS84.destination(empire_state, bearing, distance);
    /// ```
    ///
    /// See also: [`bearing`](#method.bearing)
    fn destination(&self, origin: Point<F>, bearing: F, distance: F) -> Point<F> {
        let [kx, ky] = self.coefs(origin.y());

        let (sin, cos) = bearing.to_radians().sin_cos();

        let x = origin.x() + distance * sin / kx;
        let y = origin.y() + distance * cos / ky;

        point! { x: x, y: y}
    }
}

impl<F: CoordFloat> Distance<F, Point<F>, Point<F>> for Ruler<F> {
    /// Calculates the distance between two points using the Cheap Ruler approximation.
    ///
    /// This method is optimized for performance and is best suited for distances
    /// up to a few hundred miles where absolute precision is less important than speed.
    ///
    /// # Parameters
    ///
    /// - `origin`: Starting point with coordinates in degrees (longitude, latitude)
    /// - `destination`: Ending point with coordinates in degrees (longitude, latitude)
    ///
    /// # Returns
    ///
    /// The distance between the points in meters.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo::{point, Distance};
    /// use geo_ruler::Ruler;
    ///
    /// let empire_state = point!(x: -73.9857, y: 40.7484); // Empire State Building
    /// let flatiron = point!(x: -73.9897, y: 40.7411);  // Flatiron Building
    ///
    /// // Calculate the distance between Empire State and Flatiron buildings
    /// let distance = Ruler::WGS84.distance(empire_state, flatiron); // Result in meters
    /// ```
    ///
    /// See also: [`bearing`](#method.bearing), [`destination`](#method.destination)
    fn distance(&self, origin: Point<F>, destination: Point<F>) -> F {
        let [kx, ky] = self.coefs_avg(origin.y(), destination.y());

        let dx = destination.x() * kx - origin.x() * kx;
        let dy = destination.y() * ky - origin.y() * ky;

        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

impl<F: CoordFloat + FloatConst> Bearing<F> for Ruler<F> {
    /// Calculates the bearing from one point to another using the Cheap Ruler approximation.
    ///
    /// Bearing represents the direction from the origin point to the destination point,
    /// measured in degrees clockwise from north.
    ///
    /// # Parameters
    ///
    /// - `origin`: Starting point with coordinates in degrees (longitude, latitude)
    /// - `destination`: Target point with coordinates in degrees (longitude, latitude)
    ///
    /// # Returns
    ///
    /// The bearing in degrees, where:
    /// - 0° = North
    /// - 90° = East
    /// - 180° = South
    /// - 270° = West
    ///
    /// # Examples
    ///
    /// ```
    /// use geo::{point, Bearing};
    /// use geo_ruler::Ruler;
    ///
    /// let empire_state = point!(x: -73.9857, y: 40.7484); // Empire State Building
    /// let flatiron = point!(x: -73.9897, y: 40.7411);  // Flatiron Building
    ///
    /// // Calculate the bearing from Empire State to Flatiron
    /// let bearing = Ruler::WGS84.bearing(empire_state, flatiron); // Result in degrees
    /// ```
    ///
    /// See also: [`distance`](#method.distance), [`destination`](#method.destination)
    ///
    /// # Feature Flags
    ///
    /// The implementation used depends on the feature flags enabled:
    /// - With `atan2_deg3` or `atan2_deg5`: Uses the optimized implementation from the `atan` module
    /// - Without features: Uses Rust's default `atan2` implementation
    fn bearing(&self, origin: Point<F>, destination: Point<F>) -> F {
        let [kx, ky] = self.coefs_avg(origin.y(), destination.y());

        let dx = (destination.x() - origin.x()) * kx;
        let dy = (destination.y() - origin.y()) * ky;

        #[cfg(any(feature = "atan2_deg5", feature = "atan2_deg3"))]
        {
            atan::atan2(dx, dy).to_degrees()
        }

        #[cfg(not(any(feature = "atan2_deg3", feature = "atan2_deg5")))]
        {
            dx.atan2(dy).to_degrees()
        }
    }
}

impl<F: CoordFloat + FloatConst> InterpolatePoint<F> for Ruler<F> {
    /// Returns a point along a path between two points, at a specified distance from the start.
    ///
    /// This method finds the coordinates of a point that is a certain distance along
    /// a straight line from start to end.
    ///
    /// # Parameters
    ///
    /// - `start`: Starting point with coordinates in degrees (longitude, latitude)
    /// - `end`: Ending point with coordinates in degrees (longitude, latitude)
    /// - `distance_from_start`: Distance from start in meters
    ///
    /// # Returns
    ///
    /// A point that is the specified distance along the path from start to end.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo::{point, InterpolatePoint};
    /// use geo_ruler::Ruler;
    ///
    /// let empire_state = point!(x: -73.9857, y: 40.7484); // Empire State Building
    /// let flatiron = point!(x: -73.9897, y: 40.7411);  // Flatiron Building
    ///
    /// // Find a point 100 meters from Empire State along the path to Flatiron
    /// let midway = Ruler::WGS84.point_at_distance_between(empire_state, flatiron, 100.0);
    /// ```
    ///
    /// See also: [`point_at_ratio_between`](#method.point_at_ratio_between), [`points_along_line`](#method.points_along_line)
    fn point_at_distance_between(
        &self,
        start: Point<F>,
        end: Point<F>,
        distance_from_start: F,
    ) -> Point<F> {
        let bearing = self.bearing(start, end);
        self.destination(start, bearing, distance_from_start)
    }

    /// Returns a point along a path between two points, at a specified ratio of the total path length.
    ///
    /// For the Cheap Ruler implementation, this uses a simple linear interpolation between
    /// the coordinates rather than a true geodesic calculation, which works well for
    /// city-scale distances.
    ///
    /// # Parameters
    ///
    /// - `start`: Starting point with coordinates in degrees (longitude, latitude)
    /// - `end`: Ending point with coordinates in degrees (longitude, latitude)
    /// - `ratio_from_start`: Ratio of the distance from start (0.0 = start, 1.0 = end)
    ///
    /// # Returns
    ///
    /// A point that is at the specified ratio along the path from start to end.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo::{point, InterpolatePoint};
    /// use geo_ruler::Ruler;
    ///
    /// let empire_state = point!(x: -73.9857, y: 40.7484); // Empire State Building
    /// let flatiron = point!(x: -73.9897, y: 40.7411);  // Flatiron Building
    ///
    /// // Find the midpoint between Empire State and Flatiron
    /// let midpoint = Ruler::WGS84.point_at_ratio_between(empire_state, flatiron, 0.5);
    /// ```
    ///
    /// See also: [`point_at_distance_between`](#method.point_at_distance_between), [`points_along_line`](#method.points_along_line)
    fn point_at_ratio_between(
        &self,
        start: Point<F>,
        end: Point<F>,
        ratio_from_start: F,
    ) -> Point<F> {
        let dx = (end.x() - start.x()) * ratio_from_start;
        let dy = (end.y() - start.y()) * ratio_from_start;

        point!(x : start.x() + dx, y: start.y() + dy)
    }

    /// Returns an iterator of evenly spaced points along a line between two points.
    ///
    /// This method generates points along a path such that the distance between consecutive
    /// points does not exceed the specified maximum distance.
    ///
    /// # Parameters
    ///
    /// - `start`: Starting point with coordinates in degrees (longitude, latitude)
    /// - `end`: Ending point with coordinates in degrees (longitude, latitude)
    /// - `max_distance`: Maximum distance between consecutive points in meters
    /// - `include_ends`: Whether to include the start and end points in the result
    ///
    /// # Returns
    ///
    /// An iterator that yields points along the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo::{point, InterpolatePoint};
    /// use geo_ruler::Ruler;
    ///
    /// let empire_state = point!(x: -73.9857, y: 40.7484); // Empire State Building
    /// let flatiron = point!(x: -73.9897, y: 40.7411);  // Flatiron Building
    ///
    /// // Generate points every 50m along the path from Empire State to Flatiron
    /// let points = Ruler::WGS84.points_along_line(empire_state, flatiron, 50.0, true)
    ///     .collect::<Vec<_>>();
    ///
    /// // Points will include Empire State, points at ~50m intervals, and Flatiron
    /// assert_eq!(points.first().unwrap(), &empire_state);
    /// assert_eq!(points.last().unwrap(), &flatiron);
    /// ```
    ///
    /// See also: [`point_at_distance_between`](#method.point_at_distance_between), [`point_at_ratio_between`](#method.point_at_ratio_between)
    fn points_along_line(
        &self,
        start: Point<F>,
        end: Point<F>,
        max_distance: F,
        include_ends: bool,
    ) -> impl Iterator<Item = Point<F>> {
        let distance = self.distance(start, end);
        let step = max_distance / distance;

        LinePointInterpolator::new(start, end, self, step, include_ends)
    }
}

/// Helper iterator for generating evenly spaced points along a line
struct LinePointInterpolator<'ruler, F: CoordFloat> {
    /// Starting point of the line
    start: Point<F>,
    /// Ending point of the line
    end: Point<F>,
    /// Current offset ratio along the line (0.0 to 1.0)
    offset: F,
    /// Step size for each iteration as a ratio of the total distance
    step: F,
    /// Reference to the ruler for calculations
    ruler: &'ruler Ruler<F>,
    /// Whether to include the endpoint as the final item
    include_last: bool,
}

impl<'ruler, F: CoordFloat + FloatConst> LinePointInterpolator<'ruler, F> {
    /// Creates a new line point iterator.
    ///
    /// # Parameters
    ///
    /// - `start`: The starting point
    /// - `end`: The ending point
    /// - `ruler`: Reference to the ruler for calculations
    /// - `step`: Step size as a ratio of the total distance
    /// - `include_ends`: Whether to include start and end points
    fn new(
        start: Point<F>,
        end: Point<F>,
        ruler: &'ruler Ruler<F>,
        step: F,
        include_ends: bool,
    ) -> Self {
        let offset = match include_ends {
            true => F::zero(),
            false => step,
        };

        Self {
            start,
            end,
            ruler,
            include_last: include_ends,
            offset,
            step,
        }
    }

    /// Returns point along the line at the current offset and advances the offset.
    fn advance(&mut self) -> Point<F> {
        let current = self
            .ruler
            .point_at_ratio_between(self.start, self.end, self.offset);

        self.offset = self.offset + self.step;

        current
    }

    /// Should be called at the end of the line to potentially
    /// include the end point.
    fn stop(&mut self) -> Option<Point<F>> {
        if self.include_last {
            self.include_last = false;
            Some(self.end)
        } else {
            None
        }
    }
}

impl<F: CoordFloat + FloatConst> Iterator for LinePointInterpolator<'_, F> {
    type Item = Point<F>;

    /// Advances along the line until reaching the end, then potentially
    /// yields the end point as the final item.
    fn next(&mut self) -> Option<Self::Item> {
        (self.offset < F::one())
            .then(|| self.advance())
            .or_else(|| self.stop())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::{assert_relative_eq, relative_eq};
    use geo::{Geodesic, GeodesicMeasure};

    const RELATIVE_ERROR: f64 = 0.01;
    const EPSILON: f64 = 0.01;

    #[test]
    fn distance() {
        let empire_state = point!(x: -73.9857, y: 40.7484);
        let flatiron = point!(x: -73.9897, y: 40.7411);

        let distance_geodesic = Geodesic.distance(empire_state, flatiron);
        let distance_ruler = Ruler::WGS84.distance(empire_state, flatiron);

        assert_relative_eq!(
            distance_geodesic,
            distance_ruler,
            max_relative = RELATIVE_ERROR
        );
    }

    #[test]
    fn destination() {
        let empire_state = point!(x: -73.9857, y: 40.7484);
        let distance = 100.;
        let bearing = 45.;

        let geodesic_point = Geodesic.destination(empire_state, bearing, distance);
        let ruler_point = Ruler::WGS84.destination(empire_state, bearing, distance);

        let error = Geodesic.distance(geodesic_point, ruler_point);

        assert_relative_eq!(0., error, epsilon = EPSILON, max_relative = RELATIVE_ERROR)
    }

    #[test]
    fn bearing() {
        let empire_state = point!(x: -73.9857, y: 40.7484);
        let flatiron = point!(x: -73.9897, y: 40.7411);

        assert_relative_eq!(
            Geodesic.bearing(empire_state, flatiron).rem_euclid(360.),
            Ruler::WGS84
                .bearing(empire_state, flatiron)
                .rem_euclid(360.),
            max_relative = RELATIVE_ERROR
        )
    }

    #[test]
    fn interpolate_distance() {
        let empire_state = point!(x: -73.9857, y: 40.7484);
        let flatiron = point!(x: -73.9897, y: 40.7411);
        let distance = 100.;

        let geodesic_point = Geodesic.point_at_distance_between(empire_state, flatiron, distance);
        let ruler_point = Ruler::WGS84.point_at_distance_between(empire_state, flatiron, distance);

        let error = Geodesic.distance(geodesic_point, ruler_point);

        assert_relative_eq!(0., error, epsilon = EPSILON, max_relative = RELATIVE_ERROR)
    }

    #[test]
    fn interpolate_ratio() {
        let empire_state = point!(x: -73.9857, y: 40.7484);
        let flatiron = point!(x: -73.9897, y: 40.7411);
        let ratio = 0.15;

        let geodesic_point = Geodesic.point_at_ratio_between(empire_state, flatiron, ratio);
        let ruler_point = Ruler::WGS84.point_at_ratio_between(empire_state, flatiron, ratio);

        let error = Geodesic.distance(geodesic_point, ruler_point);

        assert_relative_eq!(0., error, epsilon = EPSILON, max_relative = RELATIVE_ERROR)
    }

    #[test]
    fn interpolate_along_no_ends() {
        let empire_state = point!(x: -73.9857, y: 40.7484);
        let flatiron = point!(x: -73.9897, y: 40.7411);
        let max_distance = 100.;

        let mut points =
            Ruler::WGS84.points_along_line(empire_state, flatiron, max_distance, false);

        let Some(mut prev) = points.next() else {
            panic!("points is empty");
        };

        let mut found_empire_state = prev == empire_state;
        let mut found_flatiron = prev == flatiron;

        for curr in points {
            let distance = Geodesic.distance(prev, curr);

            if curr == empire_state {
                found_empire_state = true;
            }

            if curr == flatiron {
                found_flatiron = true;
            }

            prev = curr;

            assert!(
                distance < max_distance
                    || relative_eq!(distance, max_distance, max_relative = RELATIVE_ERROR),
            )
        }

        assert!(!found_flatiron && !found_empire_state);
    }

    #[test]
    fn interpolate_along_with_ends() {
        let empire_state = point!(x: -73.9857, y: 40.7484);
        let flatiron = point!(x: -73.9897, y: 40.7411);
        let max_distance = 100.;

        let mut points = Ruler::WGS84.points_along_line(empire_state, flatiron, max_distance, true);

        let Some(mut prev) = points.next() else {
            panic!("points is empty");
        };

        let mut found_empire_state = prev == empire_state;
        let mut found_flatiron = prev == flatiron;

        for curr in points {
            let distance = Geodesic.distance(prev, curr);

            if curr == empire_state {
                found_empire_state = true;
            }

            if curr == flatiron {
                found_flatiron = true;
            }

            prev = curr;

            assert!(
                distance < max_distance
                    || relative_eq!(distance, max_distance, max_relative = RELATIVE_ERROR),
            )
        }

        assert!(found_flatiron && found_empire_state);
    }

    #[test]
    fn non_standard_model() {
        let olympus_mons = point!(x: -226.2, y: 18.65);
        let karzok_crater = point!(x: -226.192, y: 18.2292);

        let mars_equatorial_radius = 3_396_200.;
        let mars_polar_radius = 3_376_200.;
        let mars_flattening = 0.00589;

        let mars_geoid = GeodesicMeasure::new(mars_equatorial_radius, mars_flattening);
        let mars_ruler = Ruler::new(mars_equatorial_radius, mars_polar_radius);

        let distance_geodesic = mars_geoid.distance(olympus_mons, karzok_crater);
        let distance_ruler = mars_ruler.distance(olympus_mons, karzok_crater);

        assert_relative_eq!(
            distance_geodesic,
            distance_ruler,
            max_relative = RELATIVE_ERROR
        );
    }
}
