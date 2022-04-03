use std::io::Stdin;
use std::num::ParseFloatError;
use regex::Regex;
use crate::rotation::Rotation3D;

pub fn read_line_from_cl(stdin: &Stdin) -> String {
    let mut buffer = String::new();
    // read_line returns an error when a non-UTF8 character is read
    stdin.read_line(&mut buffer);
    buffer
}

pub fn parse_line(line: &String) -> Result<Option<Rotation3D>, ParseFloatError> {
    // this regexp accept a string that contains 3 numbers (int or float) separated by a comma,
    // and a new line at the end. The command line input must follow this pattern:
    // 1.0,1.0,1.0\n
    let regexp = Regex::new(r"^\d+(\.\d+)*,\d+(\.\d+)*,\d+(\.\d+)*\n$").unwrap();
    if !regexp.is_match(line) {
        return Ok(None);
    }
    let tmp = line.replace("\n", "");
    let split: Vec<&str> = tmp.split(",").collect();
    let pitch: f32 = split[0].parse()?;
    let roll: f32 = split[1].parse()?;
    let yaw: f32 = split[2].parse()?;
    Ok(Some(Rotation3D::new(pitch, roll, yaw)))
}


