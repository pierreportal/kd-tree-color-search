use super::super::{ColorName, KdNode, Point, Rgb, coord};

use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter};

pub struct KdTree;

impl KdTree {
    pub fn new() -> Self {
        Self
    }
    pub fn load_tree(&self, binfile_name: &str) -> Option<Box<KdNode>> {
        match File::open(binfile_name) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let tree: Option<Box<KdNode>> =
                    bincode::deserialize_from(reader).expect("failed to deserialize tree");
                tree
            }
            Err(_) => {
                let colors = parse_colornames_file("magpie/color_names.txt").unwrap();
                let tree = self.build_kdtree(colors, 0);
                let file = File::create(binfile_name).unwrap();
                let writer = BufWriter::new(file);

                bincode::serialize_into(writer, &tree).expect("failed to serialize tree");
                tree
            }
        }
    }

    pub fn build_kdtree(&self, mut items: Vec<ColorName>, depth: usize) -> Option<Box<KdNode>> {
        if items.is_empty() {
            return None;
        }
        let axis = depth % 3;
        items.sort_by(|a, b| {
            coord(&a.point, axis)
                .partial_cmp(&coord(&b.point, axis))
                .unwrap()
        });
        let mid = items.len() / 2;
        let median = items[mid].clone();
        let left = items[..mid].to_vec();
        let right = if mid + 1 < items.len() {
            items[mid + 1..].to_vec()
        } else {
            vec![]
        };

        Some(Box::new(KdNode {
            color: median,
            left: self.build_kdtree(left, depth + 1),
            right: self.build_kdtree(right, depth + 1),
        }))
    }
}

fn hex_to_rgb(hex: &str) -> Rgb {
    let hex = hex.strip_prefix('#').unwrap();

    let (r, g, b) = (
        u8::from_str_radix(&hex[0..2], 16).unwrap(),
        u8::from_str_radix(&hex[2..4], 16).unwrap(),
        u8::from_str_radix(&hex[4..6], 16).unwrap(),
    );

    Rgb { r, g, b }
}

fn is_valid_hex_color(s: &str) -> bool {
    let hex = s.strip_prefix('#').unwrap_or(s);
    hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn parse_colornames_file(filename: &str) -> io::Result<Vec<ColorName>> {
    let mut color_list: Vec<ColorName> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let delimiter = " = ";

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(delimiter).collect();
        if parts.len() == 2 {
            let name = parts[0].to_string();
            let hex = parts[1].to_string();
            if is_valid_hex_color(&hex) && name.len() > 0 {
                let Rgb { r, g, b } = hex_to_rgb(&hex);
                let point = Point {
                    x: r as f32 / 255.0,
                    y: g as f32 / 255.0,
                    z: b as f32 / 255.0,
                };

                let color = ColorName { hex, name, point };
                color_list.push(color);
            }
        }
    }

    Ok(color_list)
}
