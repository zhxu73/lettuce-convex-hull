use crate::convex_hull_2d::benchmark::run_benchmark;
use crate::geometry::{print_pts, sort_points_by_x, Point};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn convex_hull(mut data: Vec<Point>) -> Vec<Point> {
    // sort by x
    sort_points_by_x(&mut data);
    const SUB_HULL_COUNT: usize = 1200;
    let result = chan_algo(&data, SUB_HULL_COUNT);
    println!("CH pt count: {}", result.len());
    // run_benchmark(&data);
    print_pts(&result);
    result
}

fn chan_sub_hull_range(
    data_len: usize,
    sub_hull_index: usize,
    sub_hull_count: usize,
) -> (usize, usize) {
    let start = sub_hull_index * data_len / sub_hull_count;
    let end = match (sub_hull_index + 1) * data_len / sub_hull_count < data_len {
        true => (sub_hull_index + 1) * data_len / sub_hull_count,
        false => data_len,
    };
    (start, end)
}

#[allow(dead_code)]
pub fn chan_algo(data: &[Point], sub_hull_count: usize) -> Vec<Point> {
    println!("per hull: {}", data.len() / sub_hull_count);
    let mut all_sub_hull: Vec<Point> = vec![];

    let mut pt_counter = 0;

    for i in 0..sub_hull_count {
        let (start, end) = chan_sub_hull_range(data.len(), i, sub_hull_count);
        pt_counter += data[start..end].len();
        let sub_hull = andrew_algo(&data[start..end]);
        all_sub_hull.extend(sub_hull);
    }
    if data.len() != pt_counter {
        eprintln!("{} != {}", data.len(), pt_counter);
        panic!("point count does not match");
    }
    let result = jarvis_march(all_sub_hull.as_slice());
    result
}

#[allow(dead_code)]
pub fn chan_algo_threaded(data: Vec<Point>, sub_hull_count: usize) -> Vec<Point> {
    println!("per hull: {}", data.len() / sub_hull_count);
    let all_sub_hull: Arc<Mutex<Vec<Point>>> = Arc::new(Mutex::new(vec![]));
    let input_data = Arc::new(data);

    let mut thread_handles = vec![];
    for index in 0..sub_hull_count {
        let input_data = Arc::clone(&input_data);
        let all_sub_hull = Arc::clone(&all_sub_hull);
        let handle = thread::spawn(move || {
            let (start, end) = chan_sub_hull_range(input_data.len(), index, sub_hull_count);
            let sub_hull = andrew_algo(&input_data[start..end]);
            {
                let mut val = all_sub_hull.lock().unwrap();
                val.extend(sub_hull);
            }
        });
        thread_handles.push(handle);
    }
    for handle in thread_handles {
        handle.join().unwrap();
    }

    let result = jarvis_march(all_sub_hull.lock().unwrap().as_slice());
    result
}

#[allow(dead_code)]
pub fn andrew_algo_sort(data: &mut Vec<Point>) -> Vec<Point> {
    sort_points_by_x(data);
    andrew_algo(data)
}

// Note: this assumes data is pre-sort by x-coordinate.
pub fn andrew_algo(data: &[Point]) -> Vec<Point> {
    let mut result: Vec<Point> = vec![data.get(0).unwrap().clone(), data.get(1).unwrap().clone()];
    // upper hull
    for pt in (data[2..]).iter() {
        while result.len() >= 2 {
            let last = result.last().unwrap();
            let second_last = result.get(result.len() - 2).unwrap();
            if orientation(second_last, last, pt) > 0 {
                result.pop();
            } else {
                break;
            }
        }
        if pt != result.last().unwrap() {
            result.push(pt.clone());
        }
    }
    let upper_hull_len = result.len();
    // println!("upper hull len: {}", upper_hull_len);

    // lower hull
    for pt in data.iter().rev() {
        while result.len() >= upper_hull_len {
            let last = result.last().unwrap();
            let second_last = result.get(result.len() - 2).unwrap();
            if orientation(second_last, last, pt) > 0 {
                result.pop();
            } else {
                break;
            }
        }
        if pt != result.get(0).unwrap() && pt != result.last().unwrap() {
            result.push(pt.clone());
        }
    }
    result
}

pub fn jarvis_march(data: &[Point]) -> Vec<Point> {
    let pt_1st = match data.get(0) {
        None => {
            eprintln!("jarvis_march, empty input");
            return vec![];
        }
        Some(val) => val,
    };

    let mut result: Vec<Point> = vec![*pt_1st];
    for _ in 0..data.len() {
        let selected = match select_pt(data, result.last().unwrap()) {
            None => {
                // no points selected
                return result;
            }
            Some(val) => val,
        };
        if &selected == pt_1st {
            break;
        }
        result.push(selected);
    }

    result
}

fn select_pt(data: &[Point], last_hull_pt: &Point) -> Option<Point> {
    let pt1 = last_hull_pt;
    for pt2 in data {
        if pt2 == last_hull_pt {
            continue;
        }

        // eprintln!("pt1: {}", pt1);
        // eprintln!("pt2: {}", pt2);

        let mut ok = true;
        // selected pt2 must have all other points on the right of pt1->pt2
        for pt3 in data {
            if pt3 == last_hull_pt {
                continue;
            }
            if pt3 == pt2 {
                continue;
            }
            let ori = orientation(pt1, pt2, pt3);
            // eprintln!("pt3 {}, ori {}", pt3, ori);
            if ori <= 0 {
                ok = false;
                break;
            }
        }
        if ok {
            return Some((*pt2).clone());
        }
    }
    None
}

// pt1->pt2 X pt1->pt3
fn orientation(pt1: &Point, pt2: &Point, pt3: &Point) -> i32 {
    // ax * by - ay * bx
    let z = (pt2.x - pt1.x) * (pt3.y - pt1.y) - (pt2.y - pt1.y) * (pt3.x - pt1.x);
    let result = if z == 0.0 {
        0
    } else if z > 0.0 {
        1
    } else {
        -1
    };
    result
}
