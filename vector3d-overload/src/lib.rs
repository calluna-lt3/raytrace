#![allow(unused_variables, dead_code)]

// NOTE: should jsut take non reference version for overloading, and then clone the interior
// values and do not mutate
// => this doesn't work bc the value is still moved inside, unless i derive Copy?
//
use std::ops::{Add, Sub, Mul};

#[derive(Debug, PartialEq)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
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


impl Add for Vector3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Self { x, y, z }
    }
}

impl Sub for &Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Self) -> Vector3D {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Vector3D::new(x, y, z)
    }
}

impl Mul for &Vector3D {
    type Output = Vector3D;

    fn mul(self, other: Self) -> Vector3D {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;
        Vector3D::new(x, y, z)
    }
}

impl Mul<f64> for &Vector3D {
    type Output =  Vector3D;

    fn mul(self, other: f64) -> Vector3D {
        let x = self.x * other;
        let y = self.y * other;
        let z = self.z * other;
        Vector3D::new(x, y, z)
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

        assert_eq!(v1 + v2, v3);
    }

    #[test]
    fn sub() {
        let v1 = Vector3D::new(1., 2., 0.);
        let v2 = Vector3D::new(3., -2., 1.);
        let v3 = Vector3D::new(-2., 4., -1.);

        assert_eq!(&v1 - &v2, v3);
    }

    #[test]
    fn mult() {
        let v1 = Vector3D::new(1., 2., 0.);
        let v2 = Vector3D::new(3., -2., 1.);
        let v3 = Vector3D::new(3., -4., 0.);

        assert_eq!(&v1 * &v2, v3);
    }

    #[test]
    fn scale() {
        let v1 = Vector3D::new(3., -2., 1.);
        let v2 = Vector3D::new(6., -4., 2.);
        let v3 = Vector3D::new(-9., 6., -3.);

        assert_eq!(&v1 * 2., v2);
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
