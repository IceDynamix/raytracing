use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn scale(&self, scalar: f64) -> Vector2 {
        Vector2::new(self.x * scalar, self.y * scalar)
    }

    pub fn euclidean_norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dot_product(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x.add(rhs.x), self.y.add(rhs.y))
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x.sub(rhs.x), self.y.sub(rhs.y))
    }
}
