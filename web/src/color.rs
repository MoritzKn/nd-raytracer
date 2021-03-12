use ndrt_lib::Float;

use wasm_bindgen::prelude::*;

pub type ColorInt = [u8; 4];

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub(crate) array: [Float; 4],
}

#[wasm_bindgen]
impl Color {
    pub fn rgba(r: Float, g: Float, b: Float, a: Float) -> Self {
        Self {
            array: [r, g, b, a],
        }
    }

    pub fn rgb(r: Float, g: Float, b: Float) -> Self {
        Self {
            array: [r, g, b, 1.0],
        }
    }

    pub fn red(&self) -> Float {
        self.array[0]
    }

    pub fn green(&self) -> Float {
        self.array[1]
    }

    pub fn blue(&self) -> Float {
        self.array[2]
    }

    pub fn alpha(&self) -> Float {
        self.array[3]
    }

    pub(crate) fn from_int(slice: &ColorInt) -> Self {
        Self {
            array: [
                (slice[0] as Float / 255.0),
                (slice[1] as Float / 255.0),
                (slice[2] as Float / 255.0),
                (slice[3] as Float / 255.0),
            ],
        }
    }

    pub(crate) fn to_int(&self) -> ColorInt {
        [
            (self.array[0] * 255.0) as u8,
            (self.array[1] * 255.0) as u8,
            (self.array[2] * 255.0) as u8,
            (self.array[3] * 255.0) as u8,
        ]
    }

    pub fn normalize(&self) -> Color {
        let len = Float::sqrt(
            self.red() * self.red() + self.green() * self.green() + self.blue() * self.blue(),
        );

        Self::rgba(
            self.red() * len,
            self.green() * len,
            self.blue() * len,
            self.alpha(),
        )
    }

    pub fn apply(&mut self, top: &Color) {
        let alpha = top.alpha();
        let invert = 1.0 - alpha;

        self.array[0] = self.red() * invert + self.red() * top.red() * alpha;
        self.array[1] = self.green() * invert + self.green() * top.green() * alpha;
        self.array[2] = self.blue() * invert + self.blue() * top.blue() * alpha;
    }

    pub fn mix(&mut self, top: &Color) {
        let alpha = top.alpha();
        let invert = 1.0 - alpha;

        self.array[0] = self.red() * invert + top.red() * alpha;
        self.array[1] = self.green() * invert + top.green() * alpha;
        self.array[2] = self.blue() * invert + top.blue() * alpha;
    }

    pub fn combine(&mut self, top: &Color) {
        self.array[0] = self.red() + top.red() * top.alpha();
        self.array[1] = self.green() + top.green() * top.alpha();
        self.array[2] = self.blue() + top.blue() * top.alpha();
    }

    pub fn adjust_brightness(&mut self, brightness: Float) {
        self.array[0] = self.red() * brightness;
        self.array[1] = self.green() * brightness;
        self.array[2] = self.blue() * brightness;
    }

    pub fn set_alpha(&mut self, alpha: Float) {
        self.array[3] = alpha;
    }

    pub fn difference(&self, other: &Color) -> Float {
        let rd = self.red() - other.red();
        let gd = self.green() - other.green();
        let bd = self.blue() - other.blue();

        Float::sqrt(rd * rd + gd * gd + bd * bd)
    }
}
