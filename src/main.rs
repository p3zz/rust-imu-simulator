mod input;
mod rotation;

use crossbeam_channel::bounded;
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
            Err(_) => continue
        };
        rotate_cube(&mut cube, rotation);
    }
}