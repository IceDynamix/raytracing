use crate::math::Scalar;
use crate::ppm::Pixel;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vector3 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Vector3 {
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Vector3 {
        Vector3 { x, y, z }
    }

    #[inline]
    pub fn scale(&self, scalar: Scalar) -> Vector3 {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn normalized(&self) -> Vector3 {
        self.scale(self.euclidean_norm().recip())
    }

    #[inline]
    pub fn euclidean_norm(&self) -> Scalar {
        self.dot_product(self).sqrt()
    }

    #[inline]
    pub fn dot_product(&self, rhs: &Vector3) -> Scalar {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub const ZERO: Vector3 = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x.add(rhs.x), self.y.add(rhs.y), self.z.add(rhs.z))
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x.sub(rhs.x), self.y.sub(rhs.y), self.z.sub(rhs.z))
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x.mul(rhs.x), self.y.mul(rhs.y), self.z.mul(rhs.z))
    }
}

impl Div for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x.div(rhs.x), self.y.div(rhs.y), self.z.div(rhs.z))
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        self.scale(-1.)
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Vector3::ZERO
    }
}
impl From<Pixel> for Vector3 {
    fn from(p: Pixel) -> Self {
        Vector3::new(p.r as f64, p.g as f64, p.b as f64)
    }
}
