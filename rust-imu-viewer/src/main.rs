extern crate kiss3d;

use std::{io, thread};
use std::fmt::{Display, Error, Formatter};
use std::fs;
use std::io::Stdin;
use std::rc::Rc;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Vector3, UnitQuaternion, Point3, Point2};
use kiss3d::scene::SceneNode;
use kiss3d::text::Font;

const IMU_HEIGHT: f32 = 0.5;
const IMU_WIDTH: f32 = 1.0;
const IMU_LENGTH: f32 = 1.0;

/// A struct that describe a 3D rotation on xyz axis, where pitch is on x axis,
/// roll on y axis and yaw on z axis
struct Rotation3D {
    pitch: f32,
    roll: f32,
    yaw: f32,
}

impl Rotation3D {
    fn new(pitch: f32, roll: f32, yaw: f32) -> Rotation3D {
        return Rotation3D {
            pitch,
            roll,
            yaw,
        };
    }
}

impl Display for Rotation3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "pitch: {}, roll: {}, yaw: {}", &self.pitch, &self.roll, &self.yaw)
    }
}

fn read_line_from_cl(stdin: &Stdin) -> String {
    let mut buffer = String::new();
    // read_line returns an error when a non-UTF8 character is read
    stdin.read_line(&mut buffer);
    return buffer;
}

fn parse_line(line: &String) -> Result<Rotation3D, String> {
    let split: Vec<&str> = line.split(",").collect();
    for char in split {
        println!("{}", char);
    }
    return Ok(Rotation3D::new(0.0, 0.0, 0.0));
}

fn rotate_cube(cube: &mut SceneNode, rotation: Rotation3D) {
    let pitch_rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), rotation.pitch);
    let roll_rot = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), rotation.roll);
    let yaw_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), rotation.yaw);

    cube.prepend_to_local_rotation(&pitch_rot);
    cube.prepend_to_local_rotation(&roll_rot);
    cube.prepend_to_local_rotation(&yaw_rot);
}


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
        let line = read_line_from_cl(&stdin);
        println!("{}", line);
    }
}