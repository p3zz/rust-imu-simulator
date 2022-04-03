use std::fmt::{Display, Formatter};
use kiss3d::scene::SceneNode;
use kiss3d::nalgebra::{UnitQuaternion, Vector3};

const IMU_HEIGHT: f32 = 0.5;
const IMU_WIDTH: f32 = 1.0;
const IMU_LENGTH: f32 = 1.0;

/// A struct that describe a 3D rotation on xyz axis, where pitch is on x axis,
/// roll on y axis and yaw on z axis
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

fn rotate_cube(cube: &mut SceneNode, rotation: Rotation3D) {
    let pitch_rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), rotation.pitch);
    let roll_rot = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), rotation.roll);
    let yaw_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), rotation.yaw);

    cube.prepend_to_local_rotation(&pitch_rot);
    cube.prepend_to_local_rotation(&roll_rot);
    cube.prepend_to_local_rotation(&yaw_rot);
}


