#![allow(unused_variables, dead_code)]

use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn add(&self, other: &Vector3D) -> Vector3D {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Vector3D::new(x, y, z)
    }

    pub fn sub(&self, other: &Vector3D) -> Vector3D {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Vector3D::new(x, y, z)
    }

    pub fn scale(&self, scalar: f64) -> Vector3D {
        let x = self.x * scalar;
        let y = self.y * scalar;
        let z = self.z * scalar;
        Vector3D::new(x, y, z)
    }

    pub fn mult(&self, other: &Vector3D) -> Vector3D {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;
        Vector3D::new(x, y, z)
    }

    pub fn div(&self, other: &Vector3D) -> Vector3D {
        let x = self.x / other.x;
        let y = self.y / other.y;
        let z = self.z / other.z;
        Vector3D::new(x, y, z)
    }


    pub fn pow(&self, exp: f64) -> Vector3D {
        let x = self.x.powf(exp);
        let y = self.y.powf(exp);
        let z = self.z.powf(exp);
        Vector3D::new(x, y, z)
    }


    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Vector3D::new(x, y, z)
    }

    pub fn normalize(&self) -> Vector3D {
        let mag = self.magnitude();
        let x = self.x/mag;
        let y = self.y/mag;
        let z = self.z/mag;
        Vector3D::new(x, y, z)
    }
}

impl Display for Vector3D {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
       write!(f, "[ {x}, {y}, {z} ]", x = self.x, y = self.y, z = self.z)
   }

}

#[cfg(test)]
mod tests {
    use std::ops::Div;

    use super::*;

    #[test]
    fn add() {
        let v1 = Vector3D::new(1., 2., 0.);
        let v2 = Vector3D::new(3., -2., 1.);
        let v3 = Vector3D::new(4., 0., 1.);
        let v4 = Vector3D::new(4., 0., 1.);

        assert_eq!(v1.add(&v2), v3);
        assert_eq!(v2.add(&v1), v4);
    }

    #[test]
    fn sub() {
        let v1 = Vector3D::new(1., 2., 0.);
        let v2 = Vector3D::new(3., -2., 1.);
        let v3 = Vector3D::new(-2., 4., -1.);
        let v4 = Vector3D::new(2., -4., 1.);

        assert_eq!(v1.sub(&v2), v3);
        assert_eq!(v2.sub(&v1), v4);
    }

    #[test]
    fn scale() {
        let v1 = Vector3D::new(3., -2., 1.);
        let v2 = Vector3D::new(6., -4., 2.);
        let v3 = Vector3D::new(-9., 6., -3.);

        assert_eq!(v1.scale(2.), v2);
        assert_eq!(v1.scale(-3.), v3);
    }

    #[test]
    fn magnitude() {
        let v1 = Vector3D::new(1., 2., 0.);
        let v2 = Vector3D::new(3., -2., 1.);

        assert_eq!(v1.magnitude(), f64::sqrt(5.));
        assert_eq!(v2.magnitude(), f64::sqrt(14.));
    }

    #[test]
    fn dot() {
        let v1 = Vector3D::new(1., 2., 0.);
        let v2 = Vector3D::new(3., -2., 1.);
        let v3 = Vector3D::new(8., 3., -8.);
        let v4 = Vector3D::new(1., 9., 2.);

        assert_eq!(v1.dot(&v2), -1.);
        assert_eq!(v2.dot(&v1), -1.);
        assert_eq!(v3.dot(&v4), 19.);
        assert_eq!(v4.dot(&v3), 19.);
    }

    #[test]
    fn cross() {
        let v1 = Vector3D::new(1., 2., 0.);
        let v2 = Vector3D::new(3., -2., 1.);
        let v3 = Vector3D::new(2., -1., -8.);
        let v4 = Vector3D::new(-2., 1., 8.);

        assert_eq!(v1.cross(&v2), v3);
        assert_eq!(v2.cross(&v1), v4);
    }


    #[test]
    fn normalize() {
        let v1 = Vector3D::new(1., 2., 0.);
        let v2 = Vector3D::new(3., -2., 1.);
        let v3 = Vector3D::new((1.).div(f64::sqrt(5.)), (2.).div(f64::sqrt(5.)), 0.);
        let v4 = Vector3D::new((3.).div(f64::sqrt(14.)), (-2.).div(f64::sqrt(14.)), (1.).div(f64::sqrt(14.)));

        assert_eq!(v1.normalize(), v3);
        assert_eq!(v2.normalize(), v4);
    }

}
