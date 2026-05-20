use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KdNode {
    pub color: ColorName,
    pub left: Option<Box<KdNode>>,
    pub right: Option<Box<KdNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(unused)]
pub struct ColorName {
    pub hex: String,
    pub name: String,
    pub point: Point,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn coord(p: &Point, axis: usize) -> f32 {
    match axis {
        0 => p.x,
        1 => p.y,
        _ => p.z,
    }
}
