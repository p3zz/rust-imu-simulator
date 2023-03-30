use std::io::Stdin;
use std::num::ParseFloatError;
use std::{io, thread};
use regex::Regex;
use crate::rotation::Rotation3D;
use crossbeam_channel::{Sender};

pub fn spawn_read_line_thread(sender: Sender<Option<Rotation3D>>) {
    let stdin: Stdin = io::stdin();
    thread::spawn(move || loop {
        let line : String = read_line_from_cl(&stdin).unwrap_or(String::from(""));
        match parse_line(&line) {
            Ok(rotation) => sender.send(rotation).unwrap(),
            Err(e) => println!("{}", e)
        }        
    });
}

/// read a line from the given stdin, then returns it. Being the read_line a blocking function,
/// don't use this function in a render loop
pub fn read_line_from_cl(stdin: &Stdin) -> Option<String> {
    let mut buffer = String::new();
    // read_line returns an error when a non-UTF8 character is read
    match stdin.read_line(&mut buffer){
        Ok(_) => Some(buffer),
        Err(_) => None
    }
}

/// parse the given line and returns a Rotation3D struct it there are no errors.
/// It first checks if the line match the correct pattern through a regexp, then proceed to split
/// the line in the 3 rotation angles (pitch, roll, yaw). If the line doesn't match the pattern,
/// the function returns an Ok(None) result
pub fn parse_line(line: &String) -> Result<Option<Rotation3D>, ParseFloatError> {
    // this regexp accept a string that contains 3 numbers (int or float) separated by a comma,
    // and a new line at the end on the string. The command line input must follow this pattern:
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


