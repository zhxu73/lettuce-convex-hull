mod chan;
mod jarvis_march;
mod volume;

use crate::geometry::{Point, Triangle};

pub fn convex_hull(mut data: Vec<Point>) -> Vec<Triangle> {
    chan::run(data)
}
