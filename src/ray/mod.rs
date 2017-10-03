extern crate cgmath;

use self::cgmath::*;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    // Generate a normalized ray from origin to destination
    pub fn new(origin: Vector3<f64>, destination: Vector3<f64>) -> Ray {
        Ray {
            origin,
            direction: (destination - origin).normalize(),
        }
    }

    pub fn distance(&self, other: Vector3<f64>) -> f64 {
        (other - self.origin).magnitude() as f64
    }
}
