# Geo Ruler

A fast, city-scale geodesic approximation library for [geo-rs](https://docs.rs/geo/latest/geo/) based on [Mapbox's Cheap Ruler algorithm](https://blog.mapbox.com/fast-geodesic-approximations-with-cheap-ruler-106f229ad016).

This crate extends the existing [metric spaces](https://docs.rs/geo/latest/geo/#metric-spaces) (namely `Geodesic`, `Haversine` and `Rhumb`) with a new `RulerMeasure` measure that prioritizes performance over precision while maintaining acceptable accuracy for most city-scale applications.

## Features

- Implements most common [operations on metric spaces](https://docs.rs/geo/latest/geo/#operations-on-metric-spaces) using generic `Float` types with zero-cost abstractions
- Uses a locally flat Earth approximation with latitude-dependent scaling, 20-100x faster than traditional methods like Haversine or Vincenty's formulas with typically < 0.1% error for distances up to 500 km
- Local formulas use the [WGS84 ellipsoidal model](https://en.wikipedia.org/wiki/World_Geodetic_System#WGS_84), but also support other elliptical models, such as GRS80, or even other celestial bodies
- Different approximate algorithms for `atan2` computations can be optionally enabled using cargo features
- Comprehensive test suite, property invariants, and correctness verification against [Karney (2013) Geodesic model](https://arxiv.org/pdf/1109.4448.pdf) using fuzz testing
- No heap allocations and `#![no_std]`. However, the [geo](https://crates.io/crates/geo) crate does require `std`
- Optional WebAssembly bindings for use from JavaScript

## Examples

### From Rust using the `geo` crate

Calculate distance between two landmarks:

```rust
use geo::{point, Distance};
use geo_ruler::geo::RulerMeasure;

fn main() {
    let empire_state = point!(x: -73.9857, y: 40.7484);
    let flatiron = point!(x: -73.9897, y: 40.7411);

    // Calculate distance in meters
    let distance = RulerMeasure::WGS84().distance(empire_state, flatiron);

    println!("Distance: {:.1} meters", distance);
}
```

Generate points along a path:

```rust
use geo::{point, InterpolatePoint};
use geo_ruler::geo::RulerMeasure;

fn main() {
    let empire_state = point!(x: -73.9857, y: 40.7484);
    let flatiron = point!(x: -73.9897, y: 40.7411);

    // Generate points with maximum 50m between each point
    let points = RulerMeasure::WGS84()
        .points_along_line(empire_state, flatiron, 50.0, true)
        .collect::<Vec<_>>();

    println!("Generated {} points along the path", points.len());
}
```

### From JavaScript using WebAssembly

This library includes WebAssembly bindings, if the `wasm` feature is enabled. To use it from JavaScript, you'll need to build the WebAssembly module:

```bash
# Install wasm-pack if you haven't already
cargo install wasm-pack

# Build the WebAssembly module
wasm-pack build --target web --out-dir pkg
```

Then use it in your JavaScript code:

```javascript
import init, { Coords } from './pkg/geo_ruler.js';

async function main() {
    // Initialize the WebAssembly module
    await init();

    // Create coordinate points
    const empireState = new Coords(-73.9857, 40.7484);
    const flatiron = new Coords(-73.9897, 40.7411);

    // Calculate distance in meters
    const distance = empireState.distance(flatiron);
    console.log(`Distance: ${distance.toFixed(1)} meters`);

    // Calculate bearing in degrees
    const bearing = empireState.bearing(flatiron);
    console.log(`Bearing: ${bearing.toFixed(1)} degrees`);

    // Calculate destination point
    const destination = empireState.destination(bearing, distance);
    console.log(`Destination: ${destination.x}, ${destination.y}`);
}

main();
```

## Performance

Geo Ruler is optimized for high performance at the cost of accuracy, especially for city-scale distances. It achieves this by using a flat-Earth approximation with latitude-dependent scaling, allowing for simple Euclidean calculations instead of complex spherical geometry.

Below are benchmark results comparing Geo Ruler against the other geo-rs implementations:

| Operation            | RulerMeasure (atan2_deg3) | RulerMeasure (atan2_deg5) | RulerMeasure (default) | Geodesic   | Haversine | Rhumb     |
|----------------------|---------------------------|---------------------------|------------------------|------------|-----------|-----------|
| Distance             | 6.17 ns                   | 6.19 ns                   | 6.17 ns                | 402.61 ns  | 16.35 ns  | 21.31 ns  |
| Bearing              | 8.51 ns                   | 10.30 ns                  | 19.79 ns               | 405.20 ns  | 25.06 ns  | 31.38 ns  |
| Destination          | 9.14 ns                   | 9.12 ns                   | 9.11 ns                | 206.43 ns  | 48.95 ns  | 33.21 ns  |
| Interpolate Distance | 25.57 ns                  | 28.52 ns                  | 37.08 ns               | 629.54 ns  | 84.33 ns  | 90.63 ns  |
| Interpolate Ratio    | 0.96 ns                   | 0.96 ns                   | 0.99 ns                | 647.79 ns  | 90.97 ns  | 89.71 ns  |
| Interpolate Along    | 10.85 ns                  | 10.85 ns                  | 10.85 ns               | 2160.50 ns | 349.33 ns | 348.23 ns |

Note: `default` refers to Rust's default `atan2` implementation (when neither `atan2_deg3` nor `atan2_deg5` features are enabled), whilst `atan2_deg3` and `atan2_deg5` refer to the polynomial approximations provided by this crate. See the [Cargo Features](#cargo-features) section for more details on these options.

### Warning

These benchmarks were performed on city-scale distances between the Empire State Building and Flatiron Building, which is under 1 km. If you are working with larger distances, the performance difference will not matter because the outputs will most likely be wrong.

For a more accurate geodesic implementation, consider using the `geo` crate's `Geodesic` metric space.

## Overview

### How It Works

The Cheap Ruler algorithm, [developed by Mapbox](https://blog.mapbox.com/fast-geodesic-approximations-with-cheap-ruler-106f229ad016), uses a flat-Earth approximation with latitude-dependent scaling.

For a given latitude, the algorithm:

1. Precalculates scale factors for converting longitude and latitude degrees to meters
2. Uses these factors to perform simple Euclidean calculations instead of complex spherical geometry
3. Maintains high accuracy for city-scale distances (typically < 0.1% error for distances up to ~500 km)

This approach is significantly faster than traditional methods like Haversine or Vincenty's formulas while maintaining excellent accuracy for most practical applications.

### Integration with geo-rs

This library extends the [geo-rs](https://docs.rs/geo/latest/geo/) ecosystem by implementing the following traits:

- [`Distance`](https://docs.rs/geo/latest/geo/algorithm/line_measures/trait.Distance.html)
- [`Bearing`](https://docs.rs/geo/latest/geo/algorithm/line_measures/trait.Bearing.html)
- [`Destination`](https://docs.rs/geo/latest/geo/algorithm/line_measures/trait.Destination.html)
- [`InterpolatePoint`](https://docs.rs/geo/latest/geo/algorithm/line_measures/trait.InterpolatePoint.html)

### Cargo Features

This library supports multiple implementations of the `atan2` function to calculate bearing and additional features:

- **`geo`**: Integration with the geo-rs crate ecosystem (enabled by default)
- **`wasm`**: WebAssembly bindings for JavaScript interop (enabled by default)
- **`atan2_deg3`**: Use a very fast and inaccurate 3rd degree polynomial approximation of `atan2` (enabled by default)
- **`atan2_deg5`**: Use a fast and less accurate 5th degree polynomial approximation of `atan2`

Note: When neither `atan2_deg3` nor `atan2_deg5` is enabled, Rust's default `atan2` implementation is used.

### Limitations

While Geo Ruler is highly efficient for common use cases, be aware of these limitations:

- Accuracy decreases as distances grow larger (beyond ~500 km)
- Not suitable for polar regions where meridians converge
- Not appropriate for applications requiring sub-meter precision over large distances

For high-precision global-scale calculations, consider using the full Geodesic implementation.

### License

MIT
