//! Property tests and correctness verification for the `geo_ruler` crate.
//!
//! This test suite verifies that the `Ruler` implementation maintains essential mathematical
//! properties of geodesic calculations (such as symmetry and accuracy).
//!
//! For verification, we use the Geodesic model from the `geo` crate as the reference implementation.
//! The Geodesic model provides highly accurate calculations based on Karney's algorithm (2013),
//! which is considered the gold standard for geodesic problems on an ellipsoid. By comparing
//! our approximation against this reference implementation, we ensure that the `Ruler`
//! maintains acceptable accuracy for city-scale distances.

use approx::relative_eq;
use geo::Geodesic;
use geo::{Bearing, Destination, Distance, point};
use geo_ruler::Ruler;
use proptest::prelude::*;

const RELATIVE_ERROR: f64 = 0.01;

proptest! {
    #[test]
    fn distance_arguments_are_symmetric(distance in 100f64..1000., bearing in 0f64..360.) {
        let empire_state = point!(x: -73.9857, y: 40.7484);

        let ruler = Ruler::WGS84;

        let destination = ruler.destination(empire_state, bearing, distance);

        let error = (ruler.distance(empire_state, destination) - ruler.distance(destination, empire_state)).abs();

        prop_assert!((error/distance) < RELATIVE_ERROR );
    }

    #[test]
    fn destination_is_an_involutory_function(distance in 100f64..1000., bearing in 0f64..360.) {
        let empire_state = point!(x: -73.9857, y: 40.7484);

        let ruler = Ruler::WGS84;

        let destination = ruler.destination(empire_state, bearing, distance);

        let inverse_bearing = (bearing + 180.).rem_euclid(360.);
        let origin = ruler.destination(destination, inverse_bearing, distance);

        let error = (ruler.distance(origin, destination) - distance).abs();

        prop_assert!((error/distance) < RELATIVE_ERROR );
    }

    #[test]
    fn distance_is_precise(distance in 100f64..1000., bearing in 0f64..360.) {
        let empire_state = point!(x: -73.9857, y: 40.7484);
        let target = Geodesic.destination(empire_state, bearing, distance);

        let ruler = Ruler::WGS84;

        let error = (ruler.distance(empire_state, target) - distance).abs();

        prop_assert!((error/distance) < RELATIVE_ERROR);
    }

    #[test]
    fn bearing_is_precise(distance in 100f64..1000., bearing in 0f64..360.) {
        let empire_state = point!(x: -73.9857, y: 40.7484);
        let target = Geodesic.destination(empire_state, bearing, distance);

        let ruler = Ruler::WGS84;

        let error = (ruler.bearing(empire_state, target) - bearing).rem_euclid(360.);

        prop_assert!(error/distance < RELATIVE_ERROR
            || relative_eq!(error, 360., max_relative = RELATIVE_ERROR)
            || relative_eq!(error, 0., max_relative = RELATIVE_ERROR))
    }

    #[test]
    fn destination_is_precise(distance in 100f64..1000., bearing in 0f64..360.) {
        let empire_state = point!(x: -73.9857, y: 40.7484);

        let target = Geodesic.destination(empire_state, bearing, distance);

        let ruler = Ruler::WGS84;

        let destination = ruler.destination(empire_state, bearing, distance);

        let error = Geodesic.distance(target, destination);

        prop_assert!((error/distance) < RELATIVE_ERROR);
    }
}
