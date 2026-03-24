use serde::{Deserialize};
use std::fs::File;
use std::io::BufReader;
#[derive(Debug, Deserialize)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Deserialize, Debug)]
pub struct ObjData {
    pub mass: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub position_x: f32,
    pub position_y: f32,
    pub radius: f32,
    pub color: Rgba,
}

pub fn parse_data() -> Vec<ObjData>{
    let file_not_oriented = File::open("assets/sun_system.json");
    match file_not_oriented {
        Ok(file) => { 
            println!("File opened successfully!");
            let reader = BufReader::new(file);
            let objs: Vec<ObjData> = serde_json::from_reader(reader).expect("Failed to parse JSON data");
            return objs 
        },
        Err(err) => {
            panic!("There was a problem opening the data file of {:?}: {:?}", "sun_system.json",err )
        } 
    }

}