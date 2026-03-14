//! # Constants Module
//!
//! This module defines mathematical and geodetic constants used throughout the geo-ruler library.
//! These constants are primarily related to the WGS84 (World Geodetic System 1984) ellipsoid,
//! which is the reference coordinate system used by GPS and many mapping applications.

/// WGS84 semi-major axis (equatorial radius) in meters.
///
/// This is the radius of the Earth at the equator according to the WGS84 ellipsoid.
/// The value is approximately 6,378,137 meters (about 6,378 kilometers).
///
/// # References
/// - WGS84 specification: NIMA TR 8350.2
/// - Used in geodetic calculations for coordinate transformations and distance measurements
pub const WGS84_RE: f32 = 6_378_137.;

/// WGS84 eccentricity squared (first eccentricity squared).
///
/// This dimensionless parameter describes how much the WGS84 ellipsoid deviates from a perfect sphere.
/// The eccentricity squared is calculated as e² = (a² - b²) / a², where:
/// - a is the semi-major axis (equatorial radius)
/// - b is the semi-minor axis (polar radius)
///
/// A value of 0 would indicate a perfect sphere, while this value of ~0.0067 indicates
/// the Earth is slightly flattened at the poles.
///
/// # References
/// - WGS84 specification: NIMA TR 8350.2
/// - Used in geodetic calculations that account for Earth's ellipsoidal shape
pub const WGS84_E2: f32 = 0.006_694_38;
