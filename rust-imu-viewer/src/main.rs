extern crate kiss3d;

use std::rc::Rc;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Vector3, UnitQuaternion, Point3, Point2};
use kiss3d::text::Font;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    let mut cube = window.add_cube(1.0, 1.0, 1.0);
    let origin: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
    let red: Point3<f32> = Point3::new(1.0, 0.0, 0.0);
    let green: Point3<f32> = Point3::new(0.0, 1.0, 0.0);
    let blue: Point3<f32> = Point3::new(0.0, 0.0, 1.0);

    cube.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        window.draw_line(&origin, &Point3::new(5.0, 0.0, 0.0), &red);
        window.draw_line(&origin, &Point3::new(0.0, 5.0, 0.0), &green);
        window.draw_line(&origin, &Point3::new(0.0, 0.0, 5.0), &blue);
        // c.prepend_to_local_rotation(&rot);
    }
}