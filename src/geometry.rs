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

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}
impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Triangle { p1, p2, p3 }
    }
}

pub fn triangles_to_pts(data: Vec<Triangle>) -> Vec<Point> {
    let mut result = vec![];
    result.reserve(data.len() * 3);
    for tri in data {
        result.push(tri.p1);
        result.push(tri.p2);
        result.push(tri.p3);
    }
    result
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub p1: Point,
    pub p2: Point,
}
impl Edge {
    pub fn new(p1: Point, p2: Point) -> Self {
        Edge { p1, p2 }
    }
}

impl PartialEq<Self> for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.p1 == other.p1 && self.p2 == other.p2) || (self.p2 == other.p1 && self.p1 == other.p2)
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3D { x, y, z }
    }
}
