//! This example demonstrates how to generate evenly spaced points
//! along a line between two landmarks in New York City using the Ruler.

use geo::{InterpolatePoint, point};
use geo_ruler::geo::RulerMeasure;

fn main() {
    // Define our two landmarks in New York City (longitude, latitude in degrees)
    let empire_state = point!(x: -73.9857, y: 40.7484); // Empire State Building
    let flat_iron = point!(x: -73.9897, y: 40.7411); // Flatiron Building

    // Set the maximum distance between consecutive points (in meters)
    let distance = 100.;

    // Generate points along the line with maximum 100m between each point
    // The last parameter (true) means we'll include both the start and end points
    let points = RulerMeasure::WGS84()
        .points_along_line(empire_state, flat_iron, distance, true)
        .collect::<Vec<_>>();

    println!("Points along the line from Empire State Building to Flatiron Building:");
    for (i, point) in points.iter().enumerate() {
        // Note that point.y() is latitude and point.x() is longitude
        println!("point {i}: lat: {:.4}, lon: {:.4}", point.y(), point.x());
    }
}
