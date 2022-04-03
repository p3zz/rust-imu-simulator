mod input;
mod rotation;

use std::{error, io, thread};
use std::fs;
use std::io::Stdin;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Vector3, UnitQuaternion, Point3, Point2};
use kiss3d::scene::SceneNode;

fn main() {
    // let mut window = Window::new("Kiss3d: cube");
    // let mut cube = window.add_cube(IMU_LENGTH, IMU_HEIGHT, IMU_WIDTH);
    // let origin: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
    // let red: Point3<f32> = Point3::new(1.0, 0.0, 0.0);
    // let green: Point3<f32> = Point3::new(0.0, 1.0, 0.0);
    // let blue: Point3<f32> = Point3::new(0.0, 0.0, 1.0);
    //
    // cube.set_color(1.0, 0.0, 0.0);
    //
    // window.set_light(Light::StickToCamera);
    //
    // while window.render() {
    //     window.draw_line(&origin, &Point3::new(5.0, 0.0, 0.0), &red);
    //     window.draw_line(&origin, &Point3::new(0.0, 5.0, 0.0), &green);
    //     window.draw_line(&origin, &Point3::new(0.0, 0.0, 5.0), &blue);
    // }

    let stdin: Stdin = io::stdin();

    loop {
        let mut line = input::read_line_from_cl(&stdin);
        match input::parse_line(&line) {
            Ok(rotation) => match rotation {
                None => println!("None value"),
                Some(rot) => println!("Rotation: {}", rot)
            }
            Err(err) => println!("Error: {}", err)
        }
    }
}