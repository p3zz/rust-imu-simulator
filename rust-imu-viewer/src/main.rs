mod input;
mod rotation;

use std::{error, io, thread};
use std::fs;
use std::io::Stdin;
use crossbeam_channel::{bounded, select, TryRecvError, unbounded};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Vector3, UnitQuaternion, Point3, Point2};
use kiss3d::scene::SceneNode;
use crate::rotation::{IMU_HEIGHT, IMU_LENGTH, IMU_WIDTH, rotate_cube};

fn main() {
    let mut graphic_env = rotation::GraphicEnvironment::new("IMU rotation viewer");
    graphic_env.init();
    let mut cube = graphic_env.window.add_cube(IMU_LENGTH, IMU_HEIGHT, IMU_WIDTH);
    cube.set_color(1.0, 0.0, 0.0);

    let (tx, rx) = bounded(0);
    input::spawn_read_line_thread(tx);

    while graphic_env.window.render() {
        graphic_env.draw_axis();
        let rotation = match rx.try_recv() {
            Ok(msg) => match msg {
                None => {
                    println!("invalid input line");
                    continue;
                }
                Some(rot) => rot
            },
            Err(e) => continue
        };
        rotate_cube(&mut cube, rotation);
    }
}