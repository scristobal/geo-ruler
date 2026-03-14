# ruler-simd

High-performance SIMD-accelerated geographic calculations for Rust.

A vectorized geospatial library that processes multiple coordinate pairs simultaneously using the `wide` crate, achieving significant performance improvements over iterative approaches.

## Features

- **SIMD Vectorization**: Processes multiple coordinate pairs simultaneously using the `wide` crate
- **Cross-platform**: Works on stable Rust across different architectures
- **`#![no_std]`**: No standard library dependency, falls back to `libm` for scalar math
- **Minimal Dependencies**: Uses only the `wide` and `libm` crates

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
use ruler_simd::length;

let longitudes = [-73.9857, -73.9897, -73.9927];
let latitudes = [40.7484, 40.7411, 40.7394];
let points = [&longitudes[..], &latitudes[..]];

let distance = length(&points); // Returns meters
```

### Cargo Features

- **`std`**: Enable standard library support and use platform math via `wide/std` (enabled by default). Without it, scalar math falls back to `libm`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ruler-simd = "0.1.0"
```

For `no_std` environments, disable default features:

```toml
[dependencies]
ruler-simd = { version = "0.1.0", default-features = false }
```

## License

Licensed under the same terms as the parent project.
