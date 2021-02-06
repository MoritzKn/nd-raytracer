use crate::Float;

pub trait Vector:
    Sized
    + Copy
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

    fn length(&self) -> Float {
        let sum_of_squares = self
            .components()
            .iter()
            .map(|c| c * c)
            .fold(0.0, |a, b| a + b);

        Float::sqrt(sum_of_squares)
    }

    fn normalize(&self) -> Self {
        let len = self.length();

        self.div_scalar(len)
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
