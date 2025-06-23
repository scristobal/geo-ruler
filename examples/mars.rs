//! This example demonstrates how to calculate the distance between two geographic points
//! using the fast Ruler implementation, which is optimized for city-scale distances.

use geo::{Distance, point};
use geo_ruler::Ruler;

fn main() {
    // Define our two landmarks on Mars (longitude, latitude in degrees)
    let olympus_mons = point!(x: -226.2, y: 18.65); // Olympus Mons
    let karzok_crater = point!(x: -226.192, y: 18.2292); // Karzok Crater

    // Mars has different equatorial and polar radii compared to Earth
    // These values are based on the IAU standard for Mars
    let mars_equatorial_radius = 3_396_200.;
    let mars_polar_radius = 3_376_200.;

    // Create a Ruler instance for Mars using its equatorial and polar radii
    // This allows us to calculate distances on the Martian surface accurately
    let mars_ruler = Ruler::new(mars_equatorial_radius, mars_polar_radius);

    // Calculate the distance between the two points using the Mars Ruler instance
    let distance = mars_ruler.distance(olympus_mons, karzok_crater);

    println!("Distance from Olympus Mons to Karzok Crater on Mars: {distance:.1} meters");
}
