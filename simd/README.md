# simd-ruler

High-performance SIMD-accelerated geographic calculations for Rust.

A vectorized geospatial library that processes multiple coordinate pairs simultaneously using Rust's portable SIMD, achieving significant performance improvements over iterative approaches.

## Features

- **SIMD Vectorization**: Processes multiple coordinate pairs simultaneously
- **Backwards Compatible**: If SIMD is not available, it falls back to a standard iterative implementation
- **No External Dependencies**: Pure Rust implementation with no external crates required

## Requirements

⚠️ **Requires nightly Rust toolchain** due to unstable [`portable_simd`](https://doc.rust-lang.org/std/simd/index.html) feature.

## Performance

Benchmarked on Apple MacBook Pro M1 Pro:

- **SIMD**: 7.93 µs
- **Iterative**: 22.45 µs

An approximate **speedup** of ~2.8x

## Usage

Calculate the total length of a polyline:

```rust
use simd_ruler::length;

let longitudes = [-73.9857, -73.9897, -73.9927];
let latitudes = [40.7484, 40.7411, 40.7394];
let points = [&longitudes[..], &latitudes[..]];

let distance = length(&points); // Returns meters
```

Ensure you're using a nightly Rust toolchain:

```bash
rustup toolchain install nightly
rustup default nightly
```

## License

Licensed under the same terms as the parent `geo-ruler` project.
