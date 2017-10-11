extern crate cgmath;

use self::cgmath::*;

pub struct Camera {
    pub origin: Vector3<f64>,
    pub target: Vector3<f64>,
}

impl Camera {
    pub fn new(origin: Vector3<f64>, target: Vector3<f64>) -> Camera {
        Camera { origin, target }
    }
}
