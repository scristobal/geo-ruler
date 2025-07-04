use crate::CheapRuler;
use wasm_bindgen::prelude::*;

#[cfg(feature = "geo")]
use geo::{CoordFloat, Distance as _, Geodesic, Point, point};

#[wasm_bindgen]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Coords {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Coords {
        Coords { x, y }
    }
}

#[wasm_bindgen]
pub fn distance(a: &Coords, b: &Coords) -> f32 {
    CheapRuler::WGS84().distance(&[a.x, a.y], &[b.x, b.y])
}

#[wasm_bindgen]
pub fn bearing(a: &Coords, b: &Coords) -> f32 {
    CheapRuler::WGS84().bearing(&[a.x, a.y], &[b.x, b.y])
}

#[wasm_bindgen]
pub fn destination(a: &Coords, b: f32, d: f32) -> Coords {
    let [x, y] = CheapRuler::WGS84().destination(&[a.x, a.y], &b, &d);
    Coords { x, y }
}

#[cfg(feature = "geo")]
impl<F: CoordFloat> From<&Coords> for Point<F> {
    fn from(val: &Coords) -> Self {
        point!(x: F::from(val.x).unwrap(), y: F::from(val.y).unwrap())
    }
}

#[cfg(feature = "geo")]
impl<F: CoordFloat + Into<f32>> From<Point<F>> for Coords {
    fn from(val: Point<F>) -> Self {
        Coords {
            x: val.x().into(),
            y: val.y().into(),
        }
    }
}

#[cfg(feature = "geo")]
#[wasm_bindgen]
pub fn distance_geo(a: &Coords, b: &Coords) -> f64 {
    Geodesic.distance(a.into(), b.into())
}
