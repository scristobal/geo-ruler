use crate::CheapRuler;
use wasm_bindgen::prelude::*;

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

    pub fn distance(&self, rhs: &Coords) -> f32 {
        CheapRuler::WGS84().distance(&[self.x, self.y], &[rhs.x, rhs.y])
    }

    pub fn bearing(&self, rhs: &Coords) -> f32 {
        CheapRuler::WGS84().bearing(&[self.x, self.y], &[rhs.x, rhs.y])
    }

    pub fn destination(&self, bearing: f32, distance: f32) -> Coords {
        let [x, y] = CheapRuler::WGS84().destination(&[self.x, self.y], &bearing, &distance);
        Coords { x, y }
    }
}
