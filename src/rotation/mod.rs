use std::fmt::{Display, Formatter};
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::nalgebra::{Point3, UnitQuaternion, Vector3};
use kiss3d::window::Window;

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
        Rotation3D {
            pitch,
            roll,
            yaw,
        }
    }
}

impl Display for Rotation3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "pitch: {}, roll: {}, yaw: {}", &self.pitch, &self.roll, &self.yaw)
    }
}

pub struct AxisColor {
    red: Point3<f32>,
    green: Point3<f32>,
    blue: Point3<f32>,
}

impl AxisColor {
    pub fn new() -> AxisColor {
        AxisColor {
            red: Point3::new(1.0, 0.0, 0.0),
            green: Point3::new(0.0, 1.0, 0.0),
            blue: Point3::new(0.0, 0.0, 1.0),

        }
    }
}

struct GraphicLine {
    origin: Point3<f32>,
    destination: Point3<f32>,
    color: Point3<f32>,
}

impl GraphicLine {
    pub fn new(origin: Point3<f32>, destination: Point3<f32>, color: Point3<f32>) -> GraphicLine {
        GraphicLine {
            origin,
            destination,
            color,
        }
    }
}

struct GraphicAxis {
    pub x: GraphicLine,
    pub y: GraphicLine,
    pub z: GraphicLine,
}

impl GraphicAxis {
    pub fn new() -> GraphicAxis {
        let color = AxisColor::new();
        let origin: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
        let length: f32 = 10.0;
        GraphicAxis {
            x: GraphicLine::new(origin, Point3::new(length, 0.0, 0.0), color.red),
            y: GraphicLine::new(origin, Point3::new(0.0, length, 0.0), color.blue),
            z: GraphicLine::new(origin, Point3::new(0.0, 0.0, length), color.green),
        }
    }
}

/// rotate the given scene node (a cube in our case) by a given Rotation3D rotation
pub fn rotate_cube(cube: &mut SceneNode, rotation: Rotation3D) {
    let rotation_vec = Vector3::new(rotation.pitch, rotation.roll, rotation.yaw);
    let unit_rotation = UnitQuaternion::new(rotation_vec);
    cube.set_local_rotation(unit_rotation);
}

pub struct GraphicEnvironment {
    pub window: Window,
    axis: GraphicAxis,
}

impl GraphicEnvironment {
    pub fn new(window_name: &str) -> GraphicEnvironment {
        GraphicEnvironment {
            window: Window::new(window_name),
            axis: GraphicAxis::new(),
        }
    }

    pub fn init(&mut self) -> () {
        self.window.set_light(Light::StickToCamera);
    }

    pub fn draw_axis(&mut self) {
        self.window.draw_line(&self.axis.x.origin, &self.axis.x.destination, &self.axis.x.color);
        self.window.draw_line(&self.axis.y.origin, &self.axis.y.destination, &self.axis.y.color);
        self.window.draw_line(&self.axis.z.origin, &self.axis.z.destination, &self.axis.z.color);
    }
}


