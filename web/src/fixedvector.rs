use crate::vector::Vector;
use crate::Float;

#[derive(Clone, Copy, Debug)]
pub struct FixedVector<const L: usize> {
    components: [Float; L],
}

impl<const L: usize> Vector for FixedVector<L> {
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

impl<const L: usize> std::ops::Add<FixedVector<L>> for FixedVector<L> {
    type Output = FixedVector<L>;

    fn add(self, other: FixedVector<L>) -> FixedVector<L> {
        Vector::add(&self, &other)
    }
}

impl<const L: usize> std::ops::Sub<FixedVector<L>> for FixedVector<L> {
    type Output = FixedVector<L>;

    fn sub(self, other: FixedVector<L>) -> FixedVector<L> {
        Vector::sub(&self, &other)
    }
}

impl<const L: usize> std::ops::Mul<FixedVector<L>> for FixedVector<L> {
    type Output = FixedVector<L>;

    fn mul(self, other: FixedVector<L>) -> FixedVector<L> {
        Vector::mul(&self, &other)
    }
}

impl<const L: usize> std::ops::Div<FixedVector<L>> for FixedVector<L> {
    type Output = FixedVector<L>;

    fn div(self, other: FixedVector<L>) -> FixedVector<L> {
        Vector::div(&self, &other)
    }
}

impl<const L: usize> std::ops::Add<Float> for FixedVector<L> {
    type Output = FixedVector<L>;

    fn add(self, other: Float) -> FixedVector<L> {
        Vector::add_scalar(&self, other)
    }
}

impl<const L: usize> std::ops::Sub<Float> for FixedVector<L> {
    type Output = FixedVector<L>;

    fn sub(self, other: Float) -> FixedVector<L> {
        Vector::sub_scalar(&self, other)
    }
}

impl<const L: usize> std::ops::Mul<Float> for FixedVector<L> {
    type Output = FixedVector<L>;

    fn mul(self, other: Float) -> FixedVector<L> {
        Vector::mul_scalar(&self, other)
    }
}

impl<const L: usize> std::ops::Div<Float> for FixedVector<L> {
    type Output = FixedVector<L>;

    fn div(self, other: Float) -> FixedVector<L> {
        Vector::div_scalar(&self, other)
    }
}
