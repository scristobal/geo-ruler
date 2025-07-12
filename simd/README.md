# simd-ruler

High-performance SIMD-accelerated geographic calculations for Rust.

A vectorized geospatial library that processes multiple coordinate pairs simultaneously using the `wide` crate, achieving significant performance improvements over iterative approaches.

## Features

- **SIMD Vectorization**: Processes multiple coordinate pairs simultaneously using the `wide` crate
- **Cross-platform**: Works on stable Rust across different architectures
- **Minimal Dependencies**: Uses only the `wide` crate for SIMD operations

## Requirements

- **Stable Rust toolchain** - no nightly required
- Compatible with various CPU architectures through the `wide` crate

## Performance

Benchmarked on Intel i9-11900K running Linux 6.15.5-arch1-1:

- **vectorized**: 524.93 ns
- **scalar**: 6.86 µs

## Usage

Calculate the total length of a polyline:

```rust
use simd_ruler::length;

let longitudes = [-73.9857, -73.9897, -73.9927];
let latitudes = [40.7484, 40.7411, 40.7394];
let points = [&longitudes[..], &latitudes[..]];

let distance = length(&points); // Returns meters
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
simd-ruler = "0.1.0"
```

## License

Licensed under the same terms as the parent `geo-ruler` project.
