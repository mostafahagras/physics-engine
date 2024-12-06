use std::ops::{Add, Mul, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec2(pub f64, pub f64);

impl Vec2 {
    pub fn from_polar(magnitude: f64, theta: f64) -> Self {
        Self(magnitude * theta.cos(), magnitude * theta.sin())
    }
    pub fn to_polar(&self) -> (f64, f64) {
        (
            (self.0 * self.0 + self.1 * self.1).sqrt(),
            self.1.atan2(self.0),
        )
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f64) -> Self::Output {
        Self(self.0 * scalar, self.1 * scalar)
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, vec: Vec2) -> Self::Output {
        Vec2(vec.0 * self, vec.1 * self)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec2;

    #[test]
    fn add() {
        let a = Vec2(0., 1.);
        let b = Vec2(1., 1.);
        let c = Vec2(1., 2.);
        assert_eq!(c, a + b);
    }

    #[test]
    fn sub() {
        let a = Vec2(0., 1.);
        let b = Vec2(1., 1.);
        let c = Vec2(1., 2.);
        assert_eq!(c - a, b);
    }

    #[test]
    fn scale() {
        let a = Vec2(1., 2.);
        let b = Vec2(2., 4.);
        assert_eq!(a * 2.0, b);
        assert_eq!(2.0 * a, b);
    }

    #[test]
    fn polar() {
        let a = Vec2(12.5, 2.4);
        let polar_a = a.to_polar();
        let cartesian_a = Vec2::from_polar(polar_a.0, polar_a.1);
        assert!((a.0 - cartesian_a.0).abs() <= 1e-12);
        assert!((a.1 - cartesian_a.1).abs() <= 1e-12);
    }
}
