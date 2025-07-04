//! This module provides alternative implementations of the atan2 function to optimize performance.
//!
//! The Rust's default `atan2` function is precise but can be computationally expensive.
//! This module offers faster polynomial approximations that are suitable for applications
//! where maximum performance is desired and small errors are acceptable.
//!
//! Two implementations are available:
//! - `atan2_deg3`: A 3rd degree polynomial approximation that is faster but less accurate (typical error < 0.1 rad)
//! - `atan2_deg5`: A 5th degree polynomial approximation that offers better accuracy (typical error < 0.01 rad) with a small performance cost
//!
//! These implementations can be used by enabling the corresponding feature flags:
//! - `atan2_deg3`
//! - `atan2_deg5`
//!
//! If neither feature is enabled, Rust's default `atan2` implementation is used.
//!
//! # Usage
//!
//! These optimized implementations are automatically used by the `bearing` method when
//! the corresponding feature flag is enabled.

use num_traits::{Float, FloatConst};

/// Fast 3rd degree polynomial approximation of the atan2 function.
///
/// This implementation uses a simplified polynomial approximation to calculate
/// the arctangent of two values. It offers significantly better performance compared
/// to more precise implementations (potentially 2-3x faster), at the cost of accuracy.
///
/// The maximum error is approximately 0.1 radians (~5.7 degrees) with typical errors
/// much smaller for common angle values.
///
/// # Implementation Details
///
/// Adapted from https://mazzo.li/posts/vectorized-atan2.html by Francesco Mazzoli.
/// Uses piecewise polynomial approximation with error-minimizing coefficients.
///
/// # Feature Flag
///
/// Available when compiled with the `atan2_deg3` feature flag.
///
/// # See Also
///
/// - [`atan2` with `atan2_deg5` flag](#method.atan2-1): Higher precision approximation
/// - Rust's default `atan2`: Maximum precision, used when no feature flags are enabled
#[cfg(feature = "atan2_deg3")]
pub fn atan2<F: Float + FloatConst + From<f32>>(y: F, x: F) -> F {
    let a1: F = 0.9817f32.into();
    let a3: F = 0.1963f32.into();

    let pi_4 = F::FRAC_PI_4();

    let abs_y = y.abs();

    let (r, a) = if x < F::zero() {
        ((x + abs_y) / (abs_y - x), pi_4 + pi_4 + pi_4)
    } else {
        ((x - abs_y) / (x + abs_y), pi_4)
    };

    let mut res = a + (a3 * r * r - a1) * r;

    if y < F::zero() {
        res = -res;
    }

    res
}

/// Accurate 5th degree polynomial approximation of the atan2 function.
///
/// This implementation uses a 5th degree polynomial to calculate the arctangent of two values.
/// It offers better accuracy than the 3rd degree version while maintaining good performance
/// (typically 1.5-2x faster than more precise implementations).
///
///gThe maximum error is approximately 0.01 radians (~0.57 degrees) with typical errors
/// much smaller for common angle values. This makes it suitable for most practical applications
/// where the small precision trade-off is acceptable for improved performance.
///
/// # Implementation Details
///
/// Adapted from https://mazzo.li/posts/vectorized-atan2.html by Francesco Mazzoli.
/// Polynomial coefficients are from "Approximations for digital computers" by Cecil Hastings,
/// a reference text for efficient numerical approximations.
///
/// Uses Horner's method for efficient polynomial evaluation, which minimizes the number
/// of multiplications required.
///
/// # Feature Flag
///
/// Available when compiled with the `atan2_deg5` feature flag.
///
/// # See Also
///
/// - [`atan2` with `atan2_deg3` flag](#method.atan2-2): Faster but less accurate approximation
/// - Rust's Default `atan2`: Maximum precision, used when no feature flags are enabled
#[cfg(feature = "atan2_deg5")]
pub fn atan2<F: Float + FloatConst + From<f32>>(y: F, x: F) -> F {
    let abs_y = y.abs();
    let abs_x = x.abs();

    /// Internal helper function for the 5th degree atan approximation
    ///
    /// Calculates atan(x) for |x| ≤ 1 using a 5th degree polynomial approximation.
    /// This function handles only the raw polynomial calculation and expects
    /// appropriate range reduction to have been performed by the caller.
    ///
    /// Uses polynomial coefficients from "Approximations for digital computers" by Cecil Hastings
    /// and Horner's method for optimal polynomial evaluation
    fn raw_atan_5<G: Float + From<f32>>(x: G) -> G {
        let a1: G = (0.995354f32).into();
        let a3: G = (-0.288679f32).into();
        let a5: G = (0.079331f32).into();

        let x_sq = x * x;
        x * (a1 + x_sq * (a3 + x_sq * a5))
    }

    let mut res = if abs_x < abs_y {
        F::FRAC_PI_2() - raw_atan_5(abs_x / abs_y)
    } else {
        raw_atan_5(abs_y / abs_x)
    };

    if x < F::zero() {
        res = F::PI() - res;
    }

    if y < F::zero() {
        res = -res;
    }

    res
}

/// Higher precision 11th degree polynomial approximation of atan function.
///
/// This function is included for reference and potential future use. It offers even higher
/// precision than the 5th degree polynomial (error < 0.001 radians) but at a higher
/// computational cost.
///
/// Polynomial coefficients from "Approximations for digital computers" by Cecil Hastings
/// and Horner's method for optimal polynomial evaluation
#[allow(dead_code)]
fn _raw_atan_11<F: Float + From<f32>>(x: F) -> F {
    let a1: F = 0.99997726f32.into();
    let a3: F = (-0.33262347f32).into();
    let a5: F = 0.19354346f32.into();
    let a7: F = (-0.11643287f32).into();
    let a9: F = 0.05265332f32.into();
    let a11: F = (-0.0117212_f32).into();

    let x_sq = x * x;
    x * (a1 + x_sq * (a3 + x_sq * (a5 + x_sq * (a7 + x_sq * (a9 + x_sq * a11)))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use proptest::prelude::*;

    const RELATIVE_ERROR: f32 = 0.1;
    const EPSILON: f32 = 0.1;

    proptest! {
        #[test]
        fn fuzzy_test_atan2(x in -1000f32..=1000., y in -1000f32..=1000.) {

            // atan2 is not defined for (0,0), skip test
            if x == 0. && y == 0. {
                return Ok(())
            }

            assert_relative_eq!(atan2(x,y), x.atan2(y), epsilon = EPSILON, max_relative = RELATIVE_ERROR);
        }
    }
}
