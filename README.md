# Ruler

A fast, city-scale geodesic approximation library based on [Mapbox's Cheap Ruler algorithm](https://blog.mapbox.com/fast-geodesic-approximations-with-cheap-ruler-106f229ad016).

The `ruler-geo` crate extends the existing [geo-rs](https://docs.rs/geo/latest/geo/) [metric spaces](https://docs.rs/geo/latest/geo/#metric-spaces) (namely `Geodesic`, `Haversine` and `Rhumb`) with a new `RulerMeasure` measure that prioritizes performance over precision while maintaining acceptable accuracy for most city-scale applications.

## Features

- Implements most common [operations on metric spaces](https://docs.rs/geo/latest/geo/#operations-on-metric-spaces) using generic `Float` types with zero-cost abstractions
- Uses a locally flat Earth approximation with latitude-dependent scaling, 20-100x faster than traditional methods like Haversine or Vincenty's formulas with typically < 0.1% error for distances up to 500 km
- Local formulas use the [WGS84 ellipsoidal model](https://en.wikipedia.org/wiki/World_Geodetic_System#WGS_84), but also support other elliptical models, such as GRS80, or even other celestial bodies
- Different approximate algorithms for `atan2` computations can be optionally enabled using cargo features
- Comprehensive test suite, property invariants, and correctness verification against [Karney (2013) Geodesic model](https://arxiv.org/pdf/1109.4448.pdf) using fuzz testing
- No heap allocations and `#![no_std]` compatible
- Optional WebAssembly bindings via the `ruler-js` crate
- Optional Python bindings via the `ruler-py` crate (built with [maturin](https://www.maturin.rs/))
- Experimental `ruler-simd` crate with SIMD-accelerated implementations of common aggregated geodesic operations, eg. length of a polyline.

## Examples

### From Rust using the `geo` crate

Calculate distance between two landmarks:

```rust
use geo::{point, Distance};
use ruler_geo::RulerMeasure;

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
use ruler_geo::RulerMeasure;

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

The `ruler-js` crate provides WebAssembly bindings. Build with [wasm-pack](https://rustwasm.github.io/wasm-pack/):

```bash
cargo install wasm-pack
wasm-pack build wasm/ --target web --out-dir pkg
```

Then use it in your JavaScript code:

```javascript
import init, { Coords } from './pkg/ruler_js.js';

async function main() {
    await init();

    const empireState = new Coords(-73.9857, 40.7484);
    const flatiron = new Coords(-73.9897, 40.7411);

    const distance = empireState.distance(flatiron);
    console.log(`Distance: ${distance.toFixed(1)} meters`);

    const bearing = empireState.bearing(flatiron);
    console.log(`Bearing: ${bearing.toFixed(1)} degrees`);

    const destination = empireState.destination(bearing, distance);
    console.log(`Destination: ${destination.x}, ${destination.y}`);
}

main();
```

### From Python

The `ruler-py` crate provides Python bindings. Build with [maturin](https://www.maturin.rs/):

```bash
pip install maturin
maturin develop -m python/Cargo.toml
```

Then use it in your Python code:

```python
from geo_ruler import Coords

empire_state = Coords(-73.9857, 40.7484)
flatiron = Coords(-73.9897, 40.7411)

distance = empire_state.distance(flatiron)
print(f"Distance: {distance:.1f} meters")

bearing = empire_state.bearing(flatiron)
print(f"Bearing: {bearing:.1f} degrees")

destination = empire_state.destination(bearing, distance)
print(f"Destination: {destination.x}, {destination.y}")
```

## Performance

Geo Ruler is optimized for high performance at the cost of accuracy, especially for city-scale distances. It achieves this by using a flat-Earth approximation with latitude-dependent scaling, allowing for simple Euclidean calculations instead of complex spherical geometry.

Below are benchmark results comparing Geo Ruler against the other geo-rs implementations:

| Operation            | RulerMeasure (atan2_deg3) | RulerMeasure (default) | Geodesic   | Haversine | Rhumb     |
|----------------------|---------------------------|------------------------|------------|-----------|-----------|
| Distance             | 6.17 ns                   | 6.17 ns                | 402.61 ns  | 16.35 ns  | 21.31 ns  |
| Bearing              | 8.51 ns                   | 19.79 ns               | 405.20 ns  | 25.06 ns  | 31.38 ns  |
| Destination          | 9.14 ns                   | 9.11 ns                | 206.43 ns  | 48.95 ns  | 33.21 ns  |
| Interpolate Distance | 25.57 ns                  | 37.08 ns               | 629.54 ns  | 84.33 ns  | 90.63 ns  |
| Interpolate Ratio    | 0.96 ns                   | 0.99 ns                | 647.79 ns  | 90.97 ns  | 89.71 ns  |
| Interpolate Along    | 10.85 ns                  | 10.85 ns               | 2160.50 ns | 349.33 ns | 348.23 ns |

Note: `default` refers to Rust's default `atan2` implementation (when `atan2_deg3` is not enabled), whilst `atan2_deg3` refers to the polynomial approximation provided by this crate. See the [Cargo Features](#cargo-features) section for more details on these options.

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

The `ruler-geo` crate extends the [geo-rs](https://docs.rs/geo/latest/geo/) ecosystem by implementing the following traits:

- [`Distance`](https://docs.rs/geo/latest/geo/algorithm/line_measures/trait.Distance.html)
- [`Bearing`](https://docs.rs/geo/latest/geo/algorithm/line_measures/trait.Bearing.html)
- [`Destination`](https://docs.rs/geo/latest/geo/algorithm/line_measures/trait.Destination.html)
- [`InterpolatePoint`](https://docs.rs/geo/latest/geo/algorithm/line_measures/trait.InterpolatePoint.html)

### Workspace Structure

| Crate | Description |
|-------|-------------|
| `ruler/` | Core library (`ruler`) |
| `geo/` | Integration with the [geo-rs](https://docs.rs/geo/latest/geo/) crate (`ruler-geo`) |
| `simd/` | SIMD-accelerated operations (`ruler-simd`) |
| `wasm/` | WebAssembly bindings (`ruler-js`) |
| `python/` | Python bindings (`ruler-py`) |

### Cargo Features

- **`std`**: Enable standard library support (enabled by default)
- **`atan2_deg3`**: Use a very fast and inaccurate 3rd degree polynomial approximation of `atan2` (enabled by default)

The core `ruler` crate is `#![no_std]` compatible. For embedded or `no_std` environments, use:

```toml
[dependencies]
ruler = { version = "0.3.0", default-features = false }
```

Note: When `atan2_deg3` is not enabled, Rust's default `atan2` implementation is used.

### Limitations

While Geo Ruler is highly efficient for common use cases, be aware of these limitations:

- Accuracy decreases as distances grow larger (beyond ~500 km)
- Not suitable for polar regions where meridians converge
- Not appropriate for applications requiring sub-meter precision over large distances

For high-precision global-scale calculations, consider using the full Geodesic implementation.

### License

MIT
