use std::fmt::{Display, Formatter};
use kiss3d::scene::SceneNode;
use kiss3d::nalgebra::{UnitQuaternion, Vector3};

pub const IMU_HEIGHT: f32 = 0.5;
pub const IMU_WIDTH: f32 = 1.0;
pub const IMU_LENGTH: f32 = 1.0;

/// A struct that describe a 3D rotation on xyz axis, where pitch is on x axis,
/// roll on y axis and yaw on z axis. Angles unit is radians/s
pub struct Rotation3D {
    pitch: f32,
    roll: f32,
    yaw: f32,
}

impl Rotation3D {
    pub fn new(pitch: f32, roll: f32, yaw: f32) -> Rotation3D {
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


/// rotate the given scene node (a cube in our case) by a given Rotation3D rotation
pub fn rotate_cube(cube: &mut SceneNode, rotation: Rotation3D) {
    let rotation_vec = Vector3::new(rotation.pitch, rotation.roll, rotation.yaw);
    let unit_rotation = UnitQuaternion::new(rotation_vec);
    cube.set_local_rotation(unit_rotation);
}


