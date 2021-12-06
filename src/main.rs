mod convex_hull;
mod geometry;
mod ply_file;
mod soil_removal;
mod to_json;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let ply = ply_file::read_ply_file(get_input_file_path());
    let input_data = match ply_file::ply_to_pts(ply) {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err.as_str());
            return;
        }
    };

    to_json::dump_to_json(&input_data);

    println!("points in input ply: {}", input_data.len());
    for pt in &input_data {
        //println!("{}", pt);
    }
    let soil_removed = soil_removal::remove_soil_simple(input_data);
    println!("soil removed len: {}", soil_removed.len());
    ply_file::write_to_ply_file(String::from("soil_removed.ply"), &soil_removed);

    println!("2d convex hull");
    let hull = convex_hull::convex_hull(soil_removed);
    let area = convex_hull::convex_hull_area(&hull);
    println!("area: {}", area);
    ply_file::write_to_ply_file(String::from("output.ply"), &hull);
}

fn get_input_file_path() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() >= 2 {
        args.get(1).unwrap().clone()
    } else {
        String::from("data/lettuce.ply")
    };
    println!("path: {}", path);
    path
}
