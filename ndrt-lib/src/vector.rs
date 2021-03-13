use crate::Float;
use std::fmt::Display;

fn fast_inv_sqrt32(n: f32) -> f32 {
    // Magic number based on Chris Lomont work:
    // const MAGIC_U32: u32 = 0x5f375a86;
    // The Original Magic Number:
    // const MAGIC_32: u32 = 0x5f3759df;
    const THREEHALFS: f32 = 1.5f32;
    let x2: f32 = n * 0.5f32;
    let mut i: u32 = unsafe { std::mem::transmute(n) }; // evil floating point bit level hacking
    i = 0x5f375a86 - (i >> 1); // what the fuck?
    let y: f32 = unsafe { std::mem::transmute(i) };
    let y = y * (THREEHALFS - (x2 * y * y)); // 1st iteration

    // y  = y * ( THREEHALFS - ( x2 * y * y ) ); // 2nd iteration, this can be remove

    y
}

pub trait Vector:
    Sized
    + Copy
    + Display
    + std::ops::Mul<Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Mul<Float, Output = Self>
    + std::ops::Add<Float, Output = Self>
    + std::ops::Sub<Float, Output = Self>
    + std::ops::Div<Float, Output = Self>
{
    fn new() -> Self;

    fn from_iter(iter: impl Iterator<Item = Float>) -> Self;

    fn pad(base: &[Float], default: Float) -> Self;

    fn components(&self) -> &[Float];

    fn sum_of_squares(&self) -> Float {
        self.components()
            .iter()
            .map(|c| c * c)
            .fold(0.0, |a, b| a + b)
    }

    fn length(&self) -> Float {
        Float::sqrt(self.sum_of_squares())
    }

    fn normalize(&self) -> Self {
        self.mul_scalar(fast_inv_sqrt32(self.sum_of_squares()))
    }

    fn dot(&self, other: &Self) -> Float {
        self.components()
            .iter()
            .zip(other.components().iter())
            .map(|(a, b)| a * b)
            .fold(0.0, |a, b| a + b)
    }

    fn add(&self, other: &Self) -> Self {
        Self::from_iter(
            self.components()
                .iter()
                .zip(other.components().iter())
                .map(|(a, b)| a + b),
        )
    }

    fn sub(&self, other: &Self) -> Self {
        Self::from_iter(
            self.components()
                .iter()
                .zip(other.components().iter())
                .map(|(a, b)| a - b),
        )
    }

    fn mul(&self, other: &Self) -> Self {
        Self::from_iter(
            self.components()
                .iter()
                .zip(other.components().iter())
                .map(|(a, b)| a * b),
        )
    }

    fn div(&self, other: &Self) -> Self {
        Self::from_iter(
            self.components()
                .iter()
                .zip(other.components().iter())
                .map(|(a, b)| a / b),
        )
    }

    fn add_scalar(&self, other: Float) -> Self {
        Self::from_iter(self.components().iter().map(|a| a + other))
    }

    fn sub_scalar(&self, other: Float) -> Self {
        Self::from_iter(self.components().iter().map(|a| a - other))
    }

    fn mul_scalar(&self, other: Float) -> Self {
        Self::from_iter(self.components().iter().map(|a| a * other))
    }

    fn div_scalar(&self, other: Float) -> Self {
        Self::from_iter(self.components().iter().map(|a| a / other))
    }
}
