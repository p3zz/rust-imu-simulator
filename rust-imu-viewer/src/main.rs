extern crate kiss3d;

use std::rc::Rc;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Vector3, UnitQuaternion, Point3, Point2};
use kiss3d::text::Font;

const IMU_HEIGHT: f32 = 0.5;
const IMU_WIDTH: f32 = 1.0;
const IMU_LENGTH: f32 = 1.0;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    let mut c = window.add_cube(IMU_LENGTH, IMU_HEIGHT, IMU_WIDTH);

    c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        // c.prepend_to_local_rotation(&rot);
    }
}