#![cfg_attr(feature = "cargo-clippy", allow(borrowed_box))]

extern crate image;
extern crate cgmath;

use self::cgmath::*;

use std::f64;

mod view_window;
mod configuration;

use self::configuration::Configuration;
use light::Light;
use camera::Camera;
use color::Color;
use object::*;
use pixel_buffer::PixelBuffer;
use ray::Ray;
use self::view_window::ViewWindow;

pub struct Scene {
    camera: Camera,
    scene_contents: SceneContents,
    scene_characteristics: SceneCharacteristics,
    pixel_buffer: PixelBuffer,
    view_window: ViewWindow,
}

struct SceneContents {
    lights: Vec<Box<Light>>,
    shapes: Vec<Shape>,
}

struct SceneCharacteristics {
    ambient_coefficient: f64,
    diffuse_coefficient: f64,
    specular_coefficient: f64,
}

struct RayHit<'a> {
    shape: &'a Shape,
    intersection: Vector3<f64>,
    distance: f64,
}

impl Scene {
    pub fn new(configuration_filename: String) -> Scene {
        let mut configuration: Configuration =
            Configuration::read_configuration(configuration_filename);

        /* Set up lights */
        let mut lights: Vec<Box<Light>> = Vec::new();
        for light_definition in &configuration.lights {
            lights.push(Box::new(light_definition.as_light()));
        }

        /*  Set up objects */
        let mut shapes: Vec<Shape> = Vec::new();
        for object_definition in &configuration.objects {
            shapes.append(&mut (object_definition.read_shapes()));
        }

        /* Set up camera and view window */
        let camera: Camera = configuration.camera();
        let view_window_position: Vector3<f64> = camera.origin +
            (camera.target - camera.origin).normalize() * configuration.viewport_distance;

        Scene {
            scene_contents: SceneContents { lights, shapes },
            scene_characteristics: SceneCharacteristics {
                ambient_coefficient: configuration.ambient_coefficient,
                diffuse_coefficient: 1f64 - configuration.ambient_coefficient,
                specular_coefficient: configuration.specular_coefficient,
            },
            camera,
            pixel_buffer: PixelBuffer::new(configuration.width, configuration.height),
            view_window: ViewWindow::new(
                configuration.width,
                configuration.height,
                configuration.viewport_width,
                view_window_position,
            ),
        }
    }

    // must find closest intersection
    fn closest_intersection(&self, ray: &Ray, this_object: Option<Shape>) -> Option<RayHit> {
        let mut result: Option<RayHit> = None;
        let mut shortest_distance: f64 = f64::MAX;

        for shape in &self.scene_contents.shapes {
            if let Some(this_shape) = this_object {
                if *shape == this_shape {
                    continue;
                }
            }

            if let Some(intersection) = shape.intersect(ray) {
                let distance: f64 = (intersection - ray.origin).magnitude();
                if shortest_distance > distance {
                    shortest_distance = distance;

                    result = Some(RayHit {
                        shape,
                        intersection,
                        distance,
                    });
                }
            }
        }

        result
    }

    fn shadow(&self, ray_hit: &RayHit, to_light: &Ray) -> bool {
        if let Some(shadow_hit) = self.closest_intersection(to_light, Some(*ray_hit.shape)) {
            true
        } else {
            false
        }
    }

    fn light(&self, ray: &Ray, ray_hit: &RayHit) -> Color {
        let shape_color: Color = ray_hit.shape.color();

        let mut result: Color = shape_color * self.scene_characteristics.ambient_coefficient;

        for light in &self.scene_contents.lights {
            let mut to_light: Ray = Ray::new(ray_hit.intersection, light.origin);
            to_light.origin += to_light.direction * 0.00001;

            if self.shadow(ray_hit, &to_light) {
                continue;
            }

            let mut normal: Vector3<f64> = ray_hit.shape.normal(
                ray_hit.intersection,
                self.camera.orientation_vector(),
            );

            let shade: f64 = to_light.direction.dot(normal);

            if shade > 0f64 {
                result = Color::new(100f64, 100f64, 100f64) *
                    f64::max(0f64, to_light.direction.dot(ray.reflection(normal)))
                        .powf(self.scene_characteristics.specular_coefficient) +
                    shape_color * self.scene_characteristics.diffuse_coefficient * shade +
                    shape_color * self.scene_characteristics.ambient_coefficient;
            }
        }

        result
    }

    fn trace(&mut self, ray: &Ray) -> Option<Color> {
        match self.closest_intersection(ray, None) {
            Some(ray_hit) => Some(self.light(ray, &ray_hit)),
            None => None,
        }
    }

    pub fn draw(&mut self) {
        for x in 0..self.view_window.pixel_width {
            for y in 0..self.view_window.pixel_height {
                let mut ray: Ray = Ray::new(self.camera.origin, self.view_window.at(x, y));

                if let Some(color) = self.trace(&ray) {
                    self.pixel_buffer.set_pixel(x, y, color);
                }
            }
        }

        self.pixel_buffer.save_image("img/scene.png").unwrap();
    }
}
