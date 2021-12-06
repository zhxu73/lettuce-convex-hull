use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Copy for Point {}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }
    #[allow(dead_code)]
    pub fn new2d(x: f64, y: f64) -> Self {
        Point { x, y, z: 0.0 }
    }
    #[allow(dead_code)]
    pub fn new2d_int(x: i32, y: i32) -> Self {
        Point {
            x: x as f64,
            y: y as f64,
            z: 0.0,
        }
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point::new(self.x, self.y, self.z)
    }
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub fn print_pts(data: &[Point]) {
    for pt in data {
        println!("{}", pt);
    }
}

pub fn sort_points_by_x(data: &mut Vec<Point>) {
    data.sort_by(|pt1, pt2| pt1.x.partial_cmp(&pt2.x).unwrap());
}

#[allow(dead_code)]
pub fn sort_points_by_y(data: &mut Vec<Point>) {
    data.sort_by(|pt1, pt2| pt1.y.partial_cmp(&pt2.y).unwrap());
}

#[allow(dead_code)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}
