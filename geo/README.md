# ruler-geo

Integration with the [geo-rs](https://docs.rs/geo/latest/geo/) crate for the [ruler](https://github.com/scristobal/geo-ruler) geodesic calculations library.

Extends the existing [metric spaces](https://docs.rs/geo/latest/geo/#metric-spaces) (namely `Geodesic`, `Haversine` and `Rhumb`) with a new `RulerMeasure` measure that prioritizes performance over precision while maintaining acceptable accuracy for most city-scale applications.

## Traits

Implements the following [operations on metric spaces](https://docs.rs/geo/latest/geo/#operations-on-metric-spaces):

- [`Distance`](https://docs.rs/geo/latest/geo/algorithm/line_measures/trait.Distance.html)
- [`Bearing`](https://docs.rs/geo/latest/geo/algorithm/line_measures/trait.Bearing.html)
- [`Destination`](https://docs.rs/geo/latest/geo/algorithm/line_measures/trait.Destination.html)
- [`InterpolatePoint`](https://docs.rs/geo/latest/geo/algorithm/line_measures/trait.InterpolatePoint.html)

## Usage

Calculate distance between two landmarks:

```rust
use geo::{point, Distance};
use ruler_geo::RulerMeasure;

fn main() {
    let empire_state = point!(x: -73.9857, y: 40.7484);
    let flatiron = point!(x: -73.9897, y: 40.7411);

    let distance = RulerMeasure::WGS84().distance(empire_state, flatiron);

    println!("Distance: {:.1} meters", distance);
}
```

Generate points along a path:

```rust
use geo::{point, InterpolatePoint};
use ruler_geo::RulerMeasure;

fn main() {
    let empire_state = point!(x: -73.9857, y: 40.7484);
    let flatiron = point!(x: -73.9897, y: 40.7411);

    let points = RulerMeasure::WGS84()
        .points_along_line(empire_state, flatiron, 50.0, true)
        .collect::<Vec<_>>();

    println!("Generated {} points along the path", points.len());
}
```

## License

MIT
