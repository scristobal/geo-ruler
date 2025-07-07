#![feature(iter_map_windows)]

use approx::assert_relative_eq;
use geo_ruler::CheapRuler;
use simd_ruler;

const RELATIVE_ERROR: f32 = 0.01;

#[test]
fn test_simd_length_basic() {
    let lats = vec![40.7484, 40.7500, 40.7516, 40.7532, 40.7540, 40.7550];
    let lons = vec![-73.9857, -73.9840, -73.9823, -73.9806, -73.9790, -73.9770];

    let points = [&lons[..], &lats[..]];

    let simd_length = simd_ruler::length::<4>(&points);

    let ruler = CheapRuler::WGS84();

    let points: Vec<[f32; 2]> = lons
        .iter()
        .zip(lats.iter())
        .map(|(&lon, &lat)| [lon, lat])
        .collect();

    let reference_length = points
        .iter()
        .map_windows(|[p, q]| ruler.distance(p, q))
        .sum::<f32>();

    assert_relative_eq!(simd_length, reference_length, max_relative = RELATIVE_ERROR);
}
