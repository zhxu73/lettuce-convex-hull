use crate::geometry::Point;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct VecPts(Vec<Point>);

pub fn dump_to_json(pts: &Vec<Point>) {
    println!("rebased and dump to json");

    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    for pt in pts {
        if pt.x < min_x {
            min_x = pt.x
        }
        if pt.y < min_y {
            min_y = pt.y
        }
    }
    println!("{}, {}", min_x, min_y);

    let mut rebased = vec![];
    for pt in pts {
        let new_pt = Point {
            x: pt.x - min_x,
            y: pt.y - min_y,
            z: pt.z,
        };
        rebased.push(new_pt);
    }

    let foo: VecPts = VecPts { 0: rebased };
    let json_str = serde_json::to_string(&foo).unwrap();

    let path = Path::new("input.json");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(json_str.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
