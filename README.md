# IMU 3D Viewer
A Rust project for real-time imu simulation

# Description
The IMU is shown as a 3D red cube and rotates of a given rotation (pitch, roll, yaw), in radians, read from stdin.

The correct input format is: *pitch*,*roll*,*yaw*\n


# Build
```bash
cargo build
```

## Run
```bash
node tools/simulator.js | cargo run
```