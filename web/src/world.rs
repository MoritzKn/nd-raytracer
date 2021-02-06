use crate::color::Color;
use crate::Float;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Surface {
    pub(crate) color: Color,
    pub(crate) reflection: Float,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Light {
    pub(crate) color: Color,
}

#[wasm_bindgen]
impl Light {
    #[wasm_bindgen(constructor)]
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Sphere {
    pub(crate) radius: Float,
    pub(crate) surface: Surface,
}

#[wasm_bindgen]
impl Sphere {
    #[wasm_bindgen(constructor)]
    pub fn new(radius: Float, color: Color, reflection: Option<Float>) -> Self {
        Self {
            radius,
            surface: Surface {
                color,
                reflection: reflection.unwrap_or(0.0),
            },
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct World {
    pub(crate) spheres: Vec<(Vec<Float>, Sphere)>,
    pub(crate) lights: Vec<(Vec<Float>, Light)>,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            spheres: vec![],
            lights: vec![],
        }
    }

    #[wasm_bindgen]
    pub fn add_sphere(&mut self, pos: Vec<Float>, sphere: Sphere) {
        self.spheres.push((pos, sphere));
    }

    #[wasm_bindgen]
    pub fn add_light(&mut self, pos: Vec<Float>, light: Light) {
        self.lights.push((pos, light));
    }
}
