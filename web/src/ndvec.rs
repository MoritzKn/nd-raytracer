use crate::vector::Vector;
use crate::Float;

#[derive(Clone, Copy, Debug)]
pub struct NdVec<const L: usize> {
    components: [Float; L],
}

impl<const L: usize> Vector for NdVec<L> {
    fn new() -> Self {
        Self {
            components: [0.0; L],
        }
    }

    #[inline]
    fn from_iter(iter: impl Iterator<Item = Float>) -> Self {
        let mut components = [0.0; L];

        for (comp, value) in components.iter_mut().zip(iter) {
            *comp = value;
        }

        Self { components }
    }

    fn pad(base: &[Float], default: Float) -> Self {
        let mut components = [default; L];

        for (comp, value) in components.iter_mut().zip(base.iter()) {
            *comp = *value;
        }

        Self { components }
    }

    #[inline]
    fn components(&self) -> &[Float] {
        &self.components
    }
}

impl<const L: usize> std::ops::Add<NdVec<L>> for NdVec<L> {
    type Output = NdVec<L>;

    fn add(self, other: NdVec<L>) -> NdVec<L> {
        Vector::add(&self, &other)
    }
}

impl<const L: usize> std::ops::Sub<NdVec<L>> for NdVec<L> {
    type Output = NdVec<L>;

    fn sub(self, other: NdVec<L>) -> NdVec<L> {
        Vector::sub(&self, &other)
    }
}

impl<const L: usize> std::ops::Mul<NdVec<L>> for NdVec<L> {
    type Output = NdVec<L>;

    fn mul(self, other: NdVec<L>) -> NdVec<L> {
        Vector::mul(&self, &other)
    }
}

impl<const L: usize> std::ops::Div<NdVec<L>> for NdVec<L> {
    type Output = NdVec<L>;

    fn div(self, other: NdVec<L>) -> NdVec<L> {
        Vector::div(&self, &other)
    }
}

impl<const L: usize> std::ops::Add<Float> for NdVec<L> {
    type Output = NdVec<L>;

    fn add(self, other: Float) -> NdVec<L> {
        Vector::add_scalar(&self, other)
    }
}

impl<const L: usize> std::ops::Sub<Float> for NdVec<L> {
    type Output = NdVec<L>;

    fn sub(self, other: Float) -> NdVec<L> {
        Vector::sub_scalar(&self, other)
    }
}

impl<const L: usize> std::ops::Mul<Float> for NdVec<L> {
    type Output = NdVec<L>;

    fn mul(self, other: Float) -> NdVec<L> {
        Vector::mul_scalar(&self, other)
    }
}

impl<const L: usize> std::ops::Div<Float> for NdVec<L> {
    type Output = NdVec<L>;

    fn div(self, other: Float) -> NdVec<L> {
        Vector::div_scalar(&self, other)
    }
}
