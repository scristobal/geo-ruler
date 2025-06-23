//! This example demonstrates how to calculate the distance between two geographic points
//! using the fast Ruler implementation, which is optimized for city-scale distances.

use geo::{Distance, point};
use geo_ruler::Ruler;

fn main() {
    // Define our two landmarks in New York City (longitude, latitude in degrees)
    let empire_state = point!(x: -73.9857, y: 40.7484); // Empire State Building
    let flatiron = point!(x: -73.9897, y: 40.7411); // Flatiron Building

    // Calculate the distance between the two points using the WGS84 ellipsoid model
    // The Ruler implementation provides fast approximation suitable for city-scale distances
    let distance = Ruler::WGS84.distance(empire_state, flatiron);

    println!("Distance from Empire State Building to Flatiron Building: {distance:.1} meters");
}
